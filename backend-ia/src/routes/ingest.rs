use axum::extract::{Json, Multipart, Path, State};
use axum::http::StatusCode;
use std::time::Instant;
use tracing::{error, info, instrument, warn};

use crate::rag::ingestion::ingest_document;
use crate::rag::loader::load_document;
use crate::rag::vision::{describe_image, is_image_extension};

#[instrument(skip_all, fields(document_id = %document_id))]
pub async fn ingest(
    State(state): State<crate::state::AppState>,
    Path(document_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let started = Instant::now();
    info!("Ingestion request received");

    let mut file_content = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!(error = %e, "Failed to read multipart field");
        StatusCode::BAD_GATEWAY
    })? {
        if field.name() == Some("file") {
            let extension = field
                .file_name()
                .and_then(|name| name.rsplit('.').next())
                .unwrap_or("")
                .to_string();
            let data = field.bytes().await.map_err(|e| {
                error!(error = %e, "Failed to read uploaded file bytes");
                StatusCode::BAD_REQUEST
            })?;
            info!(%extension, size_bytes = data.len(), "File received");

            let t = Instant::now();
            let document_loaded = if is_image_extension(&extension) {
                info!("Image detected, describing with vision agent");
                describe_image(&state.vision_agent, &data, &extension)
                    .await
                    .map_err(|e| {
                        error!(error = %format!("{e:#}"), "Vision description failed");
                        StatusCode::BAD_REQUEST
                    })?
            } else {
                load_document(&data, &extension).map_err(|e| {
                    warn!(%extension, error = %format!("{e:#}"), "Document loading failed");
                    StatusCode::BAD_REQUEST
                })?
            };
            info!(
                elapsed_ms = t.elapsed().as_millis() as u64,
                chars = document_loaded.chars().count(),
                "Document content extracted"
            );

            file_content = Some(document_loaded);
            break;
        }
    }

    let file_content = match file_content {
        Some(content) => content,
        None => {
            warn!("No 'file' field found in multipart request");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let result = ingest_document(
        &file_content,
        &document_id,
        &state.embedding_model,
        &state.collection,
    )
    .await
    .map_err(|e| {
        error!(error = %format!("{e:#}"), "Document ingestion failed");
        StatusCode::BAD_GATEWAY
    })?;

    info!(
        chunks = result.chunk_ids.len(),
        elapsed_ms = started.elapsed().as_millis() as u64,
        "Ingestion completed"
    );

    Ok(Json(
        serde_json::json!({"status": "success", "document_id": result.document_id, "chunks": result.chunk_ids}),
    ))
}