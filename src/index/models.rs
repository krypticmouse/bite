use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Embedding {
    pub id: Uuid,
    pub text: String,
    pub embedding: Vec<f32>,
}