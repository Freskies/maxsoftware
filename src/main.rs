pub mod templates;
pub mod routes;
pub mod middleware;

use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	init_tracing();
	let router: Router = routes::router();
	let listener: TcpListener = bind_tcp_listener().await?;
	axum::serve(listener, router)
		.with_graceful_shutdown(shutdown_signal())
		.await?;
	Ok(())
}

fn init_tracing() {
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::INFO)
		.with_target(false)
		.compact()
		.init();
}

async fn bind_tcp_listener() -> Result<TcpListener, Box<dyn Error>> {
	let listener = TcpListener::bind("127.0.0.1:3000").await?;
	tracing::info!(address = %listener.local_addr()?, "Server started.");
	Ok(listener)
}

async fn shutdown_signal() {
	match tokio::signal::ctrl_c().await {
		Ok(_) => {
			tracing::info!("Shutdown requested")
		}
		Err(error) => {
			tracing::error!(%error, "Failed to listen for Ctrl+C; initiating shutdown");
		}
	}
}
