mod subsystems;
mod util;

use crate::subsystems::prelude::*;
pub use subsystems::*;
use uom::si::{
    electric_potential::volt,
    electrical_resistance::ohm,
    f64::{ElectricPotential, ElectricalResistance, Time},
};
pub use util::prelude::*;

pub struct System {
    po: PostOffice,
    env: SystemEnvironment,

    battery: BatterySubsystem,
    resistor: ResistorSubsystem,
}

impl System {
    pub fn new(env: SystemEnvironment) -> Self {
        let mut po = PostOffice::new();

        let battery = BatterySubsystem::new(&mut po);
        let resistor = ResistorSubsystem::new(
            &mut po,
            ResistorConfig {
                resistance: ElectricalResistance::new::<ohm>(150.0),
                forward_voltage: ElectricPotential::new::<volt>(2.0),
                voltage_input: SignalId::from("battery_voltage"),
                voltage_output: SignalId::from("led_terminal_voltage"),
            },
        );

        Self {
            po,
            env,
            battery,
            resistor,
        }
    }

    pub fn step(&mut self, dt: Time) {
        self.battery.step(&mut self.po, &self.env, dt);
        self.resistor.step(&mut self.po, &self.env, dt);

        self.po.flip();
    }

    pub fn report_state(&self) -> serde_json::Value {
        let mut report = serde_json::Map::new();

        report.insert("battery".to_string(), self.battery.report_state());
        report.insert("resistor".to_string(), self.resistor.report_state());

        serde_json::Value::Object(report)
    }
}
