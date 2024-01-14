use axum::{
    extract::{Extension, FromRequest},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool}
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppUser {
    pub id: Uuid,
    pub name: String,
    pub created_on: chrono::DateTime<chrono::Utc>,
    pub updated_on: chrono::DateTime<chrono::Utc>,
}

impl AppUser {
    fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            created_on: now,
            updated_on: now,
        }
    }
}

#[derive(Debug)]
pub struct CurrentUser(pub AppUser);

pub struct UnauthorizedUser;
impl IntoResponse for UnauthorizedUser {
    fn into_response(self) -> askama_axum::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
