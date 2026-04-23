mod postoffice;
mod system;

pub mod prelude {
    pub use crate::util::postoffice::{PostOffice, SignalBuffer, SignalId};
    pub use crate::util::system::Subsystem;
}
