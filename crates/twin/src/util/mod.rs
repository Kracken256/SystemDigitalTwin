mod postoffice;
mod subsystem;

pub mod prelude {
    pub use crate::util::postoffice::{PostOffice, SignalId};
    pub use crate::util::subsystem::{Subsystem, SystemEnvironment};
}
