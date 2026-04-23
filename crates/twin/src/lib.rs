mod subsystems;
mod util;

pub use subsystems::*;
use uom::si::f64::Time;
pub use util::prelude::*;

pub struct System {
    post_office: PostOffice,
}

impl System {
    pub fn new() -> Self {
        Self {
            post_office: PostOffice::new(),
        }
    }

    pub fn step(&mut self, _dt: Time) {
        // TODO: Update subsystems
        self.post_office.deliver_mail();
    }

    pub fn report_state(&self) -> serde_json::Value {
        let report = serde_json::Map::new();
        serde_json::Value::Object(report)
    }
}
