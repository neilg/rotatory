#[cfg(feature = "sync")]
mod tests {
    use rotatory::{backoff, backoff::Backoff};
    use std::time::Instant;
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    struct FallibleService {
        call_count: Arc<Mutex<u32>>,
    }

    impl FallibleService {
        fn new() -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        fn do_something(&mut self) -> Result<i32, String> {
            let mut call_count = self.call_count.lock().unwrap();
            *call_count += 1;
            if *call_count == 3 {
                Ok(10)
            } else {
                Err("Kaboom".to_string())
            }
        }
    }

    #[test]
    fn example() {
        let mut service = FallibleService::new();

        let backoff = backoff::linear(Duration::from_millis(10), Duration::from_millis(2))
            .with_max_attempts(3);

        let start_time = Instant::now();
        let result = rotatory::synchronous::retry(backoff, || service.do_something());
        let time_taken = start_time.elapsed();

        assert!(
            time_taken >= Duration::from_millis(22) && time_taken < Duration::from_millis(30),
            "Time taken was {} ms. Expected it to be between 22 ms and 30 ms.",
            time_taken.as_millis()
        );

        assert_eq!(result, Ok(10));
    }
}
