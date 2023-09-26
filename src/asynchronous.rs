use crate::{backoff::Backoff, Error};
use std::{future::Future, time::Duration};

pub async fn retry<R, S, T, E>(
    mut backoff: impl Backoff,
    sleep: impl Fn(Duration) -> S,
    mut body: impl FnMut() -> R,
) -> Result<T, Error<E>>
where
    R: Future<Output = Result<T, E>>,
    S: Future,
{
    let mut tries = 0;
    loop {
        match body().await {
            Ok(t) => return Ok(t),
            Err(source) => {
                tries += 1;
                if let Some(delay) = backoff.next_delay() {
                    sleep(delay).await;
                } else {
                    return Err(Error::new(tries, source));
                }
            }
        }
    }
}
