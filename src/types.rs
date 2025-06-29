use serde::{Deserialize, Serialize};
use worker::{Response, Result as WorkerResult, kv::KvError};
use http::StatusCode;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub key: String,
}

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct ProgressState {
    pub document: String,
    pub percentage: f32,
    pub progress: String,
    pub device: String,
    pub device_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum AppError {
    StoreError,
    UnknownServerError,
    Unauthorized,
    UsernameAlreadyRegistered,
    InvalidRequest,
    DocumentNotProvided,
    Custom{message: String},
}

impl AppError {
    pub fn code(&self) -> u16 {
        match self {
            AppError::StoreError => 1000,
            AppError::UnknownServerError => 2000,
            AppError::Unauthorized => 2001,
            AppError::UsernameAlreadyRegistered => 2002,
            AppError::InvalidRequest => 2003,
            AppError::DocumentNotProvided => 2004,
            AppError::Custom { .. } => 3000,
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            AppError::StoreError => StatusCode::BAD_GATEWAY,
            AppError::UnknownServerError => StatusCode::BAD_GATEWAY,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::UsernameAlreadyRegistered => StatusCode::PAYMENT_REQUIRED,
            AppError::InvalidRequest => StatusCode::FORBIDDEN,
            AppError::DocumentNotProvided => StatusCode::FORBIDDEN,
            AppError::Custom { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AppError::UnknownServerError => "Unknown server error.",
            AppError::Unauthorized => "Unauthorized",
            AppError::UsernameAlreadyRegistered => "Username is already registered.",
            AppError::InvalidRequest => "Invalid request",
            AppError::DocumentNotProvided => "Field 'document' not provided.",
            AppError::StoreError => "Cannot connect to redis server",
            AppError::Custom { message, .. } => message,
        }
    }
}

impl From<AppError> for WorkerResult<Response> {
    fn from(error: AppError) -> Self {
        let json_value = serde_json::json!({
            "code": error.code(),
            "message": error.message()
        });

        Response::from_json(&json_value)
            .map(|mut response| {
                response.headers_mut().set("Content-Type", "application/json").ok();
                response.with_status(error.status().as_u16())
            })
    }
}

impl From<KvError> for AppError {
    fn from(_error: KvError) -> Self {
        AppError::StoreError
    }
}
