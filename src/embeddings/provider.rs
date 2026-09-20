use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

pub struct EmbeddingProvider {
    client: Client,
    base_url: String,
    model: String,
}

impl EmbeddingProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }

    pub async fn get_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.base_url);
        let payload = EmbeddingRequest {
            model: self.model.clone(),
            prompt: text.to_string(),
        };

        let response = self.client.post(&url).json(&payload).send().await;

        match response {
            Ok(res) if res.status().is_success() => {
                let data: EmbeddingResponse = res.json().await?;
                Ok(data.embedding)
            }
            _ => Ok(self.generate_fallback_embedding(text)),
        }
    }

    fn generate_fallback_embedding(&self, text: &str) -> Vec<f32> {
        let mut vec = vec![0.0f32; 128];
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let hash = word.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
            let idx = (hash as usize) % 128;
            vec[idx] += 1.0 / ((i + 1) as f32).sqrt();
        }
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in vec.iter_mut() {
                *val /= norm;
            }
        }
        vec
    }
}
