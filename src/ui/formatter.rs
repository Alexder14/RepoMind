use crate::scanner::walker::CodeChunk;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct UiFormatter;

impl UiFormatter {
    pub fn create_spinner(msg: &'static str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(msg);
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    pub fn print_banner() {
        println!("{}", "==================================================".bright_cyan());
        println!("{}", "   🧠 REPOMIND - Asistente RAG para tu Código     ".bold().cyan());
        println!("{}", "   Rendimiento extremo escrito en Rust 🦀         ".dimmed());
        println!("{}\n", "==================================================".bright_cyan());
    }

    pub fn print_context_found(chunks: &[CodeChunk]) {
        println!("\n{}", "🔍 Contexto recuperado del código:".bold().yellow());
        for (idx, chunk) in chunks.iter().enumerate() {
            println!(
                "  {} [{}] Líneas {}-{}",
                format!("#{}", idx + 1).bold().cyan(),
                chunk.file_path.bright_white(),
                chunk.start_line,
                chunk.end_line
            );
        }
        println!();
    }

    pub fn print_response(response: &str) {
        println!("{}", "🤖 Respuesta de RepoMind:".bold().green());
        println!("{}\n", "--------------------------------------------------".green());
        println!("{}", response);
        println!("{}\n", "--------------------------------------------------".green());
    }
}
