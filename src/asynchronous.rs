use crate::{Backoff, Error};
use std::{future::Future, time::Duration};

#[cfg(all(not(feature = "async_std"), not(feature = "async_tokio")))]
compile_error! {
    r#"feature "async_std" or feature "async_tokio" must be enabled"#
}

pub async fn retry<F, Fu, T, E>(mut backoff: impl Backoff, mut body: F) -> Result<T, Error<E>>
where
    F: FnMut() -> Fu,
    Fu: Future<Output = Result<T, E>>,
{
    let mut tries = 0;
    loop {
        match body().await {
            Ok(t) => return Ok(t),
            Err(cause) => {
                tries += 1;
                if let Some(delay) = backoff.next_delay() {
                    sleep(delay).await
                } else {
                    return Err(Error { tries, cause });
                }
            }
        }
    }
}

#[inline]
async fn sleep(duration: Duration) {
    if cfg!(feature = "async_tokio") {
        #[cfg(feature = "async_tokio")]
        tokio::time::sleep(duration).await;
    } else if cfg!(feature = "async_std") {
        #[cfg(feature = "async_std")]
        async_std::task::sleep(duration).await;
    } else {
        unreachable!()
    }
}
