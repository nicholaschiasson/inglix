use axum::Router;
use dotenv::dotenv;
use ing::AppState;
use reqwest::Client;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct State {
	reqwest_client: Client,
}

impl State {
	pub fn new(client: Client) -> Self {
		Self { reqwest_client: client }
	}
}

impl AppState for State {
	fn client(&self) -> &Client {
			&self.reqwest_client
	}
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::new(
			std::env::var("RUST_LOG").unwrap_or("ing=debug,tower_http=debug".into()),
		))
		.with(tracing_subscriber::fmt::layer())
		.init();

	let state = State::new(Client::new());

	let app = Router::new()
		.nest_service("/", ing::router::<State>())
		.with_state(state)
		.layer(TraceLayer::new_for_http());

	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
	tracing::debug!("listening on {}", listener.local_addr()?);
	axum::serve(listener, app).await
}
