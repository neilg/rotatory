use crate::{Backoff, Error};
use std::future::Future;

pub async fn retry<F, Fu, T, E>(mut backoff: impl Backoff, mut body: F) -> Result<T, Error<E>>
where
    F: FnMut() -> Fu,
    Fu: Future<Output = Result<T, E>>,
{
    let mut tries = 0;
    loop {
        let result = body().await;
        match result {
            Ok(t) => {
                return Ok(t);
            }
            Err(cause) => {
                tries += 1;
                let delay = backoff.next_delay();
                if let Some(delay) = delay {
                    // TODO should we prevent both being called if both features are enabled?
                    // TODO and how would we stop it?
                    #[cfg(feature = "async_tokio")]
                    tokio::time::sleep(delay).await;
                    #[cfg(feature = "async_std")]
                    async_std::task::sleep(delay).await;
                } else {
                    return Err(Error { tries, cause });
                }
            }
        }
    }
}
