use crate::controllers::service_controller::json_handler;
use crate::models::state::AppStateType;

use axum::{
    routing::{get, post},
    Router,
};
pub fn json_router() -> Router<AppStateType> {
    Router::new().route("/json", post(json_handler))
}
