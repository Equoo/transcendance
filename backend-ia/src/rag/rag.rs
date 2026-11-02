use crate::rag::chroma_store::ChromaVectorIndex;
use anyhow::{Context, Result};
use chroma::ChromaCollection;
use chroma::{ChromaHttpClient, client::ChromaHttpClientOptions};
use rig::operation::Embedding;
use rig::prelude::*;
use rig::providers::llamacpp;
use rig_fastembed::{Fastembed, FastembedModel};
use std::time::Instant;
use tracing::{info, instrument};
use url::Url;

pub struct Rag {
    pub rag_agent: Agent,
    pub vision_agent: Agent,
    pub collection: ChromaCollection,
    pub embedding_model: DynModel<Embedding>,
}

#[instrument(name = "init_rag")]
pub async fn init_rag_agent() -> Result<Rag> {
    let total = Instant::now();

    info!("Loading embedding model (may download on first run)");
    let t = Instant::now();
    let embedding_model = Fastembed::load(&FastembedModel::MultilingualE5Small)
        .context("Failed to load Fastembed model")?
        .embedding(&FastembedModel::MultilingualE5Small, None)
        .context("Failed to create embedding model")?
        .erase();
    info!(elapsed_ms = t.elapsed().as_millis() as u64, "Embedding model ready");

    info!("Connecting to ChromaDB");
    let chroma_options = ChromaHttpClientOptions {
        endpoint: Url::parse("http://chroma:8000").context("Invalid Chroma URL")?,
        ..Default::default()
    };
    let chroma_client = ChromaHttpClient::new(chroma_options);
    let collection = chroma_client
        .get_or_create_collection("mes_documents_e5_base", None, None)
        .await
        .context("Unable to access the Chroma collection")?;
    info!("Chroma collection ready");

    let llama_client = llamacpp::from_env()
        .context("llama.cpp configuration not found (environment variables)")?
        .chat(llamacpp::LLAMA_CPP);

    let index = ChromaVectorIndex::new(collection.clone(), embedding_model.clone());

    let rag_agent = AgentBuilder::new(llama_client)
        .preamble("You are an assistant for a group, knowledge and events management app. People will ask you question about the app. You will be provided files and informations by an RAG.
        If you do not find any information don't mention things like RAG or Database, just say that you can't find.")
        .temperature(1.0)
        .dynamic_context(3, index)
        .build();
    info!(top_k = 3, "RAG agent initialized");

    let vision_agent = AgentBuilder::new(
        llamacpp::from_env()
            .context("llama.cpp configuration not found for the vision agent")?
            .chat(llamacpp::LLAMA_CPP),
    )
    .preamble(
        "You describe images to index them in a document database. \
         Describe the content of the received image precisely. If it contains text, \
         a table, or a chart, transcribe them accurately.",
    )
    .temperature(1.0)
    .max_tokens(1024)
    .build();
    info!("Vision agent initialized");

    info!(elapsed_ms = total.elapsed().as_millis() as u64, "RAG initialization complete");

    Ok(Rag {
        rag_agent,
        vision_agent,
        collection,
        embedding_model,
    })
}