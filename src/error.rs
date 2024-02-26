//! # 🚨 Errors
//!

use std::fmt;

/// # 🚨 Errors
#[derive(Debug)]
pub enum KlientError {
    /// Client error
    Client,
    /// API connection error
    APIConnection,
    /// Invalid Input
    InvalidInput,
    /// Server side internal error
    InternalServerError,
}

impl std::error::Error for KlientError {}

impl fmt::Display for KlientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Client => write!(f, "client error"),
            Self::APIConnection => write!(f, "API connection Error"),
            Self::InvalidInput => write!(f, "Invalid user input"),
            Self::InternalServerError => write!(f, "Server side internal error"),
        }
    }
}
