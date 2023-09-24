use crate::{Backoff, Error};
use cfg_if::cfg_if;
use std::future::Future;

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
                    cfg_if! {
                        if #[cfg(feature = "async_tokio")] {
                            tokio::time::sleep(delay).await;
                        } else if #[cfg(feature = "async_std")] {
                            async_std::task::sleep(delay).await;
                        } else {
                            // do something with the delay to prevent the compiler warning if no async runtime is specified
                            drop(delay);
                            unreachable!("there was an error in 'rotatory'");
                        }
                    }
                } else {
                    return Err(Error { tries, cause });
                }
            }
        }
    }
}
