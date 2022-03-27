use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlx error")]
    SQLXError(#[from] sqlx::Error),
    #[error("config deserialization error")]
    ConfigError(#[from] serde_yaml::Error),
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("aws sdk credentials error")]
    AWSSDKCredentialsError(#[from] aws_types::credentials::CredentialsError),
    #[error("http error")]
    HTTPError(#[from] http::Error),
}
