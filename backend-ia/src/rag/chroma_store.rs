use chroma::ChromaCollection;
use rig::DynModel;
use rig::operation::Embedding;
use rig::vector_store::request::Filter;
use rig::vector_store::request::VectorSearchRequest;
use rig::vector_store::{VectorStoreError, VectorStoreIndex};
use serde_json::json;

pub struct ChromaVectorIndex {
    collection: ChromaCollection,
    embedding_model: DynModel<Embedding>,
}

impl ChromaVectorIndex {
    pub fn new(collection: ChromaCollection, embedding_model: DynModel<Embedding>) -> Self {
        Self {
            collection,
            embedding_model,
        }
    }

    async fn raw_query(
        &self,
        query: &str,
        n: u64,
    ) -> Result<(Vec<String>, Vec<Option<String>>, Vec<Option<f32>>), VectorStoreError> {
        let query_embedding = self
            .embedding_model
            .embed_text(query)
            .await
            .map_err(VectorStoreError::EmbeddingError)?;

        let query_vec = query_embedding
            .vec
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<f32>>();

        let results = self
            .collection
            .query(vec![query_vec], Some(n as u32), None, None, None)
            .await
            .map_err(|e| VectorStoreError::DatastoreError(Box::new(e)))?;

        let ids = results.ids.into_iter().next().unwrap_or_default();

        let docs = results
            .documents
            .and_then(|d| d.into_iter().next())
            .unwrap_or_else(|| vec![None; ids.len()]);

        let distances = results
            .distances
            .and_then(|d| d.into_iter().next())
            .unwrap_or_else(|| vec![None; ids.len()]);

        Ok((ids, docs, distances))
    }
}

impl VectorStoreIndex for ChromaVectorIndex {
    type Filter = Filter<serde_json::Value>;

    async fn top_n<T: for<'a> serde::Deserialize<'a> + Send>(
        &self,
        req: VectorSearchRequest<Self::Filter>,
    ) -> Result<Vec<(f64, String, T)>, VectorStoreError> {
        let (ids, docs, distances) = self.raw_query(req.query(), req.samples()).await?;

        ids.into_iter()
            .zip(docs)
            .zip(distances)
            .map(|((id, doc), distance)| {
                let value: T = serde_json::from_value(json!({ "document": doc }))
                    .map_err(VectorStoreError::JsonError)?;
                Ok((1.0 - distance.unwrap_or(1.0) as f64, id, value))
            })
            .collect()
    }

    async fn top_n_ids(
        &self,
        req: VectorSearchRequest<Self::Filter>,
    ) -> Result<Vec<(f64, String)>, VectorStoreError> {
        let (ids, _docs, distances) = self.raw_query(req.query(), req.samples()).await?;

        Ok(ids
            .into_iter()
            .zip(distances)
            .map(|(id, distance)| (1.0 - distance.unwrap_or(1.0) as f64, id))
            .collect())
    }
}
