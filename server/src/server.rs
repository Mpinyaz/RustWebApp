// Axum web app server
mod db;
mod models;
mod routes;
mod utils;

mod handlers;
use app::*;
use axum::routing::get;
use axum::Router;
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
// #[cfg(feature = "ssr")]

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
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(App);
        let app_state = AppState {
            pool: db_pool.clone(),
            leptos: leptos_options.clone(),
        };

        // .leptos_routes(&leptos_options,routes, App)
        // .route("/", post(server_fn_handler))
        let app = Router::new()
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .merge(routes::pages::init_router())
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(&leptos_options.site_addr)
            .await
            .unwrap();
        info!(
            "Server is intialized and listening on: localhost:{}",
            &leptos_options.site_addr
        );
        axum::serve(listener, app).await.unwrap();
    } else {
        panic!("Invalid IP address. Server not started.");
    }
}
