#[cfg(feature = "async_tokio")]
mod tests {
    use rotatory::{asynchronous, backoff, backoff::Backoff};
    use std::{
        error::Error as _,
        fmt::{Display, Formatter},
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::time::Instant;

    #[derive(Clone)]
    struct FallibleService {
        call_count: Arc<Mutex<i32>>,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct ServiceError {
        message: String,
    }

    impl ServiceError {
        fn new(message: impl ToString) -> Self {
            Self {
                message: message.to_string(),
            }
        }
    }

    impl Display for ServiceError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.message)
        }
    }

    impl std::error::Error for ServiceError {}

    impl FallibleService {
        fn new() -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        async fn do_something(&mut self) -> Result<i32, ServiceError> {
            let mut call_count = self.call_count.lock().unwrap();
            *call_count += 1;
            Err(ServiceError::new("Kaboom"))
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
        let result = asynchronous::retry(backoff, tokio::time::sleep, || {
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

        let error = result.unwrap_err();
        let error = error
            .source()
            .expect("there wasn't a source error")
            .downcast_ref::<ServiceError>()
            .expect("the source wasn't a ServiceError");

        assert_eq!(
            *error,
            ServiceError {
                message: "Kaboom".to_string()
            }
        );
    }
}
