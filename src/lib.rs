pub use backoff::{Backoff, Exponential, Linear};
use std::{
    fmt::{Display, Formatter},
    thread::sleep,
};

mod backoff;

#[derive(Debug)]
pub struct Error<E> {
    tries: u32,
    cause: E,
}

impl<E> Display for Error<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried {} times. Final error: {}", self.tries, self.cause)
    }
}

impl<E> std::error::Error for Error<E> where E: std::error::Error {}

impl<E> Error<E> {
    pub fn cause(&self) -> &E {
        &self.cause
    }
}

pub fn retry<T, E>(
    backoff: impl Backoff,
    body: impl FnMut() -> Result<T, E>,
) -> Result<T, Error<E>> {
    let mut retry = Retry { backoff, body };
    retry.run()
}

struct Retry<B, F> {
    backoff: B,
    body: F,
}

impl<T, E, B, F> Retry<B, F>
where
    B: Backoff,
    F: FnMut() -> Result<T, E>,
{
    fn run(&mut self) -> Result<T, Error<E>> {
        let mut tries = 0;
        loop {
            let result = (self.body)();
            match result {
                Ok(t) => {
                    return Ok(t);
                }
                Err(cause) => {
                    tries += 1;
                    let delay = self.backoff.next_delay();
                    if let Some(delay) = delay {
                        sleep(delay);
                    } else {
                        return Err(Error { tries, cause });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    struct AlwaysFailBackoff;

    impl Backoff for AlwaysFailBackoff {
        fn next_delay(&mut self) -> Option<Duration> {
            panic!("always fails")
        }
    }

    struct BadError;

    fn succeed() -> Result<i32, BadError> {
        Ok(10)
    }

    #[test]
    fn should_return_result() {
        let result = retry(AlwaysFailBackoff, succeed);

        assert!(matches!(result, Ok(x) if x == 10));
    }
}
