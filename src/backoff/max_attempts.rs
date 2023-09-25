use crate::backoff::Backoff;
use std::time::Duration;

#[derive(Debug)]
pub struct MaxAttempts<B> {
    attempts_remaining: u32,
    backoff: B,
}

impl<B> MaxAttempts<B> {
    pub(crate) fn new(max_attempts: u32, backoff: B) -> Self {
        Self {
            attempts_remaining: max_attempts,
            backoff,
        }
    }
}

impl<B> Backoff for MaxAttempts<B>
where
    B: Backoff,
{
    fn next_delay(&mut self) -> Option<Duration> {
        if self.attempts_remaining <= 1 {
            None
        } else {
            self.attempts_remaining -= 1;
            self.backoff.next_delay()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backoff::Iterator;

    #[test]
    fn max_attempts() {
        let mut test_subject = MaxAttempts::new(
            3,
            Iterator::new(
                vec![
                    Duration::from_millis(10),
                    Duration::from_millis(21),
                    Duration::from_millis(32),
                    Duration::from_millis(57),
                ]
                .into_iter(),
            ),
        );

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(10)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(21)));
        assert_eq!(test_subject.next_delay(), None);
    }
}
