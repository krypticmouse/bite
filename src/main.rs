use bite::traits::Index;

use bite::index::knn::FlatIndex;
use bite::embedder::openai::OpenAI;

#[tokio::main]
async fn main() {
    let data: Vec<&str> = vec!["hello", "world"];

    let embedder = OpenAI::new(
        "text-embedding-ada-002",
        "secret-api-key",
        32,
    );

    let mut indexer = FlatIndex {
        embedder,
        embeddings: None,
    };

    indexer.index(data).await;

    let query = "world";
    let top_k = 1;
    let metric = "euclidean";

    let results = indexer.search(query, top_k, metric).await;
    println!("{:?}", results)
}
