use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use sqlx::Error as SqlxError;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct PageError {
    pub status_code: StatusCode,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct ApiError {
    code: StatusCode,
    user_facing_message: String,
}

#[derive(Debug)]
pub enum InfraError {
    DatabaseError,
    NotFound,
}

#[derive(Debug)]
pub enum PostError {
    InternalServerError,
    NotFound(Uuid),
    InfraError(InfraError),
}

impl ApiError {
    pub fn new(user_facing_message: &str) -> Self {
        let code = StatusCode::INTERNAL_SERVER_ERROR;

        Self {
            code,
            user_facing_message: user_facing_message.into(),
        }
    }

    pub fn set_code(mut self, code: StatusCode) -> Self {
        self.code = code;

        self
    }

    pub fn log(self, error: impl Error) -> Self {
        tracing::error!("{error}");
        self
    }

    pub fn from_database_error(message: &str, sqlx_error: SqlxError) -> ApiError {
        tracing::error!("Failed to get project with id {:?} . ", sqlx_error);

        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
        match sqlx_error {
            SqlxError::RowNotFound => status_code = StatusCode::NOT_FOUND,
            _ => status_code = StatusCode::INTERNAL_SERVER_ERROR,
        }
        return ApiError {
            code: status_code,
            user_facing_message: message.to_string(),
        };
    }
}

impl Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.user_facing_message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> askama_axum::Response {
        (self.code, self.user_facing_message).into_response()
    }
}

impl IntoResponse for PostError {
    fn into_response(self) -> askama_axum::Response {
        let (status, err_msg) = match self {
            PostError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            PostError::NotFound(_) => (StatusCode::NOT_FOUND, "Not Found"),
            PostError::InfraError(infra_error) => match infra_error {
                InfraError::DatabaseError => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                }
                InfraError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            },
        };
        (status,
            Json(json!({"resource": "PostModel", "message": err_msg, "logged_at": chrono::Utc::now()}))).into_response()
    }
}
