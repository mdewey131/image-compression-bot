use axum::{
    extract::{DefaultBodyLimit, MultiPart},
    response::Html, 
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // Registering the tracing subscriber, which allows us to collect logs
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "compreston-site=debug, tower=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Build the application with two routes
    let router = Router::new().route("/hello", get(hello_world));

    Ok(router.into())
}