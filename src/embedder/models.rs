use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct EmbedderPayload<'a> {
    pub input: Vec<&'a str>,
    pub model: &'a str,
    pub api_key: &'a str,
    pub dimension: Option<i32>,
    base_url: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingData {
    pub index: i32,
    pub embedding: Vec<f32>,
    pub object: String,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Deserialize, Debug)]
pub struct EmbedderResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: Usage,
}

impl<'a> EmbedderPayload<'a> {
    pub fn new(input: Vec<&'a str>, model: &'a str, api_key: &'a str, dimension: Option<i32>, base_url: Option<&'a str>) -> EmbedderPayload<'a> {
        EmbedderPayload {
            input,
            model,
            api_key,
            dimension: Some(dimension.unwrap_or(768)),
            base_url: Some(base_url.unwrap_or("https://api.openai.com/v1/embeddings")),
        }
    }
}
