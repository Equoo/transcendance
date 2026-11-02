use crate::state::AppState;
use crate::types::ChatRequest;
use axum::Json;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};

use rig::prelude::*;
use rig::streaming::{Item, StreamEvent};

use futures::{Stream, StreamExt, stream};
use serde_json::json;
use std::convert::Infallible;
use std::time::Instant;
use tracing::{Instrument, error, info, info_span};

pub async fn chat_stream_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let prompt = payload.message;
    let span = info_span!("chat_stream", prompt_len = prompt.chars().count());
    info!(parent: &span, "Chat stream request received");

    let started = Instant::now();
    let agent = app_state.rag_agent.clone();

    let (run, events) = agent.prompt(prompt.as_str()).run_channel();

    let run = tokio::spawn(
        async move {
            run.await?;
            Ok::<_, PromptError>(())
        }
        .instrument(span.clone()),
    );

    let events = events.filter_map(|item| async move {
        match item {
            MultiTurnStreamItem::StreamAssistantItem(Item::Event(StreamEvent::Text {
                text,
                ..
            })) => Some(Ok(Event::default()
                .event("delta")
                .json_data(json!({"text": text}))
                .unwrap())),
            MultiTurnStreamItem::StreamAssistantItem(Item::Event(StreamEvent::Reasoning {
                text,
                ..
            })) => Some(Ok(Event::default()
                .event("reasoning")
                .json_data(json!({"text": text}))
                .unwrap())),
            _ => None,
        }
    });

    let outcome = stream::once(run).filter_map(move |result| {
        let span = span.clone();
        async move {
            let elapsed_ms = started.elapsed().as_millis() as u64;
            let event = match result {
                Ok(Ok(())) => {
                    info!(parent: &span, elapsed_ms, "Chat stream completed");
                    Event::default().event("done").data("{}")
                }
                Ok(Err(e)) => {
                    error!(parent: &span, elapsed_ms, error = %e, "Chat stream failed");
                    Event::default().event("error").data(e.to_string())
                }
                Err(e) => {
                    error!(parent: &span, elapsed_ms, error = %e, "Chat stream task panicked or was cancelled");
                    Event::default().event("error").data(e.to_string())
                }
            };
            Some(Ok(event))
        }
    });

    Sse::new(events.chain(outcome)).keep_alive(KeepAlive::default())
}