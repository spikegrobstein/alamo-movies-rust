#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse URL: {0}")]
    ParseError(#[from] url::ParseError),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
