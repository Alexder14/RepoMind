mod cli;
mod embeddings;
mod llm;
mod scanner;
mod storage;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::args::{Cli, Commands};
use embeddings::provider::EmbeddingProvider;
use llm::client::LlmClient;
use scanner::walker::RepoScanner;
use storage::vector_store::{VectorDocument, VectorStore};
use std::path::Path;
use ui::formatter::UiFormatter;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Index {
            path,
            ollama_url,
            embedding_model,
        } => {
            UiFormatter::print_banner();
            let repo_path = Path::new(&path);

            let pb = UiFormatter::create_spinner("Escaneando repositorio y fragmentando código...");
            let scanner = RepoScanner::new();
            let chunks = scanner.scan_repo(repo_path)?;
            pb.finish_with_message(format!(
                "✅ Se encontraron {} fragmentos de código.",
                chunks.len()
            ));

            let pb = UiFormatter::create_spinner("Generando vectores de embedding...");
            let embedder = EmbeddingProvider::new(ollama_url, embedding_model);
            let mut store = VectorStore::default();

            for chunk in chunks {
                let embedding = embedder.get_embedding(&chunk.content).await?;
                store.documents.push(VectorDocument { chunk, embedding });
            }

            let index_path = repo_path.join(".repomind").join("index.json");
            store.save_to_file(&index_path)?;

            pb.finish_with_message(format!(
                "🎉 Índice guardado exitosamente en: {}",
                index_path.to_string_lossy()
            ));
        }

        Commands::Ask {
            query,
            ollama_url,
            model,
            top_k,
        } => {
            UiFormatter::print_banner();

            let index_path = Path::new(".repomind").join("index.json");
            if !index_path.exists() {
                eprintln!(
                    "⚠️  No se encontró un índice de código (.repomind/index.json).\nEjecuta primero: `repomind index`"
                );
                return Ok(());
            }

            let store = VectorStore::load_from_file(&index_path)?;
            let embedder = EmbeddingProvider::new(ollama_url.clone(), "nomic-embed-text".into());

            let pb = UiFormatter::create_spinner("Buscando fragmentos de código relevantes...");
            let query_embedding = embedder.get_embedding(&query).await?;
            let relevant_chunks = store.search(&query_embedding, top_k);
            pb.finish_with_message("✅ Búsqueda vectorial completada.");

            UiFormatter::print_context_found(&relevant_chunks);

            let pb = UiFormatter::create_spinner("Generando respuesta con la IA...");
            let llm = LlmClient::new(ollama_url, model);
            let answer = llm.ask(&query, &relevant_chunks).await?;
            pb.finish_and_clear();

            UiFormatter::print_response(&answer);
        }

        Commands::Status => {
            UiFormatter::print_banner();
            let index_path = Path::new(".repomind").join("index.json");

            if index_path.exists() {
                let store = VectorStore::load_from_file(&index_path)?;
                println!(
                    "📊 Estado del Índice RepoMind:\n- Ubicación: {}\n- Fragmentos indexados: {}\n- Estado: Listo ✅",
                    index_path.to_string_lossy(),
                    store.documents.len()
                );
            } else {
                println!("⚠️  Estado: Sin indexar. Ejecuta `repomind index` para comenzar.");
            }
        }

        Commands::Docker => {
            UiFormatter::print_banner();
            println!(
                "🐳 Guía de Despliegue con Docker:\n\n\
                1. Levantar RepoMind + Ollama en segundo plano:\n   $ docker compose up -d\n\n\
                2. Descargar modelo de embeddings en Ollama:\n   $ docker exec -it repomind-ollama ollama pull nomic-embed-text\n\n\
                3. Descargar modelo de lenguaje (LLM):\n   $ docker exec -it repomind-ollama ollama pull llama3.2\n\n\
                4. Ejecutar RepoMind dentro del contenedor:\n   $ docker exec -it repomind-app repomind index\n   $ docker exec -it repomind-app repomind ask \"Explicame la estructura\""
            );
        }
    }

    Ok(())
}
