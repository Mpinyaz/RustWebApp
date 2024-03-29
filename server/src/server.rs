// Axum web app server
mod db;
mod models;
mod routes;
mod utils;

mod handlers;
use app::*;
use axum::extract::Extension;
use axum::routing::{get, post};
use axum::Router;
use handlers::main_handlers::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use models::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use utils::config::config;
use utils::db_pool::get_database_pool;
use utils::tracing::init_tracing;
// #[cfg(feature = "ssr")]

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();
    // Load configuration
    let config = config().await;
    // Establish database connection pool
    let db_pool = get_database_pool(config.db_url()).await.unwrap();

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

    if config.server.is_valid_ip().unwrap() {
        info!("🚀 Server starting...");
        // Start your server here
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(|| view! {<App/>});
        let app_state = AppState {
            pool: db_pool.clone(),
            leptos: leptos_options,
        };

        // .leptos_routes(&leptos_options,routes, App)
        // .route("/", post(server_fn_handler))
        let app = Router::new()
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .nest_service(
                "/assets",
                ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
            )
            .route("/", get(home))
            .route("/name/:name", get(name_handler))
            .route("/greeting", get(greeting_handler))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .layer(cors)
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(config.server.bind_host())
            .await
            .unwrap();
        info!(
            "Server is intialized and listening on: localhost:{}",
            config.server_port()
        );
        axum::serve(listener, app).await.unwrap();
    } else {
        panic!("Invalid IP address. Server not started.");
    }
}
