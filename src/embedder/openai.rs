use reqwest;

use crate::traits::Embedder;
use super::models::{EmbedderPayload, EmbedderResponse};

pub struct OpenAI<'a> {
    pub model: &'a str,
    pub api_key: &'a str,
    pub dimension: Option<i32>,
    base_url: &'a str,
}

impl OpenAI<'_> {
    pub fn new<'a>(model: &'a str, api_key: &'a str, dimension: i32) -> OpenAI<'a> {
        OpenAI {
            model: model,
            api_key: api_key,
            dimension: Some(dimension),
            base_url: "https://api.openai.com/v1/embeddings",
        }
    }
}

impl Embedder for OpenAI<'_> {
    async fn embed(&self, data: Vec<&str>) -> Result<EmbedderResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let payload = EmbedderPayload::new(data, self.model, self.api_key, self.dimension, Some(self.base_url));

        let response = client.post(self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let embeddings: EmbedderResponse = response.json::<EmbedderResponse>().await?;

        Ok(embeddings)
    }
}