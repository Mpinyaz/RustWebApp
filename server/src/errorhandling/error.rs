use axum::http::StatusCode;
use axum::response::IntoResponse;
use eyre::bail;

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

pub async fn handling_errors() -> Result<(), (StatusCode, String)> {
    will_fail().map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
}

fn will_fail() -> eyre::Result<()> {
    bail!("I always fail");
}
