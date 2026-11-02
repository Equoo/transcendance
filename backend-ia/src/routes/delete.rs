use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chroma::types::Key;
use std::time::Instant;
use tracing::{error, info, instrument};

use crate::state::AppState;

#[instrument(skip_all, fields(document_id = %document_id))]
pub async fn delete_document(
    State(app_state): State<AppState>,
    Path(document_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let started = Instant::now();
    info!("Deletion request received");

    let filter = Key::field("document_id").eq(document_id.clone());

    app_state
        .collection
        .delete(None, Some(filter), None)
        .await
        .map_err(|e| {
            error!(error = %e, "Document deletion failed");
            StatusCode::BAD_GATEWAY
        })?;

    info!(
        elapsed_ms = started.elapsed().as_millis() as u64,
        "Document deleted"
    );

    Ok(Json(serde_json::json!({
        "status": "deleted",
        "document_id": document_id
    })))
}