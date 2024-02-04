use crate::handlers::main_handlers::*;
use crate::models::state::AppStateType;
use axum::routing::get;
use axum::Router;
use hyper::Method;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub mod errors;
pub mod json;

pub fn init_router() -> Router<AppStateType> {
    let assets_path = std::env::current_dir().unwrap();
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![
            axum::http::Method::OPTIONS,
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
        ])
        .allow_headers(vec![
            axum::http::header::ORIGIN,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ]);

    Router::new()
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .route("/", get(home))
        .route("/name/:name", get(name_handler))
        .route("/greeting", get(greeting_handler))
        .layer(cors)
}
