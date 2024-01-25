mod config;
mod controllers;
mod db;
mod errorhandling;
mod models;
mod routes;

use crate::config::load_config;
use axum::Router;
use clap::Parser;
use models::state::AppState;
use server::Cli;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();

    let opt = Cli::parse();
    let appaddr_settings = match (opt.host, opt.port) {
        (Some(host), Some(port)) => {
            server::AppAddrSettings::new(Some(host), Some(port.parse::<String>().unwrap()))
        }
        (Some(host), None) => {
            server::AppAddrSettings::new(Some(host), server::AppAddrSettings::default().port)
        }
        (None, Some(port)) => {
            server::AppAddrSettings::new(server::AppAddrSettings::default().host, Some(port))
        }
        (None, None) => server::AppAddrSettings::default(),
    };

    if appaddr_settings.is_valid_ip().unwrap() {
        info!("🚀 Server starting...");
        // Start your server here
        let appstate = Arc::new(RwLock::new(AppState {
            appaddr_settings: appaddr_settings.clone(),
        }));
        let app = Router::new()
            .merge(routes::pages::init_router())
            .merge(routes::pages::json::json_router())
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .with_state(appstate);

        let listener = tokio::net::TcpListener::bind(&appaddr_settings.bind_host())
            .await
            .unwrap();
        info!(
            "Server is intialized and listening on: localhost:{}",
            appaddr_settings.port.unwrap()
        );

        axum::serve(listener, app).await.unwrap();
    } else {
        panic!("Invalid IP address. Server not started.");
    }
}

fn init_tracing() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "server=debug,tower_http=debug".to_owned()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
