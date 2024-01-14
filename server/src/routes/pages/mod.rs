use crate::controllers::service_controller::*;
use crate::models::state::AppStateType;
use axum::routing::get;
use axum::Router;
use hyper::Method;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub mod json;

pub fn init_router() -> Router<AppStateType> {
    let assets_path = std::env::current_dir().unwrap();
    Router::new()
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .route("/", get(home))
        .route("/name/:name", get(name_handler))
        .route("/greeting", get(greeting_handler))
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]))
}
