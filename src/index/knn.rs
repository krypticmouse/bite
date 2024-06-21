use uuid::Uuid;

use crate::embedder::openai::OpenAI;
use crate::index::models::Embedding;
use crate::traits::{Embedder, Index};

pub struct FlatIndex {
    pub embedder: OpenAI<'static>,
    pub embeddings: Option<Vec<Embedding>>,
}

fn euclidean_distance(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let mut sum = 0.0;

    for i in 0..a.len() {
        sum += (a[i] - b[i]).powi(2);
    }

    sum.sqrt()
}

fn cosine_similarity(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i].powi(2);
        norm_b += b[i].powi(2);
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

impl Index for FlatIndex {
    async fn index(&mut self, data: Vec<&str>) {
        let mut embeddings = Vec::new();

        for d in data.iter() {
            let id = Uuid::new_v4();
            let embedding = match self.embedder.embed(d).await {
                Ok(response) => response.data[0].embedding.clone(),
                Err(e) => panic!("Error embedding data: {}", e),
            };

            let embed_data = Embedding {
                id,
                text: d.to_string(),
                embedding,
            };

            embeddings.push(embed_data);
        }
        
        self.embeddings = Some(embeddings);
        println!("Indexing complete");
    }

    async fn search(&self, query: &str, top_k: i32, metric: &str) -> Vec<Embedding> {
        let query_embedding = match self.embedder.embed(query).await {
            Ok(response) => response.data[0].embedding.clone(),
            Err(e) => panic!("Error embedding query: {}", e),
        };
        let mut results = Vec::new();
        
        for e in self.embeddings.as_ref().unwrap() {
            let distance = match metric {
                "euclidean" => euclidean_distance(query_embedding.clone(), e.embedding.clone()),
                "cosine" => cosine_similarity(query_embedding.clone(), e.embedding.clone()),
                _ => panic!("Invalid metric"),
            };

            results.push((e, distance));
        }

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results.into_iter()
            .take(top_k as usize)
            .map(|(e, _)| e.clone())
            .collect()
    }
}
