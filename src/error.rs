#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Error(String);

impl<T> From<T> for Error
where
    T: ToString,
{
    fn from(error: T) -> Self {
        Self(error.to_string())
    }
}