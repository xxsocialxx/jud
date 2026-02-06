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
cargo run -- mods <uuid>        # LLM query with lexeme context

# Check compilation (faster than build)
cargo check

# Lint with Clippy
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test                                    # All tests
cargo test --test integration_test            # Integration tests only
cargo test --test property_tests              # Property-based tests
```

## Project Structure

```
jud/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library exports for testing
│   ├── models/           # Data models (domain-organized)
│   ├── db.rs             # PostgreSQL operations
│   ├── mods.rs           # LLM integration
│   ├── cache.rs          # LRU caching
│   ├── search.rs         # Fuzzy search
│   ├── normalization.rs  # Hebrew text normalization
│   ├── morphology.rs     # Morphological analysis
│   ├── display.rs        # Terminal formatting
│   ├── repl.rs           # Interactive REPL
│   ├── import.rs         # Data import utilities
│   ├── semantics.rs      # Semantic relationships
│   ├── query.rs          # Query building
│   ├── validation.rs     # Input validation
│   ├── error.rs          # Error types (miette)
│   └── observability.rs  # Logging (tracing)
├── tests/                # Integration & property tests
├── benches/              # Benchmark suites
├── scripts/              # Development scripts
├── docs/                 # Historical documentation (ADRs, reports)
├── migrations/           # Database migrations
├── CLAUDE.md             # This file
└── PROMPT.md             # Project philosophy & context
```

## Database Connection

The app connects to PostgreSQL using environment variables. Create a `.env` file:

```bash
DATABASE_URL=postgresql://601ere@localhost:5432/iddish
```

Default connection uses `NoTls` - modify `src/db.rs:33` if TLS is required.

## Architecture

**Module Structure:** See Project Structure above for complete file listing.

**Key Design Patterns:**
- **Library pattern:** `src/lib.rs` exposes modules for integration testing
- **Type-safe errors:** `thiserror` + `miette` for structured, pretty error printing
- **Caching layer:** LRU cache (1000 entries) in `src/cache.rs`
- **Query builder:** Compile-time SQL safety in `src/query.rs`
- **Structured logging:** `tracing` crate for observability

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

## Interactive REPL

Start with `cargo run -- shell` or `judiw shell` (if installed).

**REPL Commands:**
- `search <query>` / `s` - Fuzzy search
- `lookup <query>` / `l` - Direct lookup
- `view <uuid>` / `v` - View lexeme details
- `wordforms` / `wf` - Show wordforms for current lexeme
- `senses` - Show senses for current lexeme
- `back` / `b` - Navigate back
- `stats` - Database statistics
- `clear` - Clear screen
- `help` / `?` - Show help
- `exit` / `q` - Exit

**Features:**
- Command history saved to `~/.judiw_history`
- Auto-selection on single results
- Hierarchical navigation (lexeme → wordforms → senses)
- Context-sensitive help

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

## Testing

**Integration Tests** (`tests/integration_test.rs`):
- Database connection verification
- Lexeme lookup and retrieval
- Run with: `cargo test --test integration_test`

**Property-Based Tests** (`tests/property_tests.rs`):
- Uses `proptest` for invariant verification
- Morphological property testing
- Run with: `cargo test --test property_tests`

**All Tests:**
```bash
cargo test                                    # Run all tests
cargo test -- --nocapture                     # Show stdout
cargo test -- --test-threads=1                # Single-threaded
```

## Documentation

**Primary Docs:**
- `CLAUDE.md` - This file (AI agent guidance)
- `PROMPT.md` - Project philosophy, vision, requirements
- `docs/ADRs.md` - Architecture Decision Records
- `docs/INFRASTRUCTURE_COMPLETE.md` - Infrastructure status
- `docs/LINGUISTICS.md` - Linguistic features documentation

**See `docs/` folder for historical reports and session notes.**

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
