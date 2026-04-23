mod postoffice;
mod subsystem;

pub mod prelude {
    pub use crate::util::postoffice::{PostOffice, SignalBuffer, SignalId};
    pub use crate::util::subsystem::{Subsystem, SystemEnvironment};
}
