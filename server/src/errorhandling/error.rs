use axum::http::StatusCode;
use axum::response::IntoResponse;

type ErrorResponse = (StatusCode, String);

#[derive(Debug)]
pub struct PageError {
    pub status_code: StatusCode,
    pub message: Option<String>,
}

pub async fn handle_error(error: PageError) -> impl IntoResponse {
    let status_code = error.status_code;
    let message = error.message.unwrap_or_else(|| {
        status_code
            .canonical_reason()
            .unwrap_or_default()
            .to_string()
    });
    (status_code, message)
}
