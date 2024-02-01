// use super::users::*;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub type AppStateType = Arc<RwLock<AppState>>;
