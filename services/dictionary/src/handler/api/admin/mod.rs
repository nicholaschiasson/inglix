use axum::Router;

use crate::AppState;

pub mod v1;

pub fn router<S: AppState>() -> Router<S> {
    Router::new().nest("/v1", v1::router())
}
