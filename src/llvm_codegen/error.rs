#[derive(Debug)]
pub struct CodegenError {
    details: String
}

pub type CodegenResult<T> = Result<T, CodegenError>;

impl CodegenError {
    pub fn new(details: &str) -> CodegenError {
        CodegenError{details: details.to_string()}
    }
}
