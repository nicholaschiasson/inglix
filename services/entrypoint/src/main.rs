use axum::Router;
use dotenv::dotenv;
use reqwest::Client;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;

#[derive(Clone)]
pub struct State {
	reqwest_client: Client,
	sqlite_pool: SqlitePool,
}

impl State {
	pub fn new(client: Client, pool: SqlitePool) -> Self {
		Self { reqwest_client: client, sqlite_pool: pool }
	}
}

impl dictionary::AppState for State {
	fn pool(&self) -> &SqlitePool {
		&self.sqlite_pool
	}
}

impl ing::AppState for State {
	fn client(&self) -> &Client {
		&self.reqwest_client
	}
}

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

	let state = State::new(
		Client::new(),
		SqlitePoolOptions::default()
			.acquire_timeout(Duration::from_secs(3))
			.connect(&db_connection_str)
			.await
			.expect("database connection"),
	);

	let app = Router::new()
		.nest("/api", dictionary::router::<State>())
		.nest("/", ing::router::<State>())
		.with_state(state)
		.layer(TraceLayer::new_for_http());

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
	tracing::debug!("listening on {}", listener.local_addr()?);
	axum::serve(listener, app).await
}
