use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected error: {}", .0)]
    Unexpected(String),
    #[error("Could not register application on instance {}", .0)]
    MasApplicationRegister(String, #[source] reqwest::Error),
    #[error("Database error. This is unexpected")]
    DatabaseError(#[from] diesel::result::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
