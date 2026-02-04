mod models;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "judiw")]
#[command(about = "Judiw Yiddish Dictionary Terminal", long_about = None)]
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Connect to database
    println!("🔗 Connecting to Judiw database...");
    let db = models::Database::connect().await?;
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
                            println!("{}. {} → {}", 
                                i + 1,
                                lexeme.canonical_hebrew,
                                lexeme.canonical_roman.as_ref().unwrap_or(&"?".to_string())
                            );
                            println!("   Status: {}", lexeme.romanization_status);
                            println!("   Confidence: {:.2}", lexeme.romanization_confidence);
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
                Ok(lexeme_id) => {
                    match db.get_lexeme_details(lexeme_id).await {
                        Ok((lexeme, wordforms, senses)) => {
                            // Lexeme details
                            println!("═══════════════════════════════════════");
                            println!("LEXEME");
                            println!("═══════════════════════════════════════\n");
                            
                            println!("Hebrew:   {}", lexeme.canonical_hebrew);
                            println!("Latin:    {}", lexeme.canonical_roman.as_ref().unwrap_or(&"?".to_string()));
                            println!("IPA:      {}", lexeme.canonical_ipa.as_ref().unwrap_or(&"?".to_string()));
                            println!("Status:   {}", lexeme.romanization_status);
                            println!("Source:   {}", lexeme.romanization_source);
                            println!("POS:      {}", lexeme.part_of_speech.as_ref().unwrap_or(&"?".to_string()));
                            println!("Origin:   {}", lexeme.origin);
                            println!();
                            
                            if let Some(definition) = &lexeme.english_definition {
                                println!("DEFINITION: {}\n", definition);
                            }
                            
                            // Wordforms
                            if !wordforms.is_empty() {
                                println!("═══════════════════════════════════════");
                                println!("WORDFORMS ({})", wordforms.len());
                                println!("═══════════════════════════════════════\n");
                                
                                for (i, wf) in wordforms.iter().enumerate() {
                                    let standard = if wf.is_standard { "⭐" } else { "" };
                                    let canonical = if wf.is_canonical_lk { " [canonical LK]" } else { "" };
                                    
                                    println!("{}. {} {}{}", 
                                        i + 1,
                                        wf.text,
                                        standard,
                                        canonical
                                    );
                                    
                                    if let Some(dialect) = &wf.dialect {
                                        println!("   ({})", dialect);
                                    }
                                    println!();
                                }
                            }
                            
                            // Senses
                            if !senses.is_empty() {
                                println!("═══════════════════════════════════════");
                                println!("SENSES ({})", senses.len());
                                println!("═══════════════════════════════════════\n");
                                
                                for (i, sense) in senses.iter().enumerate() {
                                    println!("{}. {}", i + 1, sense.definition);
                                    
                                    if let Some(def_yi) = &sense.definition_yi {
                                        println!("   (יידיש: {})", def_yi);
                                    }
                                    println!("   [Flow: {}]", sense.flow_state);
                                    println!();
                                }
                            }
                        }
                        Err(e) => {
                            println!("❌ Error: {}\n", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Invalid UUID: {}\n", e);
                }
            }
        }
    }
    
    Ok(())
}