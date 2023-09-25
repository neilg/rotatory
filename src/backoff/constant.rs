use crate::backoff::Backoff;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Constant(Duration);

impl Constant {
    pub(crate) fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl Backoff for Constant {
    fn next_delay(&mut self) -> Option<Duration> {
        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant() {
        let mut test_subject = Constant::new(Duration::from_millis(54));

        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_millis(54)));
    }
}
