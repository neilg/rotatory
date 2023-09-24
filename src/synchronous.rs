use crate::{Backoff, Error};
use std::thread::sleep;

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
