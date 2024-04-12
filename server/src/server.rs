// Axum web app server
mod db;
mod fileserv;
mod handlers;
mod models;
mod routes;
mod utils;

use app::*;
use axum::routing::get;
use axum::Router;
use fileserv::file_and_error_handler;
use handlers::main_handlers::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use models::state::AppState;
use sqlx::migrate::Migrator;
use tracing::info;
use utils::config::config;
use utils::db_pool::get_database_pool;
use utils::tracing::init_tracing;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();
    // Load configuration
    let config = config().await;
    // Establish database connection pool
    let db_pool = get_database_pool(config.db_url()).await.unwrap();
    MIGRATOR.run(&db_pool).await.unwrap();

    if config.server.is_valid_ip().unwrap() {
        info!("🚀 Server starting...");
        // Start your server here
        let routes = generate_route_list(App);
        let app_state = AppState {
            pool: db_pool.clone(),
            leptos: config.leptos_options.clone(),
        };

        let app = Router::new()
            // .route("/", post(server_fn_handler))
            // .leptos_routes(&config.leptos_options, routes, App)
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .fallback(file_and_error_handler)
            .merge(routes::pages::init_router())
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(config.server.bind_host().await)
            .await
            .unwrap();
        info!(
            "Server is intialized and listening on: {}:{}",
            config.server_host(),
            config.server_port()
        );
        axum::serve(listener, app).await.unwrap();
    } else {
        panic!("Invalid IP address. Server not started.");
    }
}
