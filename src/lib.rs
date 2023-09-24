pub use backoff::{Backoff, Exponential, Linear};
pub use error::Error;

pub mod asynchronous;
mod backoff;
mod error;
pub mod synchronous;
