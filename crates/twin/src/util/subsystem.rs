use crate::util::postoffice::PostOffice;
use uom::si::f64::*;

pub struct SystemEnvironment {
    pub ambient_temperature: ThermodynamicTemperature,
    pub ambient_pressure: Pressure,
    pub ambient_humidity: Ratio,
}

pub trait Subsystem {
    fn step(&mut self, po: &PostOffice, env: &SystemEnvironment, dt: Time);
    fn report_state(&self) -> serde_json::Value;
}
