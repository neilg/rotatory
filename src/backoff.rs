use std::{cmp::min, time::Duration};

pub trait Backoff {
    fn next_delay(&mut self) -> Option<Duration>;
}

pub struct Exponential {
    next: Duration,
    factor: u32,
}

impl Exponential {
    pub fn new(first: Duration, factor: u32) -> Self {
        Self {
            next: first,
            factor,
        }
    }
}

impl Backoff for Exponential {
    fn next_delay(&mut self) -> Option<Duration> {
        let backoff = self.next;
        self.next = backoff * self.factor;
        Some(backoff)
    }
}

pub struct Linear {
    next: Duration,
    increment: Duration,
}

impl Linear {
    pub fn new(first: Duration, increment: Duration) -> Self {
        Self {
            next: first,
            increment,
        }
    }
}

impl Backoff for Linear {
    fn next_delay(&mut self) -> Option<Duration> {
        let backoff = self.next;
        self.next = backoff + self.increment;
        Some(backoff)
    }
}

pub struct MaxAttempts<B> {
    attempts_remaining: u32,
    backoff: B,
}

impl<B> Backoff for MaxAttempts<B>
where
    B: Backoff,
{
    fn next_delay(&mut self) -> Option<Duration> {
        if self.attempts_remaining == 0 {
            None
        } else {
            let backoff = self.backoff.next_delay();
            self.attempts_remaining -= 1;
            backoff
        }
    }
}

pub struct MaxDelay<B> {
    max_delay: Duration,
    backoff: B,
}

impl<B> Backoff for MaxDelay<B>
where
    B: Backoff,
{
    fn next_delay(&mut self) -> Option<Duration> {
        self.backoff.next_delay().map(|b| min(b, self.max_delay))
    }
}

trait Builder: Sized {
    fn with_max_delay(self, max_delay: Duration) -> MaxDelay<Self> {
        MaxDelay {
            max_delay,
            backoff: self,
        }
    }

    fn with_max_attempts(self, max_attempts: u32) -> MaxAttempts<Self> {
        MaxAttempts {
            attempts_remaining: max_attempts,
            backoff: self,
        }
    }
}

impl<B> Builder for B where B: Backoff {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential() {
        let mut test_subject = Exponential::new(Duration::from_millis(123), 2);

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(123)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(246)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(492)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(984)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(1968)));
    }

    #[test]
    fn linear() {
        let mut test_subject = Linear::new(Duration::from_millis(10), Duration::from_millis(11));

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(10)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(21)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(32)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(43)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
    }

    #[test]
    fn max_attempts() {
        let mut test_subject =
            Linear::new(Duration::from_millis(10), Duration::from_millis(11)).with_max_attempts(2);

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(10)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(21)));
        assert_eq!(test_subject.next_delay(), None);
    }

    #[test]
    fn max_delay() {
        let mut test_subject = Exponential::new(Duration::from_millis(10), 2)
            .with_max_delay(Duration::from_millis(100));

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(10)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(20)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(40)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(80)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
    }

    #[test]
    fn max_delay_and_max_attempts() {
        let mut test_subject = Exponential::new(Duration::from_millis(10), 2)
            .with_max_delay(Duration::from_millis(100))
            .with_max_attempts(6);

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(10)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(20)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(40)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(80)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), None);
    }
}
