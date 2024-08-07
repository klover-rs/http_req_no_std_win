use core::fmt;

pub struct RequestError {
    description: &'static str,
    error_code: u32
}

impl RequestError {
    #[inline]
    pub fn new(description: &'static str, error_code: u32) -> Self {
        Self {
            description,
            error_code
        }
    }
    #[inline]
    pub fn description(&self) -> &'static str {
        self.description
    }
    #[inline]
    pub fn error_code(&self) -> u32 {
        self.error_code
    }
}

impl fmt::Debug for RequestError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RequestError: {} (Code: {})", self.description, self.error_code)
    }
}

impl fmt::Display for RequestError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RequestError: {} (Code: {})", self.description, self.error_code)
    }
}

pub type ReqResult<T> = core::result::Result<T, RequestError>;