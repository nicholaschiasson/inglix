use axum::{
	body::Body,
	extract::Request,
	http::{Response, StatusCode, Uri},
	middleware::Next,
	response::IntoResponse,
	Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorPayload {
	pub message: String,
}

pub async fn not_found(uri: Uri) -> (StatusCode, String) {
	(StatusCode::NOT_FOUND, format!("resource '{uri}' not found"))
}

pub async fn handle_error(
	req: Request,
	next: Next,
) -> Result<Response<Body>, (StatusCode, Json<ErrorPayload>)> {
	let (parts, body) = next.run(req).await.into_parts();
	if parts.status.is_client_error() || parts.status.is_server_error() {
		let body = axum::body::to_bytes(body, usize::MAX)
			.await
			.unwrap_or_default()
			.to_vec();
		let body = String::from_utf8_lossy(&body);
		Err((
			parts.status,
			Json(ErrorPayload {
				message: body.to_string(),
			}),
		))
	} else {
		Ok((parts, body).into_response())
	}
}
