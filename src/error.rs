#[derive(Debug, Clone, PartialEq, Eq)]
/// An error returned from the API.
pub struct Error(String);

impl Error {
    /// Get the error message;
    pub fn message(&self) -> &str {
        self.0.as_str()
    }
}

impl<T> From<T> for Error
where
    T: ToString,
{
    fn from(error: T) -> Self {
        Self(error.to_string())
    }
}
