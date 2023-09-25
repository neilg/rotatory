use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Error<E> {
    tries: u32,
    source: E,
}

impl<E> Error<E> {
    pub fn new(tries: u32, source: E) -> Self {
        Self { tries, source }
    }

    pub fn tries(&self) -> u32 {
        self.tries
    }
    pub fn into_source(self) -> E {
        self.source
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
            self.tries, self.source
        )
    }
}

impl<E> std::error::Error for Error<E>
where
    E: std::error::Error + 'static,
{
    /// ```
    /// use std::fmt::{Display, Formatter};
    /// use std::error::Error as _;
    /// #[derive(Debug)]
    /// struct E;
    /// impl Display for E {fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///         f.write_str("E")
    ///     }
    /// }
    /// impl std::error::Error for E {}
    /// let e = rotatory::Error::new(2, E);
    /// let source: Option<&dyn std::error::Error> = e.source();
    /// assert!(source.is_some());
    /// let source: &E = source.unwrap().downcast_ref::<E>().unwrap();
    /// ```
    ///
    /// ```compile_fail
    /// use std::error::Error as _;
    /// let e = rotatory::Error::new(10, "BANG");
    /// e.source(); // this fails to compile because the source, "BANG", does not implement std::error::Error
    /// ```
    ///
    /// ```compile_fail
    /// use std::fmt::{Display, Formatter};
    /// #[derive(Debug)]
    /// struct E;
    /// impl Display for E {fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///         f.write_str("E")
    ///     }
    /// }
    /// impl std::error::Error for E {}
    /// let e = rotatory::Error::new(2, E);
    /// let x = e.source(); // this fails to compile because std::error::Error is not in scope
    /// ```
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_will_accept_non_error_source_type() {
        let e = Error::new(10, 23);

        assert_eq!(e.tries(), 10);
        assert_eq!(e.into_source(), 23);
    }

    #[test]
    fn error_message() {
        let e = Error::new(3, "it went badly wrong");

        assert_eq!(
            e.to_string(),
            "Tried 3 times. Most recent error: it went badly wrong"
        );
    }
}
