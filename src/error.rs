use core::fmt;
use std::{error, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cause {
    ExpectsEnum,
    ExpectsObject,
    InvalidUtf8,
    IsMandatory,
    IsInvalid,
    MustBeArray,
    MustBeBoolean,
    MustBeObject,
    MustBeString,
    MustBeUnsignedInteger,
    MustMapToStringOrNull,
    UnexpectedEof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error {
    attribute: &'static str,
    cause: Cause,
}

impl Error {
    pub const fn new(attribute: &'static str, cause: Cause) -> Self {
        Self { attribute, cause }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Error as fmt::Debug>::fmt(self, f)
    }
}

impl error::Error for Error {}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, e)
    }
}
