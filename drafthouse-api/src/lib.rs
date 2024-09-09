mod api;
mod client;
mod error;
mod model;
mod response;

pub use client::Client;
pub use error::Error;
pub use response::Response;

/// A convenience type with a default error type of [`Error`].
pub type Result<T, E = Error> = std::result::Result<T, E>;
