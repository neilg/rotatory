use crate::backoff::Backoff;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Iterator<I>(I);

impl<I> Iterator<I>
where
    I: std::iter::Iterator<Item = Duration>,
{
    pub(crate) fn new(i: I) -> Self {
        Self(i)
    }
}

impl<I> Backoff for Iterator<I>
where
    I: std::iter::Iterator<Item = Duration>,
{
    fn next_delay(&mut self) -> Option<Duration> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let mut test_subject = Iterator([1, 2, 3, 4, 5, 4, 3].map(Duration::from_secs).into_iter());

        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(1)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(2)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(3)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(4)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(5)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(4)));
        assert_eq!(test_subject.next_delay(), Some(Duration::from_secs(3)));
        assert_eq!(test_subject.next_delay(), None);
    }
}
