use crate::util::postoffice::PostOffice;
use uom::si::f64::Time;

pub trait Subsystem {
    fn step(&mut self, po: &PostOffice, dt: Time);
    fn report_state(&self) -> serde_json::Value;
}
