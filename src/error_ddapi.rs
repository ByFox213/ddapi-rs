use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ApiError {
    ReqwestError(String),
    JsonError(String),
}

impl From<ReqwestError> for ApiError {
    fn from(err: ReqwestError) -> Self {
        ApiError::ReqwestError(err.to_string())
    }
}

impl From<JsonError> for ApiError {
    fn from(err: JsonError) -> Self {
        ApiError::JsonError(err.to_string())
    }
}