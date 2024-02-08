use thiserror::Error as ThisError;

#[derive(Debug, Clone, PartialEq, Eq, ThisError)]
/// An error returned from the API.
#[error("Lemmy Error: {0}")]
pub struct Error(String);

impl Error {
    /// Get the error message;
    pub fn message(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self(e.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl From<gloo_net::Error> for Error {
    fn from(e: gloo_net::Error) -> Self {
        Self(e.to_string())
    }
}
