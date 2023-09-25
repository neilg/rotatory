use crate::backoff::Backoff;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Linear {
    next: Duration,
    increment: Duration,
}

impl Linear {
    pub(crate) fn new(initial: Duration, increment: Duration) -> Self {
        Self {
            next: initial,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear() {
        let mut test_subject = Linear::new(Duration::from_millis(10), Duration::from_millis(11));

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(10)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(21)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(32)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(43)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
    }
}
