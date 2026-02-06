mod cache;
mod db;
mod display;
mod models;
mod mods;
mod morphology;
mod normalization;
mod repl;
mod search;

use anyhow::Result;
use clap::{Parser, Subcommand};

use search::{search_lexemes, SearchOptions};

#[derive(Parser)]
#[command(name = "judiw")]
#[command(about = "Judiw Yiddish Dictionary Terminal", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for lexemes by Hebrew or Latin text
    Lookup {
        /// Search query (Hebrew or Latin)
        query: String,
    },
    /// View detailed information about a lexeme
    View {
        /// Lexeme UUID
        id: String,
    },
    /// Search across all fields with fuzzy matching
    Search {
        /// Search query (Hebrew or Latin)
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value_t = 20)]
        limit: usize,

        /// Minimum relevance threshold (0.0-1.0)
        #[arg(short, long, default_value_t = 0.6)]
        threshold: f64,
    },
    /// Show statistics
    Stats,
    /// Start interactive shell (REPL mode)
    Shell,
    /// Query LLM (mods) with lexeme context
    Mods {
        /// Lexeme UUID
        id: String,

        /// Prompt template to use (analyze, examples, synonyms, etymology, custom)
        #[arg(short, long, default_value = "analyze")]
        template: String,

        /// Custom prompt (only used when template='custom')
        #[arg(long)]
        prompt: Option<String>,
    },
    /// Query LLM (mods) with raw text
    ModsRaw {
        /// Raw text to send to LLM
        text: String,
    },
    /// List available prompt templates
    ModsTemplates,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Connect to database
    println!("🔗 Connecting to Judiw database...");
    let db = db::Database::connect().await?;
    println!("✅ Connected!\n");

    match cli.command {
        Commands::Lookup { query } => {
            println!("🔍 Looking up: {}\n", query);

            match db.lookup_lexeme(&query).await {
                Ok(lexemes) => {
                    if lexemes.is_empty() {
                        println!("❌ No lexemes found.\n");
                    } else {
                        println!("✅ Found {} lexeme(s):\n", lexemes.len());

                        for (i, lexeme) in lexemes.iter().enumerate() {
                            println!(
                                "{}. {} → {}",
                                i + 1,
                                lexeme.canonical_hebrew,
                                lexeme.canonical_roman
                            );
                            println!("   POS: {}", lexeme.part_of_speech);
                            println!("   Origin: {}", lexeme.origin);
                            println!();
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Error: {}\n", e);
                }
            }
        }

        Commands::View { id } => {
            println!("📖 Viewing lexeme: {}\n", id);

            match uuid::Uuid::parse_str(&id) {
                Ok(lexeme_id) => match db.get_lexeme_details(lexeme_id).await {
                    Ok((lexeme, wordforms, senses)) => {
                        display::display_lexeme_full(&lexeme, &wordforms, &senses);
                    }
                    Err(e) => {
                        println!("❌ Error: {}\n", e);
                    }
                },
                Err(e) => {
                    println!("❌ Invalid UUID: {}\n", e);
                }
            }
        }

        Commands::Search {
            query,
            limit,
            threshold,
        } => {
            println!("🔎 Fuzzy search: {}\n", query);

            let options = SearchOptions {
                max_results: limit,
                fuzzy_threshold: threshold,
                ..Default::default()
            };

            match search_lexemes(&db, &query, &options).await {
                Ok(results) => {
                    if results.is_empty() {
                        println!("❌ No results found.\n");
                    } else {
                        println!("✅ Found {} result(s):\n", results.len());
                        println!("{}\n", search::format_search_results(&results));
                    }
                }
                Err(e) => {
                    println!("❌ Error: {}\n", e);
                }
            }
        }

        Commands::Stats => {
            display_stats(&db).await?;
        }

        Commands::Shell => {
            println!("🚀 Starting interactive shell...\n");
            let mut shell = repl::Repl::new(db)?;
            shell.run().await?;
            return Ok(());
        }

        Commands::Mods {
            id,
            template,
            prompt,
        } => {
            println!(
                "🤖 Querying LLM for lexeme: {} (template: {})\n",
                id, template
            );

            match uuid::Uuid::parse_str(&id) {
                Ok(lexeme_id) => match db.get_lexeme_details(lexeme_id).await {
                    Ok((lexeme, wordforms, senses)) => {
                        let context = mods::LexemeContext {
                            lexeme,
                            wordforms,
                            senses,
                        };

                        let client = mods::ModsClient::new();

                        let result = if template == "custom" {
                            if let Some(custom_prompt) = prompt {
                                client.query_lexeme_custom(&context, &custom_prompt)
                            } else {
                                eprintln!("❌ --prompt is required when using 'custom' template\n");
                                return Ok(());
                            }
                        } else {
                            client.query_lexeme(&context, &template)
                        };

                        match result {
                            Ok(response) => {
                                println!("═══════════════════════════════════════");
                                println!("LLM RESPONSE");
                                println!("═══════════════════════════════════════\n");
                                println!("{}\n", response);
                            }
                            Err(e) => {
                                eprintln!("❌ Error: {}\n", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error fetching lexeme: {}\n", e);
                    }
                },
                Err(e) => {
                    eprintln!("❌ Invalid UUID: {}\n", e);
                }
            }
        }

        Commands::ModsRaw { text } => {
            println!("🤖 Querying LLM with raw text...\n");

            let client = mods::ModsClient::new();
            match client.query_raw(&text) {
                Ok(response) => {
                    println!("═══════════════════════════════════════");
                    println!("LLM RESPONSE");
                    println!("═══════════════════════════════════════\n");
                    println!("{}\n", response);
                }
                Err(e) => {
                    eprintln!("❌ Error: {}\n", e);
                }
            }
        }

        Commands::ModsTemplates => {
            mods::list_templates();
            println!();
        }
    }

    Ok(())
}

async fn display_stats(db: &db::Database) -> Result<()> {
    println!("═══════════════════════════════════════");
    println!("JUDIW DATABASE STATISTICS");
    println!("═══════════════════════════════════════\n");

    let (lexeme_count, wordform_count, sense_count) = db.get_stats().await?;

    println!("📚 Lexemes: {}", lexeme_count);
    println!("📝 Wordforms: {}", wordform_count);
    println!("💡 Senses: {}", sense_count);

    println!();

    Ok(())
}
