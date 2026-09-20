use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "repomind",
    author = "RepoMind Team",
    version = "0.1.0",
    about = "⚡ Asistente CLI de IA de alto rendimiento para tu código fuente",
    long_about = "RepoMind indexa repositorios locales de código y responde preguntas sobre arquitectura, errores y flujos utilizando RAG e Inteligencia Artificial."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Indexa el repositorio actual o una ruta especificada
    Index {
        /// Ruta al repositorio (por defecto es el directorio actual)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Dirección URL del servidor de Ollama
        #[arg(long, default_value = "http://localhost:11434")]
        ollama_url: String,

        /// Modelo de embeddings a utilizar
        #[arg(long, default_value = "nomic-embed-text")]
        embedding_model: String,
    },

    /// Realiza preguntas en lenguaje natural sobre el código indexado
    Ask {
        /// La pregunta sobre el código fuente
        query: String,

        /// Dirección URL del servidor de Ollama
        #[arg(long, default_value = "http://localhost:11434")]
        ollama_url: String,

        /// Modelo LLM para la respuesta
        #[arg(long, default_value = "llama3.2")]
        model: String,

        /// Número de bloques de código más relevantes a incluir en el contexto
        #[arg(short, long, default_value_t = 4)]
        top_k: usize,
    },

    /// Muestra el estado del índice vectorial actual
    Status,

    /// Muestra ayuda para desplegar con Docker Compose
    Docker,
}
