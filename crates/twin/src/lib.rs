mod subsystems;
mod util;

use crate::subsystems::prelude::*;
pub use subsystems::*;
use uom::si::{electric_current::ElectricCurrent, f64::Time};
pub use util::prelude::*;

pub struct System {
    po: PostOffice,
    env: SystemEnvironment,

    battery: BatterySubsystem,
}

impl System {
    pub fn new(env: SystemEnvironment) -> Self {
        let mut po = PostOffice::new();

        let load = SignalId("battery_current_demand");
        po.register_current(load);
        po.write_current(
            load,
            ElectricCurrent::new::<uom::si::electric_current::ampere>(1.0),
        );

        let battery = BatterySubsystem::new(&mut po);

        Self { po, env, battery }
    }

    pub fn step(&mut self, dt: Time) {
        self.battery.step(&mut self.po, &self.env, dt);

        self.po.deliver_mail();
    }

    pub fn report_state(&self) -> serde_json::Value {
        let mut report = serde_json::Map::new();
        report.insert("battery".to_string(), self.battery.report_state());
        serde_json::Value::Object(report)
    }
}
