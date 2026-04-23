mod battery;
mod resistor;

pub mod prelude {
    pub use super::battery::BatterySubsystem;
    pub use super::resistor::{ResistorConfig, ResistorSubsystem};
}
