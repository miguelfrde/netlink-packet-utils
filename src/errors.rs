// SPDX-License-Identifier: MIT

use anyhow::anyhow;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Encode error occurred: {inner}")]
pub struct EncodeError {
    inner: anyhow::Error,
}

impl From<&'static str> for EncodeError {
    fn from(msg: &'static str) -> Self {
        EncodeError {
            inner: anyhow!(msg),
        }
    }
}

impl From<String> for EncodeError {
    fn from(msg: String) -> Self {
        EncodeError {
            inner: anyhow!(msg),
        }
    }
}

impl From<anyhow::Error> for EncodeError {
    fn from(inner: anyhow::Error) -> EncodeError {
        EncodeError { inner }
    }
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error(
        "Invalid MAC address. Expected 6 bytes, received {received} bytes"
    )]
    InvalidMACAddress { received: usize },

    #[error(
        "Invalid IP address. Expected 4 or 16 bytes, received {received} bytes"
    )]
    InvalidIPAddress { received: usize },

    #[error("Invalid string")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(
        "Invalid number. Expected {expected} bytes, received {received} bytes"
    )]
    InvalidNumber { expected: usize, received: usize },

    #[error("Invalid buffer {name}. Expected at least {minimum_length} bytes, received {received} bytes")]
    InvalidBuffer {
        name: &'static str,
        received: usize,
        minimum_length: usize,
    },

    #[error(transparent)]
    Nla(#[from] crate::nla::NlaError),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}

impl From<&str> for DecodeError {
    fn from(msg: &str) -> Self {
        let error: Box<dyn std::error::Error> = msg.to_string().into();
        DecodeError::Other(error)
    }
}

impl From<String> for DecodeError {
    fn from(msg: String) -> Self {
        let error: Box<dyn std::error::Error> = msg.into();
        DecodeError::Other(error)
    }
}
