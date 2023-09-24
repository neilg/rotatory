pub use backoff::{Backoff, Exponential, Linear};
pub use error::Error;

#[cfg(feature = "async")]
pub mod asynchronous;
mod backoff;
mod error;
#[cfg(feature = "sync")]
pub mod synchronous;
