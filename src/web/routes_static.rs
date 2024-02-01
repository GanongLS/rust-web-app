use crate::config;
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

// Note: Here we can just return a MethodRouter rather than a full Router
//       since ServeDir is a service.
pub fn serve_dir() -> MethodRouter {
	async fn handle_404() -> (StatusCode, &'static str) {
		let message: &'static str = ("Resource not found");
		(StatusCode::NOT_FOUND, message)
	}

	any_service(
		ServeDir::new(&config().WEB_FOLDER).not_found_service(handle_404.into_service()),
	)
}
