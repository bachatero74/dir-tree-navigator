use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    StrError(String),

    #[error("Błąd IO: {0}")]
    IoError(#[from] std::io::Error),
}
