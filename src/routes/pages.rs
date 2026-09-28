use crate::templates::{AIContentTemplate, AITemplate, PrivacyPolicyTemplate, HomeContentTemplate, HomeTemplate, NotFoundTemplate, SpeedyWebContentTemplate, SpeedyWebTemplate};
use askama::Template;
use axum::http::header::VARY;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};

pub async fn home(headers: HeaderMap) -> Response {
	render_page_or_content(&headers, HomeTemplate, HomeContentTemplate)
}

pub async fn speedyweb(headers: HeaderMap) -> Response {
	render_page_or_content(&headers, SpeedyWebTemplate, SpeedyWebContentTemplate)
}

pub async fn ai(headers: HeaderMap) -> Response {
	render_page_or_content(&headers, AITemplate, AIContentTemplate)
}

pub async fn privacy_policy() -> Response {
	render_page(PrivacyPolicyTemplate, false)
}

pub async fn not_found_page() -> Response {
	match NotFoundTemplate.render() {
		Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
		Err(error) => {
			tracing::error!(%error, template = std::any::type_name::<NotFoundTemplate>(), "Failed to render not-found page");
			(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Impossibile caricare la pagina",
			)
				.into_response()
		}
	}
}

pub async fn not_found(headers: HeaderMap) -> Response {
	let is_htmx: bool = is_request_from_htmx(&headers);

	if is_htmx {
		return (
			StatusCode::NO_CONTENT,
			[("HX-Redirect", "/404"), ("Cache-Control", "no-store")],
		)
			.into_response();
	}

	not_found_page().await
}

pub fn render_page_or_content<P: Template, C: Template>(headers: &HeaderMap, page: P, content: C) -> Response {
	let wants_content: bool = is_request_from_htmx(headers);
	if wants_content {
		render_page(content, true)
	} else {
		render_page(page, false)
	}
}

pub fn render_page<T: Template>(page: T, partial: bool) -> Response {
	let result = page.render();

	let mut response = match result {
		Ok(html) => Html(html).into_response(),
		Err(error) => {
			let template_name = std::any::type_name::<T>();
			tracing::error!(%error, template = template_name, partial, "Failed to render page");
			(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Impossibile caricare la pagina",
			)
				.into_response()
		}
	};

	response
		.headers_mut()
		.insert(VARY, HeaderValue::from_static("HX-Request-Type"));

	response
}

fn is_request_from_htmx(headers: &HeaderMap) -> bool {
	headers
		.get("HX-Request-Type")
		.and_then(|value| value.to_str().ok())
		== Some("partial")
}
