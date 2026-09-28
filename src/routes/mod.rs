use crate::middleware;
use crate::routes::pages::{ai, home, not_found, not_found_page, privacy_policy, speedyweb};
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

pub mod pages;

pub fn router() -> Router {
	Router::new()
		.route("/", get(home))
		.route("/speedyweb", get(speedyweb))
		.route("/ai", get(ai))
		.route("/privacy", get(privacy_policy))
		.route("/404", get(not_found_page))
		.nest_service("/static", ServeDir::new("static"))
		.fallback(get(not_found))
		.layer(axum::middleware::from_fn(middleware::log_request))
}
