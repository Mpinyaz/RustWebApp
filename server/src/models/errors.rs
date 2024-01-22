use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::error::Error;

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
