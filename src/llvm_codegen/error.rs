use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CodegenError {
    details: String,
}

pub type CodegenResult<T> = Result<T, CodegenError>;

impl CodegenError {
    pub fn new(details: &str) -> CodegenError {
        CodegenError {
            details: details.to_string(),
        }
    }
}

impl Error for CodegenError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.details)
    }
}
