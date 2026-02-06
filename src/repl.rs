// ============================================================================
// INTERACTIVE SHELL (REPL)
// ============================================================================

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::Arc;

/// REPL shell for interactive dictionary exploration
pub struct Repl {
    editor: DefaultEditor,
    db: Arc<crate::db::Database>,
    current_lexeme: Option<uuid::Uuid>,
    current_sense: Option<i64>,
    history_path: Option<std::path::PathBuf>,
}

impl Repl {
    pub fn new(db: crate::db::Database) -> Result<Self> {
        let mut editor = DefaultEditor::new()?;

        // Set up history file
        let history_path = dirs::home_dir().map(|p| p.join(".judiw_history"));

        if let Some(ref path) = history_path {
            let _ = editor.load_history(path);
        }

        Ok(Self {
            editor,
            db: Arc::new(db),
            current_lexeme: None,
            current_sense: None,
            history_path,
        })
    }

    /// Run the REPL loop
    pub async fn run(&mut self) -> Result<()> {
        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║           Judiw Yiddish Dictionary - Interactive Shell        ║");
        println!("╚═══════════════════════════════════════════════════════════════╝");
        println!();
        println!("Type 'help' for commands or 'exit' to quit.");
        println!();

        loop {
            let prompt = self.build_prompt();
            match self.editor.readline(&prompt) {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Save to history
                    let _ = self.editor.add_history_entry(line);

                    // Parse and execute command
                    if let Err(e) = self.execute_command(line).await {
                        println!("❌ Error: {}\n", e);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("Goodbye!");
                    break;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }

        // Save history
        if let Some(ref path) = self.history_path {
            let _ = self.editor.save_history(path);
        }

        Ok(())
    }

    fn build_prompt(&self) -> String {
        if let Some(lexeme_id) = self.current_lexeme {
            format!("judiw:{}> ", lexeme_id)
        } else {
            "judiw> ".to_string()
        }
    }

    async fn execute_command(&mut self, input: &str) -> Result<()> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match command.as_str() {
            "cache" => {
                if !args.is_empty() && args[0] == "clear" {
                    self.db.cache.clear();
                    println!("✅ Cache cleared\n");
                } else {
                    self.cmd_cache();
                }
            }
            "help" | "?" | "h" => self.show_help(),
            "exit" | "quit" | "q" => {
                println!("Goodbye!");
                std::process::exit(0);
            }
            "search" | "s" => self.cmd_search(args).await?,
            "lookup" | "l" => self.cmd_lookup(args).await?,
            "view" | "v" => self.cmd_view(args).await?,
            "stats" => self.cmd_stats().await?,
            "wordforms" | "wf" => self.cmd_wordforms().await?,
            "senses" => self.cmd_senses().await?,
            "back" | "b" => self.cmd_back(),
            "clear" | "cls" => {
                print!("\x1b[2J\x1b[H");
                println!();
            }
            _ => {
                // Try as implicit search
                self.cmd_search(&[input]).await?;
            }
        }

        Ok(())
    }

    fn show_help(&self) {
        println!("╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║                              COMMANDS                                   ║");
        println!("╠═══════════════════════════════════════════════════════════════════════╣");
        println!("║ search <query>  | s  - Fuzzy search across all fields                ║");
        println!("║ lookup <query>  | l  - Direct lexeme lookup                          ║");
        println!("║ view <uuid>     | v  - View lexeme details                           ║");
        println!("║ wordforms       | wf - Show wordforms of current lexeme              ║");
        println!("║ senses          |   - Show senses of current lexeme                  ║");
        println!("║ back            | b  - Navigate back in hierarchy                    ║");
        println!("║ stats           |   - Show database statistics                      ║");
        println!("║ clear           |   - Clear screen                                   ║");
        println!("║ help            | ?  - Show this help                                ║");
        println!("║ exit            | q  - Exit shell                                    ║");
        println!("╚═══════════════════════════════════════════════════════════════════════╝");
        println!();
        println!("💡 Tip: You can also type any word to search for it directly!");
        println!();
    }

    async fn cmd_search(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: search <query>\n");
            return Ok(());
        }

        let query = args.join(" ");
        println!("🔎 Searching for: {}\n", query);

        let options = crate::search::SearchOptions {
            max_results: 10,
            ..Default::default()
        };

        match crate::search::search_lexemes(&self.db, &query, &options).await {
            Ok(results) => {
                if results.is_empty() {
                    println!("❌ No results found.\n");
                } else {
                    println!("{}\n", crate::search::format_search_results(&results));

                    // If we got exactly one result, automatically select it
                    if results.len() == 1 {
                        println!("💡 Auto-selected result (only match).");
                    }
                }
            }
            Err(e) => {
                println!("❌ Search error: {}\n", e);
            }
        }

        Ok(())
    }

    async fn cmd_lookup(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: lookup <query>\n");
            return Ok(());
        }

        let query = args[0];
        println!("🔍 Looking up: {}\n", query);

        match self.db.lookup_lexeme(query).await {
            Ok(lexemes) => {
                if lexemes.is_empty() {
                    println!("❌ No lexemes found.\n");
                } else {
                    println!("✅ Found {} lexeme(s):\n", lexemes.len());
                    for (i, lexeme) in lexemes.iter().enumerate() {
                        println!(
                            "{}. {} ({})",
                            i + 1,
                            lexeme.canonical_hebrew,
                            lexeme.canonical_roman
                        );
                        if let Some(def) = &lexeme.english_definition {
                            println!("   {}\n", def);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Error: {}\n", e);
            }
        }

        Ok(())
    }

    async fn cmd_view(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: view <uuid>\n");
            return Ok(());
        }

        let id_str = args[0];
        match uuid::Uuid::parse_str(id_str) {
            Ok(lexeme_id) => match self.db.get_lexeme_details(lexeme_id).await {
                Ok((lexeme, wordforms, senses)) => {
                    crate::display::display_lexeme_full(&lexeme, &wordforms, &senses);
                    self.current_lexeme = Some(lexeme_id);
                    println!("\n💡 Now viewing: {}", lexeme.canonical_hebrew);
                    println!("   Type 'wordforms', 'senses', or 'back'\n");
                }
                Err(e) => {
                    println!("❌ Error: {}\n", e);
                }
            },
            Err(e) => {
                println!("❌ Invalid UUID: {}\n", e);
            }
        }

        Ok(())
    }

    async fn cmd_stats(&self) -> Result<()> {
        let (lexemes, wordforms, senses) = self.db.get_stats().await?;
        let cache_info = self.db.cache.format_stats();

        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║                    DATABASE STATISTICS                       ║");
        println!("╠═══════════════════════════════════════════════════════════════╣");
        println!(
            "║  📚 Lexemes:    {:>8}                                      ║",
            lexemes
        );
        println!(
            "║  📝 Wordforms:  {:>8}                                      ║",
            wordforms
        );
        println!(
            "║  💡 Senses:     {:>8}                                      ║",
            senses
        );
        println!("╠═══════════════════════════════════════════════════════════════╣");
        println!("║  {} ║", cache_info);
        println!("╚═══════════════════════════════════════════════════════════════╝\n");

        Ok(())
    }

    async fn cmd_wordforms(&self) -> Result<()> {
        if let Some(lexeme_id) = self.current_lexeme {
            match self.db.get_lexeme_details(lexeme_id).await {
                Ok((_, wordforms, _)) => {
                    if wordforms.is_empty() {
                        println!("No wordforms found.\n");
                    } else {
                        println!("═══════════════════════════════════════");
                        println!("WORDFORMS ({})", wordforms.len());
                        println!("═══════════════════════════════════════\n");

                        for (i, wf) in wordforms.iter().enumerate() {
                            let standard = if wf.is_standard { "⭐" } else { "" };
                            println!("{}. {} {}", i + 1, wf.text, standard);
                        }
                        println!();
                    }
                }
                Err(e) => {
                    println!("❌ Error: {}\n", e);
                }
            }
        } else {
            println!("❌ No lexeme selected. Use 'view <uuid>' first.\n");
        }

        Ok(())
    }

    async fn cmd_senses(&self) -> Result<()> {
        if let Some(lexeme_id) = self.current_lexeme {
            match self.db.get_lexeme_details(lexeme_id).await {
                Ok((_, _, senses)) => {
                    if senses.is_empty() {
                        println!("No senses found.\n");
                    } else {
                        println!("═══════════════════════════════════════");
                        println!("SENSES ({})", senses.len());
                        println!("═══════════════════════════════════════\n");

                        for (i, sense) in senses.iter().enumerate() {
                            println!("{}. {}", i + 1, sense.definition);
                            if let Some(def_yi) = &sense.definition_yi {
                                println!("   (יידיש: {})", def_yi);
                            }
                            println!();
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Error: {}\n", e);
                }
            }
        } else {
            println!("❌ No lexeme selected. Use 'view <uuid>' first.\n");
        }

        Ok(())
    }

    fn cmd_back(&mut self) {
        if self.current_lexeme.is_some() {
            self.current_lexeme = None;
            self.current_sense = None;
            println!("↩ Back to top level\n");
        } else {
            println!("Already at top level\n");
        }
    }

    fn cmd_cache(&self) {
        let stats = self.db.cache.stats();
        let hit_rate = self.db.cache.hit_rate();

        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║                      CACHE STATUS                            ║");
        println!("╠═══════════════════════════════════════════════════════════════╣");
        println!(
            "║  Entries:    {:>8}                                        ║",
            stats.size
        );
        println!(
            "║  Hits:       {:>8}                                        ║",
            stats.hits
        );
        println!(
            "║  Misses:     {:>8}                                        ║",
            stats.misses
        );
        println!(
            "║  Hit Rate:   {:>7.1}%                                        ║",
            hit_rate * 100.0
        );
        println!("╠═══════════════════════════════════════════════════════════════╣");
        println!("║  Commands: clear | stats                                     ║");
        println!("╚═══════════════════════════════════════════════════════════════╝\n");

        // Handle cache subcommands
        println!("💡 Type 'cache clear' to clear cache\n");
    }
}
