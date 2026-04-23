use crate::{PostOffice, SignalId, Subsystem, SystemEnvironment};
use uom::si::{energy::joule, f64::*, thermodynamic_temperature::kelvin};

pub struct BatterySubsystem {
    charge_level: Energy,
    voltage_output: SignalId,
    current_output: SignalId,
    surface_temperature: ThermodynamicTemperature,
}

impl BatterySubsystem {
    pub fn new(po: &mut crate::PostOffice) -> Self {
        let voltage_id = SignalId("battery_voltage");
        let current_id = SignalId("battery_current");
        let temperature_id = SignalId("battery_temperature");

        po.register_voltage(voltage_id);
        po.register_current(current_id);
        po.register_temperature(temperature_id);

        Self {
            charge_level: Energy::new::<joule>(1.0),
            surface_temperature: ThermodynamicTemperature::new::<kelvin>(300.0),

            voltage_output: voltage_id,
            current_output: current_id,
        }
    }
}

impl Subsystem for BatterySubsystem {
    fn step(&mut self, po: &PostOffice, env: &SystemEnvironment, dt: Time) {
        let discharge_rate = Power::new::<uom::si::power::watt>(0.1);
        self.charge_level -= discharge_rate * dt;

        po.write_voltage(
            self.voltage_output,
            ElectricPotential::new::<uom::si::electric_potential::volt>(12.0),
        );
        po.write_current(
            self.current_output,
            ElectricCurrent::new::<uom::si::electric_current::ampere>(1.0),
        );
    }

    fn report_state(&self) -> serde_json::Value {
        serde_json::json!({
            "charge_level": self.charge_level,
        })
    }
}
