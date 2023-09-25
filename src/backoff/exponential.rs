use crate::backoff::Backoff;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Exponential {
    next: Duration,
    multiplier: u32,
}

impl Exponential {
    pub(crate) fn new(initial: Duration, multiplier: u32) -> Self {
        Self {
            next: initial,
            multiplier,
        }
    }
}

impl Backoff for Exponential {
    fn next_delay(&mut self) -> Option<Duration> {
        let backoff = self.next;
        self.next = backoff * self.multiplier;
        Some(backoff)
    }
}

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
}
