use crate::index::models::Embedding;
use crate::embedder::models::EmbedderResponse;

pub trait Index {
    async fn index(&mut self, data: Vec<&str>);
    async fn search(&self, query: &str, top_k: i32, metric: &str) -> Vec<Embedding>;
}

pub trait Embedder {
    async fn embed(&self, data: &str) -> Result<EmbedderResponse, reqwest::Error>;
    async fn batch_embed(&self, data: Vec<&str>) -> Result<EmbedderResponse, reqwest::Error>;
}