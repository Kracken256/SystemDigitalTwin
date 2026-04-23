use crate::prelude::PowerPerTemperature;
use crate::{PostOffice, SignalId, Subsystem, SystemEnvironment};
use uom::si::{
    electric_current::ampere, electric_potential::volt, electrical_resistance::ohm, power::watt,
    thermodynamic_temperature::kelvin,
};
use uom::si::{f64::*, temperature_interval};

pub struct ResistorConfig {
    pub name: &'static str,
    pub resistance_ref: ElectricalResistance, // R at 293.15K
    pub temp_coefficient: f64,                // alpha (1/K)
    pub thermal_mass: HeatCapacity,           // J/K
    pub cooling_coefficient: PowerPerTemperature, // W/K
    pub voltage_input: SignalId,
    pub current_demand_bus: SignalId,
}

impl ResistorConfig {
    /// Helper for standard 1/4W through-hole resistors
    pub fn standard(
        name: &'static str,
        ohms: ElectricalResistance,
        voltage_input: SignalId,
        current_demand_bus: SignalId,
    ) -> Self {
        Self {
            name,
            resistance_ref: ohms,
            temp_coefficient: 0.00393, // Copper-ish coefficient
            thermal_mass: HeatCapacity::new::<uom::si::heat_capacity::joule_per_kelvin>(0.5),
            cooling_coefficient: Power::new::<watt>(0.02)
                / TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(1.0),
            voltage_input,
            current_demand_bus,
        }
    }
}

pub struct Resistor {
    config: ResistorConfig,
    temperature: ThermodynamicTemperature,
    current_resistance: ElectricalResistance,
    last_i_draw: ElectricCurrent,
}

impl Resistor {
    pub fn new(po: &mut PostOffice, config: ResistorConfig) -> Self {
        // Register telemetry for this specific resistor's health
        po.register::<ThermodynamicTemperature>(SignalId::from(format!("{}_temp", config.name)));

        Self {
            config,
            temperature: ThermodynamicTemperature::new::<kelvin>(293.15), // Start at 20C
            current_resistance: ElectricalResistance::new::<ohm>(0.0), // Calculated in first step
            last_i_draw: ElectricCurrent::new::<ampere>(0.0),
        }
    }
}

impl Subsystem for Resistor {
    fn step(&mut self, po: &mut PostOffice, env: &SystemEnvironment, dt: Time) {
        // 1. Update Resistance based on current Temperature
        let t_ref = ThermodynamicTemperature::new::<kelvin>(293.15);
        let delta_t = TemperatureInterval::new::<temperature_interval::kelvin>(
            self.temperature.get::<kelvin>() - t_ref.get::<kelvin>(),
        );

        self.current_resistance = self.config.resistance_ref
            * (1.0
                + self.config.temp_coefficient
                    * delta_t.get::<uom::si::temperature_interval::kelvin>());

        // 2. Electrical Calculation: I = V / R
        let v_in = po
            .read::<ElectricPotential>(self.config.voltage_input)
            .cloned()
            .unwrap_or_else(|| ElectricPotential::new::<volt>(0.0));

        let current = v_in / self.current_resistance;
        let power_dissipated = v_in * current;

        // 3. Thermodynamic Calculation (Newton's Law of Cooling)
        let t_amb = env.ambient_temperature;
        let t_diff = TemperatureInterval::new::<temperature_interval::kelvin>(
            self.temperature.get::<kelvin>() - t_amb.get::<kelvin>(),
        );

        let cooling_power = t_diff * self.config.cooling_coefficient;
        let net_power = power_dissipated - cooling_power;

        // dT = (P / ThermalMass) * dt
        let delta_temp = (net_power / self.config.thermal_mass) * dt;
        self.temperature += delta_temp;

        // 4. Bus Updates
        self.last_i_draw = current;
        po.accumulate::<ElectricCurrent>(self.config.current_demand_bus, current);
        po.write::<ThermodynamicTemperature>(
            SignalId::from(format!("{}_temp", self.config.name)),
            self.temperature,
        );
    }

    fn report_state(&self) -> serde_json::Value {
        serde_json::json!({
            "resistance_ohms": self.current_resistance.get::<ohm>(),
            "temp_kelvin": self.temperature.get::<kelvin>(),
            "current_draw_a": self.last_i_draw.get::<ampere>(),
        })
    }
}
