use axum::Router;
use sqlx::SqlitePool;

mod dal;
mod handler;
mod model;

pub trait AppState: Clone + Send + Sync + 'static {
	fn pool(&self) -> &SqlitePool;
}

pub fn router<S: AppState>() -> Router<S> {
	handler::api::router()
}
