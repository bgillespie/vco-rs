use reqwest::Error as RequestError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use api_v1::Integer;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ClientError {
    #[error("Error making request: {0:?}")]
    Request(RequestError),

    #[error("Client create error: {0:?}")]
    ReqwestClientCreate(RequestError),

    #[error("Response error: {0:?}")]
    Response(RequestError),

    #[error("Error returned from API: {0:?}")]
    Api(String),

    #[error("JSON error: {0:?}")]
    Json(serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: Integer,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    pub valid: bool,
    pub error: Option<ErrorValidationDetails>,
    pub warn: Option<ErrorValidationDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorValidationDetails {
    pub code: String,
    pub message: String,
    pub path: String,
}
