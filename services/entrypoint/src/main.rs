use axum::Router;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::new(
			std::env::var("RUST_LOG").unwrap_or("entrypoint=debug,inglix=debug,tower_http=debug".into()),
		))
		.with(tracing_subscriber::fmt::layer())
		.init();

	let db_connection_str = std::env::var("DATABASE_URL").unwrap_or("inglix.db".to_string());

	let state = dictionary::State::new(
		SqlitePoolOptions::default()
			.acquire_timeout(Duration::from_secs(3))
			.connect(&db_connection_str)
			.await
			.expect("database connection"),
	);

	let app = Router::new()
		.nest("/api", dictionary::router::<dictionary::State>())
		.nest("/", ing::router::<dictionary::State>())
		.with_state(state)
		.layer(TraceLayer::new_for_http());

	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
	tracing::debug!("listening on {}", listener.local_addr()?);
	axum::serve(listener, app).await
}
