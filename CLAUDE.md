# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Judiw Terminal** is a Rust-based CLI tool for interacting with the Judiw Yiddish dictionary database. This is an independent project that connects directly to PostgreSQL (separate from the Django codebase).

**Project Location:** `/Users/601ere/jud/`
**Binary Name:** `judiw`

## Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run -- lookup <query>     # Search lexemes
cargo run -- view <uuid>        # View lexeme details

# Check compilation (faster than build)
cargo check

# Lint with Clippy
cargo clippy

# Format code
cargo fmt
```

## Database Connection

The app connects to PostgreSQL using environment variables. Create a `.env` file:

```bash
DATABASE_URL=postgresql://postgres@localhost:5432/iddish
```

Or set individual variables:
- `PGDATABASE=iddish`
- `PGUSER=postgres`
- `PGPASSWORD=your_password`
- `PGHOST=localhost`
- `PGPORT=5432`

## Architecture

**Core Components:**
- `src/main.rs` - CLI entry point using `clap` with subcommands
- `src/models.rs` - Data models (Lexeme, Wordform, Sense) and Database operations

**Key Tables:**
- `linguayi_lexeme` - Main dictionary entries (Hebrew, romanization, IPA)
- `linguayi_wordform` - Surface spellings and dialect variants
- `linguayi_sense` - Meanings and definitions
- `linguayi_token` - Corpus occurrences

**CLI Commands:**
- `lookup <query>` - Search by Hebrew or Latin text
- `view <uuid>` - View detailed lexeme information

## Project Philosophy

From `PROMPT.md` - This is a personal tool built with:
- **Sound architecture** - Modular, testable, idiomatic Rust
- **Start small, build slow** - Get core operations right first
- **CLI first** - No TUI yet (planned for later)
- **Single binary distribution** - Compiled once, works forever

## Dependencies

- `tokio-postgres` - PostgreSQL client with async support
- `tokio` - Async runtime
- `clap` - CLI argument parsing (derive feature)
- `serde`/`serde_json` - Serialization
- `anyhow` - Error handling
- `uuid` - UUID generation (v1 feature for postgres compatibility)
- `chrono` - Date/time handling
- `dotenv` - Environment variable loading

## Code Conventions

- Use `anyhow::Result` for error handling
- All database operations are async
- Mirror Django model structure in Rust structs
- Use `?` operator for error propagation
- Environment-based configuration (no hardcoded credentials)

## Known Issues

The project is in early development. Check compilation status with `cargo check` before starting work.
