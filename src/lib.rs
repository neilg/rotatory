pub use backoff::{Backoff, Exponential, Linear};
pub use error::Error;
pub use synchronous::retry;

mod backoff;
mod error;
mod synchronous;
