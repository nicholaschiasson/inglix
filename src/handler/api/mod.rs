use axum::{http::StatusCode, Json, Router};
use serde::Serialize;

use crate::AppRouter;

pub mod admin;
pub mod public;

pub fn router() -> AppRouter {
    Router::new()
        .nest("/admin", admin::router())
        .nest("/public", public::router())
}
