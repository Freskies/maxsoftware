use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn log_request(request: Request, next: Next) -> Response {
	let method = request.method().clone();
	let path = request.uri().path().to_owned();
	let started_at = Instant::now();

	tracing::info!(%method, %path, "Request received");

	let response: Response = next.run(request).await;
	let duration_ms = started_at.elapsed().as_secs_f64() * 1000.0;

	tracing::info!(%method, %path, status = response.status().as_u16(), duration_ms, "Response ready");

	response
}
