use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose::STANDARD};
use rig::agent::Agent;
use rig::completion::message::{ImageMediaType, UserContent};

pub fn is_image_extension(ext: &str) -> bool {
    matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp" | "gif")
}

fn media_type(ext: &str) -> Option<ImageMediaType> {
    match ext.to_lowercase().as_str() {
        "png" => Some(ImageMediaType::PNG),
        "jpg" | "jpeg" => Some(ImageMediaType::JPEG),
        "webp" => Some(ImageMediaType::WEBP),
        "gif" => Some(ImageMediaType::GIF),
        _ => None,
    }
}

pub async fn describe_image(agent: &Agent, bytes: &[u8], extension: &str) -> Result<String> {
    let media = media_type(extension)
        .ok_or_else(|| anyhow!("Unsupported image format : .{extension}"))?;

    let image = UserContent::image_base64(STANDARD.encode(bytes), Some(media), None);

    let response = agent.prompt(image).await?;
    let description = response.output;

    if description.trim().is_empty() {
        return Err(anyhow!("Description empty"));
    }
    Ok(description)
}