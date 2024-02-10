use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
    pub status_code: u16,
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK);
        let json_body = axum::Json(self);

        let mut response = json_body.into_response();

        *response.status_mut() = status;

        response
    }
}
