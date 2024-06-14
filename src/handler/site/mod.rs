use axum::Router;
use tower_http::services::ServeDir;

use crate::AppRouter;

pub fn router() -> AppRouter {
    Router::new().nest_service("/", ServeDir::new("rsrc"))
}
