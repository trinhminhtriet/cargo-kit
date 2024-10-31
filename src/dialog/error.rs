use inquire::InquireError;

#[derive(thiserror::Error, Debug)]
pub enum DialogError {
    #[error("The dialog was interupted")]
    Interrupted,
    #[error("A generic error has occurred")]
    Generic(#[from] anyhow::Error),
}

pub type PromptResult<T> = Result<T, DialogError>;

impl From<InquireError> for DialogError {
    fn from(value: InquireError) -> Self {
        match value {
            InquireError::OperationInterrupted => Self::Interrupted,
            InquireError::OperationCanceled => Self::Interrupted,
            error => Self::Generic(error.into()),
        }
    }
}
