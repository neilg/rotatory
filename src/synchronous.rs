use crate::{backoff::Backoff, Error};
use std::thread::sleep;

pub fn retry<T, E>(
    mut backoff: impl Backoff,
    mut body: impl FnMut() -> Result<T, E>,
) -> Result<T, Error<E>> {
    let mut tries = 0;
    loop {
        match body() {
            Ok(t) => {
                return Ok(t);
            }
            Err(source) => {
                tries += 1;
                if let Some(delay) = backoff.next_delay() {
                    sleep(delay)
                } else {
                    return Err(Error::new(tries, source));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    struct NeverRetry;

    impl Backoff for NeverRetry {
        fn next_delay(&mut self) -> Option<Duration> {
            None
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct BadError;

    fn succeed() -> Result<i32, BadError> {
        Ok(10)
    }

    #[test]
    fn should_return_result() {
        let result = retry(NeverRetry, succeed);

        assert_eq!(result, Ok(10));
    }
}
