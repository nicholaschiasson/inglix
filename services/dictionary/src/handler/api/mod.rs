use axum::{middleware, routing::method_routing, Router};

use error::{handle_error, not_found};

use crate::AppState;

pub mod admin;
pub mod public;

mod error;

pub fn router<S: AppState>() -> Router<S> {
    Router::new()
        .nest("/admin", admin::router())
        .nest("/public", public::router())
        .route("/*path", method_routing::any(not_found))
        .layer(middleware::from_fn(handle_error))
}
