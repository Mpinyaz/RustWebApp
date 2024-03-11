// Axum web app server
mod db;
mod models;
mod routes;
mod utils;

use axum::Router;

use models::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use utils::config::config;
use utils::db_pool::get_database_pool;
use utils::tracing::init_tracing;

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();
    // Load configuration
    let config = config().await;
    // Estblish database connection pool
    let db_pool = get_database_pool(config.db_url()).await.unwrap();

    if config.server.is_valid_ip().unwrap() {
        info!("🚀 Server starting...");
        // Start your server here
        let appstate = Arc::new(RwLock::new(AppState { pool: db_pool }));
        let app = Router::new()
            .merge(routes::pages::init_router())
            .merge(routes::pages::json::json_router())
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .with_state(appstate);

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
