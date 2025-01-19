use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not register application on instance {}", .0)]
    MasApplicationRegister(String, #[source] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
