use super::loader::chunk_text;
use chroma::ChromaCollection;
use chroma::types::{Key, Metadata, MetadataValue};
use rig::DynModel;
use rig::operation::Embedding;

pub struct IngestResult {
    pub document_id: String,
    pub chunk_ids: Vec<String>,
}

pub async fn ingest_document(
    file_content: &str,
    document_id: &str,
    embedding_model: &DynModel<Embedding>,
    collection: &ChromaCollection,
) -> anyhow::Result<IngestResult> {
    let chunks: Vec<String> = chunk_text(file_content, 500);

	let prefixed: Vec<String> = chunks.iter().map(|c| format!("passage: {c}")).collect();

    let embeddings: Vec<Vec<f32>> = embedding_model
        .call(prefixed)
        .await?
        .embeddings
        .into_iter()
        .map(|e| e.vec.into_iter().map(|v| v as f32).collect())
        .collect();

    let ids: Vec<String> = (0..chunks.len())
        .map(|i| format!("{}_{}", document_id, i))
        .collect();

    let mut metadata = Metadata::new();
    metadata.insert(
        "document_id".to_string(),
        MetadataValue::Str(document_id.to_string()),
    );
    let metadatas: Vec<Option<Metadata>> = vec![Some(metadata); chunks.len()];

    let filter = Key::field("document_id").eq(document_id.to_string());
    collection.delete(None, Some(filter), None).await?;

    collection
        .add(
            ids.clone(),
            embeddings,
            Some(chunks.into_iter().map(Some).collect()),
            None,
            Some(metadatas),
        )
        .await?;

    Ok(IngestResult {
        document_id: document_id.to_string(),
        chunk_ids: ids,
    })
}