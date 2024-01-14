// use super::users::*;
use serde::{Deserialize, Serialize};
use server::AppAddrSettings;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppState {
    pub appaddr_settings: AppAddrSettings,
}

pub type AppStateType = Arc<RwLock<AppState>>;
