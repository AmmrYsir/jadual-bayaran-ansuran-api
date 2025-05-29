use axum::{Router, routing::get};
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	let app = Router::new().route("/", get(hello_handler));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
	"Hello, World!"
}