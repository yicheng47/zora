# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Stack

- **Language:** Rust (edition 2024, requires rustc 1.85+)
- **Async runtime:** Tokio
- **MCP server:** rmcp 0.16 (official Rust SDK, proc-macro based)
- **Database:** SQLite via rusqlite (bundled, WAL mode) + sqlite-vec (vector search) + FTS5 (keyword search)
- **Embeddings:** Ollama (`nomic-embed-text`, 768 dims) default, FTS5-only fallback
- **CLI:** clap 4 (derive)
- **File watching:** notify + notify-debouncer-mini
- **Frontmatter:** gray_matter
- **Error handling:** thiserror per module, anyhow in main.rs

## Project Layout

```
src/
  main.rs              # CLI entry point (clap), wires components
  server.rs            # MCP server + 4 tool handlers (rmcp)
  store.rs             # Markdown file I/O for ~/.zora/
  index.rs             # SQLite indexer, chunking, embedding
  search.rs            # Hybrid search (vector + BM25)
  watch.rs             # File watcher with debounced reindex
  config.rs            # ZoraConfig, EmbeddingConfig, SearchConfig
  error.rs             # Per-module error types
  embed/
    mod.rs             # AnyEmbedder enum (sealed, not dyn Trait)
    ollama.rs          # Ollama HTTP client
prompts/               # MCP tool description prompts (the engine)
  search.md, observe.md, remember.md, reflect.md
docs/
  arch/                # Product spec
  features/            # Feature specs (product-level)
  impls/               # Implementation plans (code-level)
```

## Build & Dev

```bash
make build   # cargo build --release
make run     # cargo run -- serve
make test    # cargo test
make check   # cargo check
make lint    # cargo clippy -- -D warnings
make fmt     # cargo fmt
```

Run a single test: `cargo test <test_name>`

## Workflow

- **Planning:** For non-trivial features, write an implementation plan to `docs/impls/<feature-name>.md` before coding.
- **Feature specs** live in `docs/features/` — product-level, don't modify during implementation.
- **Prompts** in `prompts/` are the core intelligence — tool descriptions that teach agents how to observe. Treat changes to prompts as carefully as API changes.
- Run `cargo check` after dependency changes to sync `Cargo.lock`.

## Skills (slash commands)

- `/release` — Create a new release for Zora
- `/feature` — Create, list, or manage feature specs and GitHub issues

## Conventions

- Store operations (file I/O) are sync, wrapped in `spawn_blocking` at async boundaries.
- SQLite access goes through `Mutex<Connection>` + `spawn_blocking`. WAL mode enabled.
- Embedding provider is a sealed enum (`AnyEmbedder`), not `dyn Trait`. All vector operations are skipped when `AnyEmbedder::None`.
- MCP tool-originated writes (observe/remember) trigger direct incremental reindex, not through the file watcher.
- Tool descriptions are loaded from `prompts/*.md` via `include_str!` — keep them as separate files, not inline strings.
- Data directory is `~/.zora/` with subdirectories: `identity/`, `disposition/`, `context/`, `signal/`, `.index/` (gitignored).
