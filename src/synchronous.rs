use crate::backoff::Backoff;
use std::thread::sleep;

pub fn retry<T, E>(
    mut backoff: impl Backoff,
    mut body: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    loop {
        match body() {
            Ok(t) => {
                return Ok(t);
            }
            Err(e) => {
                if let Some(delay) = backoff.next_delay() {
                    sleep(delay)
                } else {
                    return Err(e);
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

    #[test]
    fn should_return_result_on_ok() {
        let fallible = || Ok::<_, String>(10);

        let result = retry(NeverRetry, fallible);

        assert_eq!(result, Ok(10));
    }

    #[test]
    fn should_return_error_on_failure() {
        let fallible = || Err::<i32, _>("failure".to_string());

        let result = retry(NeverRetry, fallible);

        assert_eq!(result, Err("failure".to_string()));
    }
}
