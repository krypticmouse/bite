use crate::traits::Index;

struct FlatIndex {
    dimension: usize,
}

impl Index for FlatIndex {
    fn index(&self, data: &str) {
        println!("Indexing data: {}", data);
    }

    fn search(&self, index: usize) -> Option<&str> {
        println!("Searching index: {}", index);
        None
    }
}