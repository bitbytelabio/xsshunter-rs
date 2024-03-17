pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Env error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Parse int error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Parse multipart error: {0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    #[error("Error: {0}")]
    Other(String),
}
