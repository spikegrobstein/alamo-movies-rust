use reqwest::StatusCode;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse URL: {0}")]
    ParseError(#[from] url::ParseError),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("API Error [{status}]: {body}")]
    HttpError { status: StatusCode, body: String },

    #[error("API Error [{status}]: {}", error.description)]
    ApiError { status: StatusCode, error: V2Error },
}

#[derive(Debug, Deserialize)]
pub struct V2ErrorBody {
    pub error: V2Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2Error {
    pub errorcode: V2ErrorCode,
    pub description: String,
    pub error_type: String,
}

#[derive(Debug, Deserialize)]
pub struct V2ErrorCode {
    pub category: u32,
    pub code: u32,
    pub description: String,
}
