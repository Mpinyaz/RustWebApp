use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_tracing() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "server=debug,tower_http=debug,axum::rejection=info,sqlx=info".to_owned()
            }),
        ))
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(ErrorLayer::default())
        .init();
}
