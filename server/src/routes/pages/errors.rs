use crate::models::errors::PageError;
use axum::http::StatusCode;
use axum::response::IntoResponse;

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
