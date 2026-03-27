# Shadow

**Your AI identity, observable and portable.**

Shadow is an open-source, local-first persona engine that builds a deep understanding of who you are through accumulated behavioral signals across AI agent interactions. It runs as a single binary on your machine and exposes an MCP interface for any AI agent (Claude Code, Codex, OpenClaw, etc.) to observe and recall.

The name comes from Jung: the shadow is the part of yourself you don't see. AI agents observe patterns about you — how you think, decide, react, contradict yourself — that you can't self-report. Shadow makes those observations visible and useful.

## The Idea

The AI industry has been climbing a cognitive stack:

```
┌─────────────────────────────────────────┐
│         Understanding Layer             │  ← Shadow. Unsolved.
│         who you are, how you think,     │     No one is building this
│         behavioral identity             │     as an open, portable layer.
├─────────────────────────────────────────┤
│         Memory Layer                    │  ← Being solved now.
│         context, persistence,           │     mem0, Letta, Claude memory,
│         retrieval                       │     ChatGPT memory.
├─────────────────────────────────────────┤
│         Logic Layer                     │  ← Solved.
│         reasoning, code, planning,      │     OpenAI o-series, Claude,
│         tool use                        │     Gemini, open-source LLMs.
└─────────────────────────────────────────┘
```

Memory systems store facts: "user prefers Rust," "user works at ByteDance." Shadow builds understanding: "user processes problems by building, not theorizing — will reject an abstract explanation but engage deeply if you frame it as something to construct. Meticulous about API boundaries but impatient with boilerplate. Competitive, even with tools."

The gap between "remembers your preferences" and "understands who you are" is the entire opportunity. Shadow fills that gap — not as a commercial product, but as open infrastructure anyone can use and improve.

## Principles

1. **Observable.** Everything Shadow knows is a markdown file you can read, edit, or delete. The engine is prompts, not hidden models. The psychiatrist's notes are open on the table.

2. **Portable.** Your identity is a directory of markdown files. Copy it to another machine, check it into git, sync it however you want. No accounts, no servers, no lock-in.

3. **Agent-agnostic.** Any AI agent that speaks MCP can connect. Shadow doesn't care if you use Claude Code, Codex, Cursor, or a custom agent. Your identity follows you across tools.

4. **Local-first.** The binary runs on your machine. Embeddings are computed locally. Nothing leaves your device unless you choose to push the git repo somewhere.

5. **Behavioral, not declarative.** Shadow cares about what you do, not what you say you are. Corrections, reactions, choices, contradictions — these are the real signals. Self-reports are performance.

6. **Contradictions are the point.** People are not consistent. Someone can be meticulous about architecture and sloppy about CSS. Patient when learning something new, impatient when re-explaining something old. Shadow captures the tensions, not a flattened average.

## Architecture

```
┌─────────────────────────────────────────┐
│  AI Agents                              │
│  Claude Code, Codex, Cursor, OpenClaw…  │
└──────────────┬──────────────────────────┘
               │ MCP (stdio / SSE)
┌──────────────▼──────────────────────────┐
│  shadow (single binary, Rust)           │  The engine
│                                         │
│  ├── MCP Server     (agent interface)   │
│  ├── File Watcher   (auto-reindex)      │
│  ├── Indexer        (chunk + embed)     │
│  ├── Search Engine  (hybrid retrieval)  │
│  └── Embeddings     (Ollama / remote)   │
│                                         │
└──────────────┬──────────────────────────┘
               │ reads/writes
┌──────────────▼──────────────────────────┐
│  ~/.shadow/ (source of truth)           │
│                                         │
│  ├── identity/     who you are          │
│  ├── disposition/  how you operate      │
│  ├── context/      what you're on now   │
│  ├── signal/       raw observations     │
│  ├── SHADOW.md     readable snapshot    │
│  └── .index/       sqlite + embeddings  │
│                    (gitignored)          │
└─────────────────────────────────────────┘
```

### The Binary

A single Rust executable. Everything statically linked — SQLite, sqlite-vec, all baked in. No runtime dependencies, no DLLs, no extensions to load. Download it, run `shadow serve`, point your agents at it.

- **MCP server** — stdio or SSE transport, for agents to connect
- **File watcher** — monitors `~/.shadow/` for changes, triggers reindex
- **Indexer** — markdown-aware chunking (respects frontmatter, splits on headers, keeps small files whole), generates embeddings, builds FTS index
- **Search engine** — hybrid retrieval combining vector similarity (semantic) and BM25 (keyword), with temporal decay and MMR diversity re-ranking
- **Embeddings** — calls Ollama for local embedding inference by default. No API keys required. Optional remote providers (OpenAI, Gemini) for higher quality or when Ollama isn't available.

Storage: SQLite with sqlite-vec for vectors, FTS5 for keyword search. Single file, lives in `~/.shadow/.index/`, gitignored as a derived artifact.

### The Data Layer

Plain markdown files with YAML frontmatter. Git-managed. This is the only thing that matters — everything else is derived.

```yaml
# ~/.shadow/identity/profile.md
---
type: identity
created: 2026-03-27
updated: 2026-03-27
---

Backend engineer turned founder turned indie dev. Builds to understand,
not the other way around. Deep in Go and Rust, new to frontend but
learning fast through construction. Competitive about craft. Values
density and precision over explanation.
```

```yaml
# ~/.shadow/disposition/working_style.md
---
type: disposition
created: 2026-03-27
updated: 2026-03-27
axes:
  - [meticulous, impatient]    # meticulous about APIs, impatient with boilerplate
  - [builder, delegator]       # prefers to own code, uses AI for acceleration
  - [minimal, defensive]       # strips error handling, trusts internal paths
---

Processes problems by building, not by discussing. Will reject abstract
explanations but engage deeply when framed as something to construct.

Contradiction: extremely patient when learning a new domain they respect
(spent weeks on foliate-js internals), but will rewrite your code from
scratch rather than ask for a second iteration.
```

```yaml
# ~/.shadow/signal/2026-03-27.md
---
type: signal
date: 2026-03-27
---

## 14:32 — behavioral/correction
User rejected suggested error handling for an internal function call.
Said "don't add validation for scenarios that can't happen." Pattern:
trusts internal code paths, only validates at system boundaries.
Source: claude-code session

## 15:10 — behavioral/cognitive
When discussing architecture, user immediately jumped to drawing the
system as boxes and arrows rather than describing it in words. Thinks
spatially. Prefers diagrams over prose for system design.
Source: claude-code session
```

## MCP Interface

The interface design is the engine. Tool descriptions are prompts that teach agents how to observe, and what to recall. The intelligence is in the LLM — Shadow just structures the observation process.

### Tools

#### `shadow_search`

```json
{
  "name": "shadow_search",
  "description": "Search the user's persona for relevant context. Call this:\n- At conversation start, to understand who you're talking to\n- When the user's reaction surprises you\n- When calibrating tone, depth, or approach\n- Before assuming what the user knows or wants\n\nReturns ranked memory fragments with relevance scores. Results include identity (who they are), dispositions (how they operate), context (what they're working on), and raw signals (recent observations).",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": {
        "type": "string",
        "description": "Natural language query. Be specific about what you need to know. 'how does this user learn new things' is better than 'user preferences'."
      },
      "filter": {
        "type": "string",
        "enum": ["identity", "disposition", "context", "signal"],
        "description": "Optional: narrow search to a specific memory type."
      },
      "limit": {
        "type": "integer",
        "default": 6,
        "description": "Max results to return."
      }
    },
    "required": ["query"]
  }
}
```

#### `shadow_observe`

```json
{
  "name": "shadow_observe",
  "description": "Record a behavioral observation about the user. You are a behavioral analyst — observe patterns, not facts.\n\nGood observations (patterns and signals):\n- 'User rewrote my implementation from scratch instead of iterating — prefers to own code, uses AI for acceleration not delegation'\n- 'User pushed back hard on adding error handling for internal paths — trusts system boundaries, values minimalism'\n- 'User responded in Mandarin when expressing frustration — language switching is emotional, not contextual'\n- 'User asked for the architectural diagram before reading any code — thinks spatially, processes top-down'\n\nBad observations (just facts, not signals):\n- 'User is working on a Rust project'\n- 'User asked me to fix a bug'\n- 'User's name is Jason'\n\nCapture the behavior AND your interpretation of what it reveals. The interpretation is the valuable part.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "observation": {
        "type": "string",
        "description": "The behavioral observation. Include what happened and what it suggests about how the user thinks, works, or decides."
      },
      "signal_type": {
        "type": "string",
        "enum": ["behavioral", "preference", "correction", "emotional", "cognitive"],
        "description": "behavioral: how they act. preference: what they choose. correction: when they push back. emotional: affect signals. cognitive: how they think/process."
      }
    },
    "required": ["observation", "signal_type"]
  }
}
```

#### `shadow_remember`

```json
{
  "name": "shadow_remember",
  "description": "Store a durable, curated memory. Unlike observations (raw signals), memories are interpreted understanding.\n\nUse this for:\n- identity: who the user is — background, role, how they see themselves\n- disposition: how they operate — the tensions and contradictions, not averages\n- context: what they're working on and WHY — motivations matter more than tasks\n\nA disposition memory should capture contradictions explicitly: 'meticulous about X but careless about Y' is more valuable than 'generally careful.'",
  "inputSchema": {
    "type": "object",
    "properties": {
      "content": {
        "type": "string",
        "description": "The memory content in markdown."
      },
      "type": {
        "type": "string",
        "enum": ["identity", "disposition", "context"],
        "description": "Memory category."
      },
      "path": {
        "type": "string",
        "description": "Optional: path to update an existing memory file (e.g., 'identity/profile.md'). If omitted, a new file is created with a generated name."
      }
    },
    "required": ["content", "type"]
  }
}
```

#### `shadow_reflect`

```json
{
  "name": "shadow_reflect",
  "description": "Trigger a reflection pass. Returns recent unprocessed signals and asks you to synthesize.\n\nDuring reflection, you are a psychiatrist reviewing session notes:\n- Look for patterns across multiple observations\n- Identify contradictions — these are the most valuable insights. People are not consistent, and the inconsistency IS the understanding.\n- Update existing disposition memories when new evidence refines them\n- Create new disposition memories when a pattern emerges that wasn't captured\n- Challenge your existing model — what did you get wrong?\n\nCall this periodically (every few sessions) or when you notice accumulated signals that haven't been synthesized.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "scope": {
        "type": "string",
        "enum": ["recent", "full"],
        "default": "recent",
        "description": "recent: last 7 days of signals. full: all unprocessed signals."
      }
    }
  }
}
```

### Resources

#### `shadow://snapshot`

Auto-generated persona snapshot, injected at session start. Gives any new agent session immediate context without searching.

```
# Shadow Snapshot

## Identity
Backend engineer (Go, Rust), indie dev building wyc studios.
Full-time at ByteDance (logistics settlement). Married, young son.

## How They Work
- Builds to understand. Rejects abstract explanations, engages
  when something is framed as construction.
- Meticulous about APIs and architecture. Impatient with boilerplate.
- Owns code — uses AI for acceleration, not delegation.
- Thinks spatially. Reaches for diagrams before prose.
- Competitive about craft. Measures against high standards.

## Current Context
- Building Shadow — open-source persona engine
- Quill ebook reader — pivoting toward AI reading companion
- Q2 2026 OKR at ByteDance: risk control, AI exploration,
  agent-driven dev workflow

## Active Tensions
- Builder vs. delegator: deep instinct to own everything, but
  learning to let AI carry more weight
- Precision vs. speed: will spend hours on the right abstraction,
  then ship something rough to test an idea
- Private vs. expressive: terse in conversation, deeply reflective
  in writing
```

## The Engine Is Prompts

Shadow's intelligence is not in the binary. The binary is plumbing — indexing, search, file management. The actual persona engine lives in two places:

### 1. MCP Tool Descriptions (above)

The tool descriptions teach agents HOW to observe. When Claude Code connects to Shadow and reads the `shadow_observe` tool schema, it learns what constitutes a meaningful behavioral signal vs. a useless fact. The tool description is the prompt. This is the psychiatrist training.

### 2. Reflection Prompts (returned by `shadow_reflect`)

When an agent calls `shadow_reflect`, the server returns the raw signals AND a synthesis prompt. The prompt guides the agent through consolidation — pattern detection, contradiction identification, disposition modeling. The LLM does the thinking. Shadow provides the structure.

This means:
- **The engine is transparent.** Every prompt is in the codebase. Users can read exactly how they're being profiled.
- **The engine is improvable.** Better prompts = better observations = better understanding. This is the open-source contribution surface.
- **The engine is LLM-agnostic.** Any model capable of following instructions can be the analyst. The prompts work with Claude, GPT, Gemini, open-source models.

## Search Architecture

### Indexing

- **Markdown-aware chunking**: respects YAML frontmatter (never splits it), splits on `##` headers, keeps files under 400 tokens as single chunks. A 15-line disposition memory stays whole.
- **Embedding**: Ollama with `nomic-embed-text` by default (768 dims, runs locally). Optional remote providers (OpenAI, Gemini) for higher quality.
- **FTS5**: full-text inverted index for keyword matching.
- **File watcher**: debounced (1.5s), triggers incremental reindex on file changes.

### Retrieval

Hybrid search combining two signals:

- **Vector search** (weight: 0.7) — cosine similarity on embeddings. Finds semantically relevant memories even when the words don't match.
- **BM25 keyword search** (weight: 0.3) — exact token matching via FTS5. Catches specific names, terms, identifiers that embeddings might miss.

Post-processing:
- **Temporal decay** — recent signals rank higher. Half-life: 30 days. Identity and disposition memories (curated, durable) are exempt from decay.
- **MMR re-ranking** — Maximal Marginal Relevance ensures result diversity. Avoids returning five variations of the same observation.

### Storage

SQLite with:
- `memories` table — file path, content hash, last modified, type
- `chunks` table — chunk text, embedding vector, source file, line range
- `chunks_vec` — sqlite-vec virtual table for vector search
- `chunks_fts` — FTS5 virtual table for keyword search
- `embedding_cache` — avoids re-embedding unchanged content

Single file at `~/.shadow/.index/shadow.db`. Gitignored. Fully derived — delete it and it rebuilds from the markdown files.

## Tech Stack

### Language & Runtime

**Rust** — single static binary with everything (SQLite, sqlite-vec, FTS5) statically linked. True cross-platform: one binary per target, no runtime dependencies, no DLLs or shared libraries to ship. Compiles to native code on macOS, Linux, and Windows.

### Core Dependencies

| Crate | Purpose | Why |
|-------|---------|-----|
| [`rmcp`](https://github.com/modelcontextprotocol/rust-sdk) | MCP server | Official Rust SDK. Proc-macro based — `#[tool]` attribute on async functions, JSON schema auto-generates via `schemars`. Tokio async runtime. Supports stdio transport. |
| [`rusqlite`](https://github.com/rusqlite/rusqlite) | SQLite database | Rust bindings for SQLite with bundled feature — statically links SQLite into the binary. Battle-tested, widely used. |
| [`sqlite-vec`](https://github.com/asg017/sqlite-vec) | Vector search | Statically linked as a SQLite extension via rusqlite. No separate DLL/so needed — this is the key reason for choosing Rust. |
| [`notify`](https://github.com/notify-rs/notify) | File watching | Cross-platform filesystem notifications (inotify/FSEvents/ReadDirectoryChangesW). |
| [`tokio`](https://github.com/tokio-rs/tokio) | Async runtime | Required by rmcp. Also drives the file watcher, HTTP client for Ollama, and concurrent indexing. |
| [`clap`](https://github.com/clap-rs/clap) | CLI parsing | Derive-based CLI argument parsing. |
| [`serde`](https://github.com/serde-rs/serde) / [`serde_yaml`](https://github.com/dtolnay/serde-yaml) | Serialization | YAML frontmatter parsing, JSON for MCP protocol. |

### Embedding Providers

| Provider | Role | Notes |
|----------|------|-------|
| **Ollama** (default) | Local embedding inference | HTTP API on localhost:11434. Model: `nomic-embed-text` (768 dims). No API keys. |
| **OpenAI** (optional) | Remote embeddings | `text-embedding-3-small`. Higher quality, requires API key. |
| **Gemini** (optional) | Remote embeddings | `gemini-embedding-001`. Alternative remote provider. |

Embedding provider is configurable. Ollama is the default because it requires no accounts or API keys — just `ollama pull nomic-embed-text` and go.

### MCP Server Pattern

The official Rust SDK uses proc macros. Tool handlers are async functions with `#[tool]` attributes. Input types derive `JsonSchema` for automatic schema generation:

```rust
use rmcp::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchInput {
    /// Natural language query. Be specific about what you need to know.
    pub query: String,
    /// Optional: narrow search to a specific memory type.
    pub filter: Option<MemoryType>,
    /// Max results to return.
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum MemoryType {
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "disposition")]
    Disposition,
    #[serde(rename = "context")]
    Context,
    #[serde(rename = "signal")]
    Signal,
}

#[derive(Clone)]
pub struct ShadowServer {
    store: Store,
    index: Index,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ShadowServer {
    #[tool(name = "shadow_search", description = "...")]  // prompt loaded from file
    async fn search(
        &self,
        Parameters(input): Parameters<SearchInput>,
    ) -> Result<ToolResult, McpError> {
        let results = self.index.search(&input.query, input.filter, input.limit).await?;
        Ok(ToolResult::from_text(serde_json::to_string_pretty(&results)?))
    }

    #[tool(name = "shadow_observe", description = "...")]
    async fn observe(
        &self,
        Parameters(input): Parameters<ObserveInput>,
    ) -> Result<ToolResult, McpError> {
        self.store.append_signal(&input.observation, &input.signal_type).await?;
        Ok(ToolResult::from_text("Observation recorded."))
    }

    #[tool(name = "shadow_remember", description = "...")]
    async fn remember(
        &self,
        Parameters(input): Parameters<RememberInput>,
    ) -> Result<ToolResult, McpError> {
        let path = self.store.write_memory(&input.content, &input.r#type, input.path.as_deref()).await?;
        Ok(ToolResult::from_text(format!("Memory saved to {path}")))
    }

    #[tool(name = "shadow_reflect", description = "...")]
    async fn reflect(
        &self,
        Parameters(input): Parameters<ReflectInput>,
    ) -> Result<ToolResult, McpError> {
        let signals = self.store.unprocessed_signals(input.scope).await?;
        let prompt = include_str!("../../prompts/reflect.md");
        Ok(ToolResult::from_text(format!("{prompt}\n\n---\n\n{signals}")))
    }
}
```

### Project Structure

```
shadow/
├── src/
│   ├── main.rs          # CLI entry point (clap)
│   ├── server.rs        # MCP server setup and tool handlers
│   ├── store.rs         # Markdown file read/write operations
│   ├── index.rs         # SQLite indexer, chunking, embedding
│   ├── search.rs        # Hybrid search engine (vector + BM25)
│   ├── embed/           # Embedding provider trait + implementations
│   │   ├── mod.rs
│   │   ├── ollama.rs
│   │   └── openai.rs
│   └── watch.rs         # File watcher with debounced reindex
├── prompts/             # Tool description prompts (the engine)
│   ├── search.md
│   ├── observe.md
│   ├── remember.md
│   └── reflect.md
├── docs/
│   └── arch/
│       └── product_spec_v0.md
├── Cargo.toml
├── Makefile
└── README.md
```

Key design choice: **prompts live in their own directory as markdown files**, not embedded in Rust string literals. This makes them readable, editable, and the primary contribution surface for the open-source community. The Rust binary includes them at compile time via `include_str!`.

## CLI

```bash
# Start the engine
shadow serve                    # MCP server (stdio)
shadow serve --sse              # MCP server (SSE, for remote agents)
shadow serve --port 3847        # Custom SSE port

# Manual operations
shadow search "how does the user learn"    # Search from terminal
shadow index                               # Force reindex
shadow index --rebuild                     # Full rebuild (re-embed everything)
shadow status                              # Index health, file counts, embedding model

# Data management
shadow init                     # Initialize ~/.shadow/ with directory structure
shadow snapshot                 # Regenerate SHADOW.md from current memories
shadow export --format json     # Export persona as structured JSON
```

## Roadmap

### v0.1 — Foundation
- [ ] MCP server via `rmcp` (official Rust SDK, stdio transport)
- [ ] Markdown file store with directory convention (`~/.shadow/`)
- [ ] SQLite indexer via `rusqlite` (bundled) + `sqlite-vec` (statically linked) + FTS5
- [ ] Ollama embedding integration (`nomic-embed-text`)
- [ ] Hybrid search (vector + BM25)
- [ ] Four MCP tools: search, observe, remember, reflect
- [ ] Prompt files in `prompts/` dir, embedded via `include_str!`
- [ ] File watcher via `notify` crate with debounced reindex
- [ ] CLI via `clap`: serve, search, index, init, status
- [ ] Cross-platform builds (macOS, Linux, Windows)
- [ ] Migrate existing ~/.claude/memory/ as seed data

### v0.2 — Search Refinement
- [ ] Temporal decay and MMR in search
- [ ] Snapshot auto-generation (SHADOW.md)
- [ ] Git integration (auto-commit on memory writes)
- [ ] SSE transport for MCP
- [ ] Reflection prompt refinement based on usage

### v0.3 — Ecosystem
- [ ] Export/import (JSON, structured formats)
- [ ] Prompt contribution framework (community observer prompts)
- [ ] Optional remote embedding providers (OpenAI, Gemini)
- [ ] Multi-directory support (separate persona per context)
- [ ] Plugin system for custom signal types

## Why Open Source

The thing that's usually proprietary in persona products is the inference model. Shadow has no proprietary model. The LLM does the inference. Shadow provides:

1. **Prompts** — the methodology for observation and profiling. Open-sourcing invites improvement, not competition.
2. **Plumbing** — indexing, search, file management. Commodity code.
3. **Data format** — markdown files. The most portable format possible.

The value is in YOUR accumulated persona, which stays on YOUR machine. Shadow is the tool that helps build it. The tool should be free. The identity is yours.

## License

MIT
