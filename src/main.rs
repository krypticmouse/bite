use bite::traits::Embedder;
use bite::embedder::openai::OpenAI;

#[tokio::main]
async fn main() {
    let data: Vec<&str> = vec!["hello", "world"];

    let embedder = OpenAI::new(
        "text-embedding-ada-002",
        "secret_key",
        32,
    );

    let result = embedder.embed(data).await.unwrap();
    println!("{:?}", result.data[0].embedding);
}
