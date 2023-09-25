pub use error::Error;

#[cfg(feature = "async")]
pub mod asynchronous;
pub mod backoff;
mod error;
#[cfg(feature = "sync")]
pub mod synchronous;
