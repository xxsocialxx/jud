// ============================================================================
// DISPLAY UTILITIES
// ============================================================================

use crate::models::{Lexeme, Sense, Wordform};

/// Display lexeme with full details
pub fn display_lexeme_full(lexeme: &Lexeme, wordforms: &[Wordform], senses: &[Sense]) {
    // Lexeme header
    println!("═══════════════════════════════════════");
    println!(
        "LEXEME: {} ({})",
        lexeme.canonical_hebrew, lexeme.canonical_roman
    );
    println!("═══════════════════════════════════════\n");

    println!("POS:      {}", lexeme.part_of_speech);
    println!("Status:   {}", lexeme.status);

    if let Some(ipa) = &lexeme.canonical_ipa {
        println!("IPA:      {}", ipa);
    }

    if let Some(definition) = &lexeme.english_definition {
        println!("\nDEFINITION: {}", definition);
    }

    println!();

    // Wordforms
    if !wordforms.is_empty() {
        println!("═══════════════════════════════════════");
        println!("WORDFORMS ({})", wordforms.len());
        println!("═══════════════════════════════════════\n");

        for (i, wf) in wordforms.iter().enumerate() {
            let standard = if wf.is_standard { "⭐" } else { "" };
            let canonical = if wf.is_canonical_lk { " [LK]" } else { "" };

            println!("{}. {}{}{}", i + 1, wf.text, standard, canonical);

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
            println!("   [Flow: {}]\n", sense.flow_state);
        }
    }
}
