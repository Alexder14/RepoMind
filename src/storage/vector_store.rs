use crate::scanner::walker::CodeChunk;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct VectorDocument {
    pub chunk: CodeChunk,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct VectorStore {
    pub documents: Vec<VectorDocument>,
}

impl VectorStore {
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        let store: VectorStore = serde_json::from_str(&content)?;
        Ok(store)
    }

    pub fn search(&self, query_embedding: &[f32], top_k: usize) -> Vec<CodeChunk> {
        if self.documents.is_empty() {
            return Vec::new();
        }

        let mut scored_docs: Vec<(f32, &CodeChunk)> = self
            .documents
            .iter()
            .map(|doc| {
                let score = cosine_similarity(query_embedding, &doc.embedding);
                (score, &doc.chunk)
            })
            .collect();

        scored_docs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored_docs
            .into_iter()
            .take(top_k)
            .map(|(_, chunk)| chunk.clone())
            .collect()
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_vectors_cosine_similarity() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let score = cosine_similarity(&a, &b);
        assert!((score - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_orthogonal_vectors_cosine_similarity() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let score = cosine_similarity(&a, &b);
        assert!((score - 0.0).abs() < 1e-5);
    }
}

