use axum::{
    extract::{Extension, FromRequest},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct CurrentUser(pub AppUser);

pub struct UnauthorizedUser;
impl IntoResponse for UnauthorizedUser {
    fn into_response(self) -> askama_axum::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
