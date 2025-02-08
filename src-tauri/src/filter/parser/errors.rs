use std::error::Error;
use std::fmt;

/// Error type returned when parsing a block line fails.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// Indicates that the input line was empty.
    EmptyLine,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}
