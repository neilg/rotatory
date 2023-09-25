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
            Err(cause) => {
                tries += 1;
                if let Some(delay) = backoff.next_delay() {
                    sleep(delay)
                } else {
                    return Err(Error { tries, cause });
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
