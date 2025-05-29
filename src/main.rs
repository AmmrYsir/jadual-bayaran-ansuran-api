mod routes;

use tokio::net::TcpListener;
use std::net::SocketAddr;

use routes::route_register::create_routes;

#[tokio::main]
async fn main() {
	let app = create_routes();
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app).await.unwrap();
}