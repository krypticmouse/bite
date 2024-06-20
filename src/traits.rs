use crate::embedder::models::EmbedderResponse;

pub trait Index {
    fn index(&self, data: &str);
    fn search(&self, index: usize) -> Option<&str>;
}

pub trait Embedder {
    async fn embed(&self, data: Vec<&str>) -> Result<EmbedderResponse, reqwest::Error>;
}