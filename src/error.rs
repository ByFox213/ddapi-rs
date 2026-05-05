use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound,
    EmptyBody,
    /// DDStats sometimes returns `{ "error": "..." }` with HTTP 200.
    RemoteMessage(String),
    Http(reqwest::Error),
    HttpStatus {
        status: reqwest::StatusCode,
        body: String,
    },
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not found"),
            Error::EmptyBody => write!(f, "Empty response body"),
            Error::RemoteMessage(msg) => write!(f, "{msg}"),
            Error::Http(e) => write!(f, "HTTP error: {e}"),
            Error::HttpStatus { status, body } => write!(f, "HTTP status {status}: {body}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(e) => Some(e),
            Error::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Http(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}
