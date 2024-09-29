use axum::Router;
use sqlx::SqlitePool;

mod dal;
mod handler;
mod model;

pub trait AppState: Clone + Send + Sync + 'static {
  fn pool(&self) -> &SqlitePool;
}

#[derive(Clone)]
pub struct State {
    sqlite_pool: SqlitePool,
}

impl State {
  pub fn new(pool: SqlitePool) -> Self {
    Self {
      sqlite_pool: pool,
    }
  }
}

impl AppState for State {
  fn pool(&self) -> &SqlitePool {
    &self.sqlite_pool
  }
}

pub fn router<S: AppState>() -> Router<S> {
  handler::api::router()
}
