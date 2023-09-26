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
