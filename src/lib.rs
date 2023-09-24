use std::thread::sleep;
use std::time::Duration;

pub trait Backoff {
    fn next_delay(&mut self) -> Duration;
}

pub enum OpError {}

pub struct Error {}

pub fn retry<T>(
    backoff: impl Backoff,
    body: impl FnMut() -> Result<T, OpError>,
) -> Result<T, Error> {
    let mut retry = Retry { backoff, body };
    retry.run()
}

struct Retry<B, F> {
    backoff: B,
    body: F,
}

impl<T, B, F> Retry<B, F>
where
    B: Backoff,
    F: FnMut() -> Result<T, OpError>,
{
    fn run(&mut self) -> Result<T, Error> {
        loop {
            let result = (self.body)();
            match result {
                Ok(t) => {
                    return Ok(t);
                }
                Err(_e) => {
                    let delay = self.backoff.next_delay();
                    sleep(delay);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysFailBackoff;

    impl Backoff for AlwaysFailBackoff {
        fn next_delay(&mut self) -> Duration {
            panic!("always fails")
        }
    }

    #[test]
    fn should_return_result() {
        let result = retry(AlwaysFailBackoff, || Ok(10));

        assert!(matches!(result, Ok(x) if x == 10));
    }
}
