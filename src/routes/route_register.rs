use axum::{Router, routing::get};

pub fn create_routes() -> Router {
	Router::new()
		.nest("/api/v1", v1_routes())
}

pub fn v1_routes() -> Router {
	Router::new()
		.route("/", get(|| async { "Hello, World!" }))
		.route("/goodbye", get(|| async { "Goodbye, World!" }))
}
