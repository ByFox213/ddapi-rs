#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Other(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Player not found"),
            ApiError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ApiError {}
