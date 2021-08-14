// use std::io::{Error};

#[derive(Debug, Clone)]
pub struct NunkiError {
  description: String,
}

impl NunkiError {
    fn from(io_error: std::io::Error) -> Self {
        NunkiError {
            description: "TODO".to_string()
        }
    }
}