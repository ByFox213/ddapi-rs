use std::fmt;

#[derive(Clone, Debug)]
pub enum ApiError {
    NotFound,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Player not found"),
        }
    }
}

impl std::error::Error for ApiError {}
