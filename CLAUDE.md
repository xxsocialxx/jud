# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Judiw Terminal** is a Rust-based CLI tool for interacting with the Judiw Yiddish dictionary database. This is an independent project that connects directly to PostgreSQL (separate from the Django codebase).

**Project Location:** `/Users/601ere/jud/`
**Binary Name:** `judiw`
**GitHub:** https://github.com/xxsocialxx/jud

## Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run -- lookup <query>     # Search lexemes
cargo run -- view <uuid>        # View lexeme details
cargo run -- search <query>     # Fuzzy search with options
cargo run -- stats              # Database statistics
cargo run -- shell              # Interactive REPL mode

# Check compilation (faster than build)
cargo check

# Lint with Clippy
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test
```

## Database Connection

The app connects to PostgreSQL using environment variables. Create a `.env` file:

```bash
DATABASE_URL=postgresql://601ere@localhost:5432/iddish
```

Default connection uses `NoTls` - modify `src/db.rs:33` if TLS is required.

## Architecture

**Module Structure:**
- `src/main.rs` - CLI entry point using `clap` with subcommands
- `src/models/` - Data models organized by domain (lexeme, wordform, sense, etymology, etc.)
- `src/db.rs` - PostgreSQL connection, queries, and caching layer
- `src/mods.rs` - LLM integration via `mods` CLI with prompt templates
- `src/search.rs` - Fuzzy search and result formatting
- `src/repl.rs` - Interactive shell (REPL) with rustyline
- `src/cache.rs` - LRU query caching
- `src/normalization.rs` - Unicode text normalization
- `src/morphology.rs` - Morphological analysis
- `src/display.rs` - Terminal output formatting
- `src/import.rs` - Data import utilities
- `src/semantics.rs` - Semantic relationships
- `src/error.rs` - Error types with miette
- `src/observability.rs` - Logging with tracing
- `src/validation.rs` - Input validation

**Key Tables:**
- `linguayi_lexeme` - Main dictionary entries (Hebrew, romanization, IPA)
- `linguayi_wordform` - Surface spellings and dialect variants
- `linguayi_sense` - Meanings and definitions
- `linguayi_token` - Corpus occurrences

**CLI Commands:**
- `lookup <query>` - Search by Hebrew or Latin text
- `view <uuid>` - View detailed lexeme information
- `search <query>` - Fuzzy search with `--limit` and `--threshold` options
- `stats` - Show database statistics
- `shell` - Start interactive REPL
- `mods <uuid>` - Query LLM with lexeme context (uses prompt templates)
- `mods-raw <text>` - Send raw text to LLM
- `mods-templates` - List available prompt templates

## Mods Integration (LLM)

**TODO: Configure LLM backend**

The app integrates with the `mods` CLI tool for LLM queries:
- Current setup: `mods` connected to OpenRouter
- To configure: Run `mods --help` to set API keys
- To install: `brew install mods` or from https://github.com/charmbracelet/mods

**Built-in Prompt Templates:**
- `analyze` - Full linguistic analysis (etymology, usage, notes)
- `examples` - Generate Yiddish usage examples with translations
- `synonyms` - Find synonyms and related words
- `etymology` - Detailed etymological analysis with cognates
- `custom` - Use your own prompt with `--prompt` flag

**Usage Examples:**
```bash
# Analyze a lexeme with LLM
cargo run -- mods <uuid> --template analyze

# Get usage examples
cargo run -- mods <uuid> --template examples

# Custom prompt with full context
cargo run -- mods <uuid> --template custom --prompt "Explain the grammar of {{HEBREW}}"

# Raw LLM query
cargo run -- mods-raw "What is Yiddish?"

# List templates
cargo run -- mods-templates
```

**Context Gathering:**
The mods integration automatically includes:
- Lexeme data (Hebrew, romanization, IPA, POS, origin, definition)
- All linked wordforms (spellings, dialect variants)
- All senses (meanings)

**Template Variables:**
- `{{HEBREW}}` - Canonical Hebrew text
- `{{ROMAN}}` - Romanization
- `{{IPA}}` - IPA transcription
- `{{POS}}` - Part of speech
- `{{ORIGIN}}` - Etymological origin
- `{{DEFINITION}}` - English definition
- `{{WORDFORMS}}` - All wordforms
- `{{SENSES}}` - All senses
- `{{CUSTOM}}` - For custom templates

**File:** `src/mods.rs` - Prompts are currently hardcoded; future plan is config file for runtime tweaking.

## Project Philosophy

From `PROMPT.md` - This is a personal tool built with:
- **Sound architecture** - Modular, testable, idiomatic Rust
- **Start small, build slow** - Get core operations right first
- **CLI first, REPL added** - Interactive mode via rustyline
- **Single binary distribution** - Compiled once, works forever
- **Caching** - LRU cache for query results (1000 entry capacity)

## Dependencies

- `tokio-postgres` - PostgreSQL client with async support
- `tokio` - Async runtime (full features)
- `clap` - CLI argument parsing (derive feature)
- `serde`/`serde_json` - Serialization
- `anyhow` - Error handling
- `thiserror` - Error derive macros
- `miette` - Fancy error reports
- `uuid` - UUID generation (v1 feature for postgres compatibility)
- `chrono` - Date/time handling
- `rustyline` - REPL readline support
- `lru` - LRU cache implementation
- `tracing`/`tracing-subscriber` - Structured logging
- `unicode-normalization` - Text normalization

## Code Conventions

- Use `anyhow::Result` for application errors, `thiserror` for library errors
- All database operations are async
- Models organized in `src/models/` module with re-exports in `mod.rs`
- Use `?` operator for error propagation
- Environment-based configuration (no hardcoded credentials)
- Use `tracing::info!`, `tracing::error!` for logging
- Connection spawns background task for PostgreSQL connection manager

## Release Build

Optimized release profile configured in `Cargo.toml`:
- LTO enabled, single codegen unit
- Binary stripped, panic=abort
- Maximum optimization level
