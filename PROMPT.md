# JUDIW TERMINAL APP - RUST

**Project Philosophy:**

I want to build a Rust-based terminal application for interacting with my Judiw Yiddish dictionary database. This is a personal tool - just for me - for dictionary-related operations: lexeme lookup, editing, making connections between wordforms/senses/lexemes. It's very text-based, and a terminal interaction just feels right.

**My Rust Obsession:**

I have a soft spot for Rust despite never having written a line of code. Why? Because of what it represents:
- **Philosophy** - Safety without garbage collection, performance without compromise
- **Aesthetics** - `cargo build`, single binary distribution, the compiler as quality control
- **Stability** - Binary compiled once, works forever, no dependency hell, no "Python 2 → Python 3" breakage
- **Craftsmanship** - Feels engineered, not thrown together
- **Community** - Welcoming, documentation-focused, helpful

As a "vibe developer" who relies on agents to write code, I appreciate Rust's strict compiler - it catches mistakes agents make. Python might crash at runtime; Rust compiler yells until it's right.

**Inspiration: French CLI**

You built me an incredible French learning CLI (`/Users/601ere/French/f`) that showed me:
- Minimal, terminal-based interfaces can be beautiful
- Text-based learning is effective
- Compiled binaries just work - no dependencies
- Sound architecture beats fast iteration
- Personality in code (street mode, nerd mode) makes tools memorable

I want that same aesthetic for Judiw.

**Judiw Context:**

Judiw is a Django-based Yiddish linguistic engine with a 4-pillar architecture:
1. **Lexeme** - Abstract word concept (57K+ entries)
2. **Wordform** - Surface spellings (dialect variants)
3. **Sense** - Meanings with definitions
4. **Token** - Corpus occurrences/provenance

**Database:** PostgreSQL with complex schema:
- `linguayi_lexeme` - Main lexeme table with Hebrew, romanization, IPA, vectors
- `linguayi_wordform` - Specific spellings, dialect variants
- `linguayi_sense` - Meanings
- `linguayi_token` - Corpus evidence
- Plus: Source, RomanizationHistory, LearningSequence, etc.

**Current State:** Early development, basic operations, but growing fast.

**My Vision:**

**Start Small, Build Slow:**
- Don't worry about deliverables yet
- Focus on sound architecture
- CLI first (no TUI yet)
- Get the core right, expand later

**Option 1 Architecture:**
Rust terminal app → Direct PostgreSQL connection → Query tables → Display results

**Why This Works:**
- Fast, direct, no Django overhead
- Compiled binary runs anywhere
- Single file distribution
- Offline capable (cached data)
- Feels like a serious tool

**Primary Operations (Phase 1):**
1. **Lexeme Lookup** - Search by Hebrew/Latin, view details
2. **Edit Romanization** - Update canonical_roman, canonical_ipa
3. **View Connections** - See wordforms/senses for a lexeme
4. **Status Reports** - Quality, confidence, origin breakdowns

**Not Building Yet:**
- TUI (keep it CLI for now)
- Authentication (just me, database credentials)
- Complex visualizations (text-based first)
- Bulk operations (get core operations right first)

**Technical Requirements:**

**Database Connection:**
- PostgreSQL (same as Django)
- Credentials from environment variables (PGDATABASE, PGUSER, PGPASSWORD, PGHOST, PGPORT)
- Connection pooling for efficiency

**Data Models (Rust structs):**
- Mirror Django models: Lexeme, Wordform, Sense
- Use `serde` for JSON parsing
- Use `tokio-postgres` for database
- Use `sqlx` or write raw SQL (your call)

**User Interface:**
- Terminal CLI (ratatui comes later)
- Simple commands: `lookup`, `edit`, `view`, `report`
- Text-based output (tables, lists, details)
- Fast, responsive, feels like a tool

**Sound Architecture Principles:**
1. **Modular** - Easy to add new operations
2. **Testable** - Unit tests for database operations
3. **Configurable** - Environment variables, config file
4. **Error Handling** - Graceful failures, clear messages
5. **Documentation** - Code comments, usage examples
6. **Idiomatic Rust** - Follow Rust conventions (Result, Option, etc.)

**My Enthusiasm Level:**

I'm excited about this because:
- It's my personal tool for a project I care about deeply
- Rust feels like the future - compiled, safe, stable
- Terminal interaction is elegant and fast
- Building slow with sound architecture means it lasts
- French CLI showed me minimal tools can be powerful
- This is my chance to have a serious, professional-grade linguistic tool

**What I Need From You:**

1. **Start with CLI foundation** - Basic PostgreSQL connection, simple lookup
2. **Build incrementally** - Add operations as we go, not everything at once
3. **Reference French CLI aesthetics** - Minimal, clean, effective
4. **Embrace Rust philosophy** - Safety, performance, single binary distribution
5. **Plan for expansion** - CLI → TUI later, but architect for it
6. **Teach me** - Explain choices so I understand the Rust way

**Let's Build Something Great Together.**

Start simple. Get the database connection working. Build one operation that works perfectly. Expand from there.

This is the beginning of my personal Yiddish dictionary terminal toolkit. Let's make it beautiful.

---

## PROJECT LOCATION

**Directory:** `/Users/601ere/jud/`

**This is an independent Rust project** - separate from the Django Judiw codebase. It connects directly to the PostgreSQL database but has its own architecture, build system, and binary distribution.

**Why separate?**
- Sound architecture: Independent tool, not coupled to Django
- Easy to distribute: Single binary, no Django dependencies
- Clean separation: Rust code lives with Rust code
- Future flexibility: Can grow into its own ecosystem

---

## CREDENTIALS REFERENCE

**Database Connection Info:**
```bash
PGDATABASE=iddish
PGUSER=postgres
PGPASSWORD=[from environment]
PGHOST=localhost
PGPORT=5432
```

**Key Tables:**
```sql
linguayi_lexeme       -- Main lexeme table
linguayi_wordform     -- Spellings/variants
linguayi_sense         -- Meanings/definitions
linguayi_token        -- Corpus occurrences
linguayi_source       -- Bibliographic sources
```

---

## CURRENT STATE

**Fresh slate - ready to build:**
- Rust 1.93.0 installed ✅
- Project initialized ✅
- Dependencies configured ✅ (tokio-postgres, uuid, chrono, clap, serde)
- Data models defined ✅ (Lexeme, Wordform, Sense)
- Database connection module ✅
- CLI structure with 2 commands ✅ (lookup, view)
- `.env` file for credentials ✅

**Next step:** `cargo build` → compile the binary

---

Let's build this. Slow. Right. Together.