use constant::Constant;
use exponential::Exponential;
use iterator::Iterator;
use linear::Linear;
use max_attempts::MaxAttempts;
use max_delay::MaxDelay;
use std::time::Duration;

mod constant;
mod exponential;
mod iterator;
mod linear;
mod max_attempts;
mod max_delay;

pub fn constant(duration: Duration) -> impl Backoff {
    Constant::new(duration)
}

pub fn exponential(initial: Duration, multiplier: u32) -> impl Backoff {
    Exponential::new(initial, multiplier)
}

pub fn iterator(iter: impl std::iter::Iterator<Item = Duration>) -> impl Backoff {
    Iterator::new(iter)
}

pub fn linear(initial: Duration, increment: Duration) -> impl Backoff {
    Linear::new(initial, increment)
}

pub trait Backoff {
    fn next_delay(&mut self) -> Option<Duration>;
    fn with_max_attempts(self, max_attempts: u32) -> MaxAttempts<Self>
    where
        Self: Sized,
    {
        MaxAttempts::new(max_attempts, self)
    }

    fn with_max_delay(self, max_delay: Duration) -> MaxDelay<Self>
    where
        Self: Sized,
    {
        MaxDelay::new(max_delay, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_delay_and_max_attempts() {
        let mut test_subject = Iterator::new(
            [98, 99, 100, 101, 102, 103, 104, 105]
                .map(Duration::from_millis)
                .into_iter(),
        )
        .with_max_delay(Duration::from_millis(100))
        .with_max_attempts(6);

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(98)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(99)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), None);
    }
}
