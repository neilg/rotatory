use crate::backoff::Backoff;
use std::{cmp::min, time::Duration};

#[derive(Debug)]
pub struct MaxDelay<B> {
    max_delay: Duration,
    backoff: B,
}

impl<B> MaxDelay<B> {
    pub(crate) fn new(max_delay: Duration, backoff: B) -> Self {
        Self { max_delay, backoff }
    }
}

impl<B> Backoff for MaxDelay<B>
where
    B: Backoff,
{
    fn next_delay(&mut self) -> Option<Duration> {
        self.backoff.next_delay().map(|b| min(b, self.max_delay))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backoff::Iterator;

    #[test]
    fn max_delay() {
        let mut test_subject = MaxDelay::new(
            Duration::from_millis(100),
            Iterator::new(
                [95, 96, 97, 98, 99, 100, 101, 102, 103]
                    .map(Duration::from_millis)
                    .into_iter(),
            ),
        );

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(95)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(96)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(97)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(98)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(99)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(100)));
    }
}
