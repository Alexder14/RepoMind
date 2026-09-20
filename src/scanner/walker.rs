use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChunk {
    pub id: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
}

pub struct RepoScanner {
    ignored_dirs: Vec<String>,
    valid_extensions: Vec<String>,
}

impl RepoScanner {
    pub fn new() -> Self {
        Self {
            ignored_dirs: vec![
                ".git".into(),
                "node_modules".into(),
                "target".into(),
                "dist".into(),
                "build".into(),
                ".repomind".into(),
                "vendor".into(),
            ],
            valid_extensions: vec![
                "rs".into(),
                "go".into(),
                "py".into(),
                "js".into(),
                "ts".into(),
                "tsx".into(),
                "jsx".into(),
                "html".into(),
                "css".into(),
                "json".into(),
                "toml".into(),
                "yaml".into(),
                "yml".into(),
                "md".into(),
                "dockerfile".into(),
                "sh".into(),
                "c".into(),
                "cpp".into(),
                "h".into(),
                "java".into(),
            ],
        }
    }

    pub fn scan_repo(&self, repo_path: &Path) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();

        for entry in WalkDir::new(repo_path)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e.path()))
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if self.valid_extensions.contains(&ext_str) {
                        if let Ok(file_chunks) = self.chunk_file(entry.path(), repo_path) {
                            chunks.extend(file_chunks);
                        }
                    }
                }
            }
        }

        Ok(chunks)
    }

    fn is_ignored(&self, path: &Path) -> bool {
        path.components().any(|comp| {
            let name = comp.as_os_str().to_string_lossy();
            self.ignored_dirs.iter().any(|ignored| name == *ignored)
        })
    }

    fn chunk_file(&self, file_path: &Path, root_path: &Path) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() {
            return Ok(Vec::new());
        }

        let rel_path = file_path
            .strip_prefix(root_path)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        let chunk_size = 40;
        let overlap = 10;
        let mut chunks = Vec::new();

        let mut i = 0;
        let mut chunk_id = 0;
        while i < lines.len() {
            let end = (i + chunk_size).min(lines.len());
            let chunk_lines = &lines[i..end];
            let chunk_text = chunk_lines.join("\n");

            if !chunk_text.trim().is_empty() {
                chunks.push(CodeChunk {
                    id: format!("{}:{}", rel_path, chunk_id),
                    file_path: rel_path.clone(),
                    start_line: i + 1,
                    end_line: end,
                    content: chunk_text,
                });
                chunk_id += 1;
            }

            if end == lines.len() {
                break;
            }
            i += chunk_size - overlap;
        }

        Ok(chunks)
    }
}
