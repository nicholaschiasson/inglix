use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
	Router::new().nest_service(
		"/",
		ServeDir::new("rsrc").not_found_service(ServeFile::new("rsrc/notfound.html")),
	)
}
