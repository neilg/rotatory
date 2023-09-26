use crate::backoff::Backoff;
use std::{future::Future, time::Duration};

pub async fn retry<R, S, T, E>(
    mut backoff: impl Backoff,
    sleep: impl Fn(Duration) -> S,
    mut body: impl FnMut() -> R,
) -> Result<T, E>
where
    R: Future<Output = Result<T, E>>,
    S: Future,
{
    loop {
        match body().await {
            Ok(t) => return Ok(t),
            Err(e) => {
                if let Some(delay) = backoff.next_delay() {
                    sleep(delay).await;
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

    struct NeverRetry;
    impl Backoff for NeverRetry {
        fn next_delay(&mut self) -> Option<Duration> {
            None
        }
    }

    async fn sleep(_: Duration) {}

    #[tokio::test]
    async fn should_return_result_on_ok() {
        let fallible = || async { Ok::<_, String>(26) };

        let result = retry(NeverRetry, sleep, fallible).await;

        assert_eq!(result, Ok(26));
    }

    #[tokio::test]
    async fn should_return_error_on_failure() {
        let fallible = || async { Err::<i32, _>("oh dear".to_string()) };

        let result = retry(NeverRetry, sleep, fallible).await;

        assert_eq!(result, Err("oh dear".to_string()));
    }
}
