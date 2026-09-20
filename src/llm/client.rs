use crate::scanner::walker::CodeChunk;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

pub struct LlmClient {
    client: Client,
    base_url: String,
    model: String,
}

impl LlmClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }

    pub async fn ask(&self, query: &str, context_chunks: &[CodeChunk]) -> Result<String> {
        let mut context_text = String::new();
        for (idx, chunk) in context_chunks.iter().enumerate() {
            context_text.push_str(&format!(
                "--- Fragmento #{} [Archivo: {} | Líneas {}-{}] ---\n{}\n\n",
                idx + 1,
                chunk.file_path,
                chunk.start_line,
                chunk.end_line,
                chunk.content
            ));
        }

        let prompt = format!(
            "Eres RepoMind, un asistente experto en ingeniería de software. \
            Basándote ÚNICAMENTE en el siguiente contexto de código fuente, responde de manera clara, estructurada y precisa a la pregunta del usuario.\n\n\
            ### CONTEXTO DE CÓDIGO FUENTE:\n{}\n\n\
            ### PREGUNTA DEL USUARIO:\n{}\n\n\
            ### RESPUESTA TÉCNICA:",
            context_text, query
        );

        let url = format!("{}/api/generate", self.base_url);
        let payload = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let res = self.client.post(&url).json(&payload).send().await;

        match res {
            Ok(response) if response.status().is_success() => {
                let body: OllamaGenerateResponse = response.json().await?;
                Ok(body.response)
            }
            _ => {
                Ok(format!(
                    "💡 [Modo Demostración RepoMind]\n\n\
                    He localizado los fragmentos de código más relevantes en tu repositorio para responder a: \"{}\"\n\n\
                    📌 **Fragmentos clave encontrados:**\n{}\n\n\
                    ℹ️ *Tip: Para respuestas generadas por IA en tiempo real, ejecuta Ollama en segundo plano (`ollama run {}`) o usa Docker Compose (`docker compose up -d`).*",
                    query,
                    context_chunks.iter().map(|c| format!("- `{}` (Líneas {}-{})", c.file_path, c.start_line, c.end_line)).collect::<Vec<_>>().join("\n"),
                    self.model
                ))
            }
        }
    }
}
