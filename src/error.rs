use std::{error::Error, fmt};

pub type GenericError = Box<Error>;
pub type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug)]
pub struct DispError {
    details: String,
}

pub type DispResult<T> = Result<T, DispError>;

impl DispError {
    // TODO: change function signature to string.
    // this will save unneeded reallocations.
    pub fn new(details: &str) -> DispError {
        DispError {
            details: details.to_string(),
        }
    }
}

impl Error for DispError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for DispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.details)
    }
}
