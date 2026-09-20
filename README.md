# 🧠 RepoMind

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust_🦀-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Container-Docker_🐳-blue.svg" alt="Docker">
  <img src="https://img.shields.io/badge/AI-RAG_%2B_Ollama-purple.svg" alt="AI">
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License">
</p>

> **RepoMind** es un asistente CLI de alto rendimiento escrito en **Rust** que indexa repositorios locales de código y responde preguntas sobre su arquitectura, funciones y flujos utilizando **Retrieval-Augmented Generation (RAG)** e Inteligencia Artificial 100% local o vía API.

---

```
==================================================
   🧠 REPOMIND - Asistente RAG para tu Código     
   Rendimiento extremo escrito en Rust 🦀         
==================================================
```

---

## ✨ Características Principales

* ⚡ **Ultra Rápido**: Compilado nativamente en Rust con tiempo de respuesta en milisegundos.
* 🔍 **Smart Code Chunking**: Escanea el repositorio respetando reglas de ignorado (`.git`, `node_modules`, `target`, etc.) y fragmenta el código de forma semántica.
* 🧠 **Búsqueda Vectorial**: Calcula la similitud de coseno (*cosine similarity*) entre las consultas del usuario y el código fuente.
* 🐳 **Docker-Native**: Incluye `Dockerfile` multi-etapa (< 20MB) y `docker-compose.yml` para desplegar el asistente + servidor **Ollama** de forma limpia.
* 💡 **Modo Demostración Integrado**: Funciona instantáneamente incluso si no tienes un servidor LLM activo, mostrando los fragmentos relevantes encontrados.

---

## 🚀 Inicio Rápido (Local)

### Requisitos Prácticos
- **Rust & Cargo** (v1.75+)
- *(Opcional)* **Ollama** instalado localmente (`ollama run nomic-embed-text` y `ollama run llama3.2`)

### 1. Clonar e Instalar
```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/repomind.git
cd repomind

# Compilar ejecutable
cargo build --release
```

### 2. Indexar tu Repositorio
Escanea e indexa el código fuente actual:
```bash
cargo run --release -- index
```

### 3. Hacer Preguntas sobre tu Código
```bash
cargo run --release -- ask "Explícame cómo funciona el proceso de almacenamiento vectorial"
```

---

## 🐳 Despliegue con Docker Compose

Puedes correr **RepoMind** junto con un servidor **Ollama** aislado en un solo comando:

```bash
# 1. Levantar la pila de contenedores
docker compose up -d

# 2. Descargar los modelos en Ollama
docker exec -it repomind-ollama ollama pull nomic-embed-text
docker exec -it repomind-ollama ollama pull llama3.2

# 3. Indexar y Consultar dentro del contenedor
docker exec -it repomind-app repomind index
docker exec -it repomind-app repomind ask "¿Cuál es la estructura del CLI?"
```

---

## 📁 Arquitectura del Proyecto

```
src/
├── main.rs                 # Punto de entrada y despacho de comandos CLI
├── cli/                    # Definición de argumentos y subcomandos con Clap
├── scanner/                # Escáner de archivos y fragmentador de sintaxis (Chunker)
├── embeddings/             # Cliente de vectores de embedding con fallback determinista
├── storage/                # Base de datos vectorial JSON con cálculo de Similitud Coseno
├── llm/                    # Cliente de comunicación con Ollama / APIs de IA
└── ui/                     # Formateador visual para consola (Spinners y banners)
```

---

## 📜 Licencia

Distribuido bajo la Licencia **MIT**. Consulta el archivo `LICENSE` para más detalles.
