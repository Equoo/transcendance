mod rag;
mod routes;
mod state;
mod types;

use anyhow::Result;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use rag::init_rag_agent;
use state::AppState;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

	tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .with_target(false)
        .init();

    let rag = init_rag_agent().await?;
	info!("Starting RAG agent");

    let app_state = AppState::new(
        rag.rag_agent,
        rag.vision_agent,
        rag.embedding_model,
        rag.collection,
    );
    let app = Router::new()
        .route("/chat/stream", post(routes::chat::chat_stream_handler))
        .route("/health", get(routes::health::health_check))
        .route(
            "/documents/{document_id}",
            post(routes::ingest::ingest).delete(routes::delete::delete_document),
        )
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .with_state(app_state);

	let addr = "0.0.0.0:7070";
    let listener = TcpListener::bind(addr).await?;

	info!(%addr, "Server Listening");

    axum::serve(listener, app).await?;

    Ok(())
}