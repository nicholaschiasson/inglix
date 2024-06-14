use axum::Router;

use crate::AppRouter;

pub mod v1;

pub fn router() -> AppRouter {
    Router::new().nest("/v1", v1::router())
}
