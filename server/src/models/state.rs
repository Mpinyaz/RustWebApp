use axum::extract::FromRef;
use leptos::LeptosOptions;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub leptos: LeptosOptions,
}
impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.pool.clone()
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(app_state: &AppState) -> LeptosOptions {
        app_state.leptos.clone()
    }
}
pub type AppStateType = Arc<RwLock<AppState>>;
