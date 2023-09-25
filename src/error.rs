use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Error<E> {
    pub tries: u32,
    pub cause: E,
}

impl<E> Error<E> {
    pub fn tries(&self) -> u32 {
        self.tries
    }
    pub fn cause(&self) -> &E {
        &self.cause
    }
    pub fn into_cause(self) -> E {
        self.cause
    }
}

impl<E> Display for Error<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tried {} times. Most recent error: {}",
            self.tries, self.cause
        )
    }
}

impl<E> std::error::Error for Error<E> where E: std::error::Error {}
