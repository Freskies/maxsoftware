use crate::routes::pages::{ai, customized, home, not_found, not_found_page, speedyweb};
use crate::{middleware};
use axum::{Router, routing::get};

pub mod pages;

pub fn router() -> Router {
	Router::new()
		.route("/", get(home))
		.route("/speedyweb", get(speedyweb))
		.route("/customized", get(customized))
		.route("/ai", get(ai))
		.route("/404", get(not_found_page))
		.fallback(get(not_found))
		.layer(axum::middleware::from_fn(middleware::log_request))
}
