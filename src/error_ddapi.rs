use reqwest::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApiError {
    ReqwestError(Error),
    JsonError(serde_json::Error),
}

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        ApiError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::JsonError(err)
    }
}
