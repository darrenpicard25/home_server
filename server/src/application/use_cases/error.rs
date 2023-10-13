use crate::domain::repositories::error::RepositoryError;

#[derive(Debug)]
pub enum UseCaseError {
    NotFound,
    Unknown,
    BadInput,
}

pub type UseCaseResult<T> = Result<T, UseCaseError>;

impl From<RepositoryError> for UseCaseError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Unknown(s) => {
                tracing::error!("Unknown error occurred: {s}");

                Self::Unknown
            }
        }
    }
}
