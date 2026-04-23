mod subsystems;
mod util;

use crate::subsystems::prelude::*;
pub use subsystems::*;
use uom::si::f64::Time;
pub use util::prelude::*;

pub struct System {
    post_office: PostOffice,
    env: SystemEnvironment,

    battery: BatterySubsystem,
}

impl System {
    pub fn new(env: SystemEnvironment) -> Self {
        let mut post_office = PostOffice::new();
        let battery = BatterySubsystem::new(&mut post_office);

        Self {
            post_office,
            env,
            battery,
        }
    }

    pub fn step(&mut self, dt: Time) {
        self.battery.step(&mut self.post_office, &self.env, dt);

        self.post_office.deliver_mail();
    }

    pub fn report_state(&self) -> serde_json::Value {
        let mut report = serde_json::Map::new();
        report.insert("battery".to_string(), self.battery.report_state());
        serde_json::Value::Object(report)
    }
}
