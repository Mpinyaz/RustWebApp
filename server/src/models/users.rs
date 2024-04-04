use axum::{
    extract::{Extension, FromRequest},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct AppUser {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl AppUser {
    fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            created_at: chrono::Utc::now(),
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
