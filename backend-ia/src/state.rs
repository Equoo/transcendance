use std::sync::Arc;

use chroma::ChromaCollection;
use rig::Agent;
use rig::DynModel;
use rig::operation::Embedding;

#[derive(Clone)]
pub struct AppState {
    pub rag_agent: Arc<Agent>,
    pub vision_agent: Arc<Agent>,
    pub embedding_model: DynModel<Embedding>,
    pub collection: ChromaCollection,
}

impl AppState {
    pub fn new(
        rag_agent: Agent,
        vision_agent: Agent,
        embedding_model: DynModel<Embedding>,
        collection: ChromaCollection,
    ) -> Self {
        Self {
            rag_agent: Arc::new(rag_agent),
            vision_agent: Arc::new(vision_agent),
            embedding_model,
            collection,
        }
    }
}