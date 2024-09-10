mod api;
mod client;
mod error;
mod model;
mod response;

pub use client::Client;
pub use error::{Error, V2Error, V2ErrorBody, V2ErrorCode};
pub use response::Response;

/// A convenience type with a default error type of [`Error`].
pub type Result<T, E = Error> = std::result::Result<T, E>;
