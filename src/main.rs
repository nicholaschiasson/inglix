use axum::{middleware, Router};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;

use crate::handler::error::{handle_error, not_found};

mod dal;
mod handler;
mod model;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}

pub type AppRouter = Router<AppState>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or("inglix=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or("inglix.db".to_string());

    let state = AppState {
        pool: SqlitePoolOptions::default()
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("database connection"),
    };

    let app = Router::new()
        .nest("/api", handler::api::router())
        .fallback(not_found)
        .nest_service("/", ServeDir::new("rsrc"))
        .with_state(state)
        .layer(middleware::from_fn(handle_error))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
