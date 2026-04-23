use crate::{PostOffice, SignalId, Subsystem, SystemEnvironment};
use uom::si::f64::*;
use uom::si::{
    ISQ, Quantity, SI, electric_current::ampere, electric_potential::volt,
    electrical_resistance::ohm, energy::joule, power::watt, temperature_interval,
    thermodynamic_temperature, thermodynamic_temperature::degree_celsius,
};
use uom::typenum::{N1, N3, P1, P2, Z0};

pub type PowerPerTemperature = Quantity<ISQ<P2, P1, N3, Z0, N1, Z0, Z0>, SI<f64>, f64>;

pub struct BatterySpec {
    pub max_energy: Energy,
    pub internal_resistance: ElectricalResistance,
    pub thermal_mass: HeatCapacity,
    pub cooling_coefficient: PowerPerTemperature,
    pub min_voltage: ElectricPotential,
    pub max_voltage: ElectricPotential,
}

pub struct BatterySubsystem {
    spec: BatterySpec,
    charge_level: Energy,
    surface_temperature: ThermodynamicTemperature,

    current_load_input: SignalId,
    voltage_output: SignalId,
    temperature_output: SignalId,
}

impl BatterySubsystem {
    pub fn new(po: &mut PostOffice) -> Self {
        let spec = BatterySpec {
            max_energy: Energy::new::<joule>(3600.0 * 100.0), // 100 Wh
            internal_resistance: ElectricalResistance::new::<ohm>(0.05),
            thermal_mass: HeatCapacity::new::<uom::si::heat_capacity::joule_per_kelvin>(500.0),
            cooling_coefficient: Power::new::<watt>(0.5)
                / TemperatureInterval::new::<temperature_interval::kelvin>(1.0),
            min_voltage: ElectricPotential::new::<volt>(3.0),
            max_voltage: ElectricPotential::new::<volt>(4.2),
        };

        let voltage_id = SignalId::from("battery_voltage");
        let current_id = SignalId::from("battery_current_demand");
        let temperature_id = SignalId::from("battery_temperature");

        po.register::<ElectricPotential>(voltage_id);
        po.register::<ElectricCurrent>(current_id);
        po.register::<ThermodynamicTemperature>(temperature_id);

        Self {
            charge_level: spec.max_energy,
            surface_temperature: ThermodynamicTemperature::new::<degree_celsius>(20.0),
            spec,
            voltage_output: voltage_id,
            current_load_input: current_id,
            temperature_output: temperature_id,
        }
    }

    fn calculate_voltage(&self) -> ElectricPotential {
        let soc = (self.charge_level / self.spec.max_energy)
            .get::<uom::si::ratio::ratio>()
            .clamp(0.0, 1.0);

        // Linear interpolation between min and max voltage based on SoC
        self.spec.min_voltage + (self.spec.max_voltage - self.spec.min_voltage) * soc
    }
}

impl Subsystem for BatterySubsystem {
    fn step(&mut self, po: &mut PostOffice, env: &SystemEnvironment, dt: Time) {
        // 1. Get the current demand from the system
        let current_draw = *po
            .read::<ElectricCurrent>(self.current_load_input)
            .unwrap_or(&ElectricCurrent::new::<ampere>(0.0));

        // 2. Calculate Voltage and Power
        let open_circuit_voltage = self.calculate_voltage();
        // Voltage Sag: V_actual = V_oc - (I * R_internal)
        let voltage_drop = current_draw * self.spec.internal_resistance;
        let terminal_voltage =
            (open_circuit_voltage - voltage_drop).max(ElectricPotential::new::<volt>(0.0));

        let power_draw = terminal_voltage * current_draw;

        // 3. Update Energy (uom handles the dimensional math: Energy = Power * Time)
        let energy_consumed = power_draw * dt;
        self.charge_level = (self.charge_level - energy_consumed).max(Energy::new::<joule>(0.0));

        // 4. Thermal Modeling
        // Joule Heating: P = I^2 * R
        let heat_generated = current_draw * current_draw * self.spec.internal_resistance;

        let delta_t = TemperatureInterval::new::<temperature_interval::kelvin>(
            self.surface_temperature
                .get::<thermodynamic_temperature::kelvin>()
                - env
                    .ambient_temperature
                    .get::<thermodynamic_temperature::kelvin>(),
        );
        let cooling_power = delta_t * self.spec.cooling_coefficient;

        let net_heat_flux = heat_generated - cooling_power;

        // Temperature Change: dT = (dQ / ThermalMass)
        let temp_change = (net_heat_flux * dt) / self.spec.thermal_mass;
        self.surface_temperature += temp_change;

        // 5. Update PostOffice
        po.write::<ElectricPotential>(self.voltage_output, terminal_voltage);
        po.write::<ThermodynamicTemperature>(self.temperature_output, self.surface_temperature);
        po.clear_accumulator::<ElectricCurrent>(self.current_load_input);
    }

    fn report_state(&self) -> serde_json::Value {
        serde_json::json!({
            "charge_j": self.charge_level.get::<joule>(),
            "voltage_v": self.calculate_voltage().get::<volt>(),
            "temp_k": self.surface_temperature.get::<thermodynamic_temperature::kelvin>(),
        })
    }
}
