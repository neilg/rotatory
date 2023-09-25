#[cfg(feature = "async_tokio")]
mod tests {
    use rotatory::{asynchronous, backoff, backoff::Backoff, Error};
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::time::Instant;

    #[derive(Clone)]
    struct FallibleService {
        call_count: Arc<Mutex<i32>>,
    }

    impl FallibleService {
        fn new() -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        async fn do_something(&mut self) -> Result<i32, String> {
            let mut call_count = self.call_count.lock().unwrap();
            *call_count += 1;
            Err("Kaboom".to_string())
        }
    }

    #[tokio::test]
    async fn example() {
        let service = FallibleService::new();

        let initial = Duration::from_millis(20);
        let backoff = backoff::exponential(initial, 2)
            .with_max_delay(Duration::from_millis(200))
            .with_max_attempts(10);

        let start_time = Instant::now();
        let result = asynchronous::retry(backoff, || {
            let mut service = service.clone();
            async move { service.do_something().await }
        })
        .await;
        let time_taken = start_time.elapsed();
        assert!(
            time_taken >= Duration::from_millis(1300) && time_taken < Duration::from_millis(1400),
            "Time taken was {} ms. Expected it to be between 1,300 ms and 1,400 ms.",
            time_taken.as_millis()
        );

        let Error { tries, cause } = result.unwrap_err();

        assert_eq!(cause, "Kaboom");
        assert_eq!(tries, 10);
    }
}