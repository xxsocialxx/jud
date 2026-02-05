// ============================================================================
// MODS INTEGRATION - LLM CONTEXT-AWARE QUERIES
// ============================================================================
//
// TODO: Configure LLM backend (currently uses local 'mods' CLI with OpenRouter)
// - Provider: OpenRouter (https://openrouter.ai/)
// - CLI: 'mods' by charmbracelet
// - Configuration: Run 'mods --help' to set up your API key
//
// PROMPT SYSTEM:
// - Templates are defined below and can be customized
// - Context is automatically gathered from lexeme/wordform data
// - Future: Move prompts to config file for runtime tweaking
// ============================================================================

use anyhow::{Result, anyhow};
use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::models::{Lexeme, Wordform, Sense};

// ============================================================================
// PROMPT TEMPLATES
// ============================================================================

/// Prompt templates for different LLM interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
}

impl PromptTemplate {
    /// Format the template with lexeme context
    pub fn format(&self, context: &LexemeContext) -> String {
        self.template
            .replace("{{HEBREW}}", &context.lexeme.canonical_hebrew)
            .replace("{{ROMAN}}", &context.lexeme.canonical_roman)
            .replace("{{IPA}}", &context.lexeme.canonical_ipa.as_deref().unwrap_or("N/A"))
            .replace("{{POS}}", &context.lexeme.part_of_speech)
            .replace("{{ORIGIN}}", &context.lexeme.origin)
            .replace("{{DEFINITION}}", &context.lexeme.english_definition.as_deref().unwrap_or("N/A"))
            .replace("{{WORDFORMS}}", &format_wordforms(&context.wordforms))
            .replace("{{SENSES}}", &format_senses(&context.senses))
    }
}

fn format_wordforms(wordforms: &[Wordform]) -> String {
    if wordforms.is_empty() {
        "None".to_string()
    } else {
        wordforms.iter()
            .map(|wf| {
                let markers = if wf.is_canonical_lk { " [canonical]" } else { "" };
                format!("- {} ({}){}", wf.text, wf.script, markers)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn format_senses(senses: &[Sense]) -> String {
    if senses.is_empty() {
        "None".to_string()
    } else {
        senses.iter()
            .map(|s| format!("- {}", s.definition))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Built-in prompt templates
pub fn builtin_templates() -> Vec<PromptTemplate> {
    vec![
        PromptTemplate {
            name: "analyze".to_string(),
            template: r#"Analyze this Yiddish word:

Word: {{HEBREW}} ({{ROMAN}})
IPA: {{IPA}}
Part of Speech: {{POS}}
Origin: {{ORIGIN}}
Definition: {{DEFINITION}}

Wordforms:
{{WORDFORMS}}

Senses:
{{SENSES}}

Provide linguistic analysis including etymology, usage patterns, and interesting notes."#.to_string(),
        },
        PromptTemplate {
            name: "examples".to_string(),
            template: r#"Generate natural usage examples for this Yiddish word:

Word: {{HEBREW}} ({{ROMAN}})
Part of Speech: {{POS}}
Definition: {{DEFINITION}}

Provide 3-5 example sentences in Yiddish with transliteration and English translation."#.to_string(),
        },
        PromptTemplate {
            name: "synonyms".to_string(),
            template: r#"Find synonyms and related words for:

Word: {{HEBREW}} ({{ROMAN}})
Part of Speech: {{POS}}
Definition: {{DEFINITION}}

List Yiddish synonyms with nuances and differences in meaning or usage."#.to_string(),
        },
        PromptTemplate {
            name: "etymology".to_string(),
            template: r#"Trace the etymology of this Yiddish word:

Word: {{HEBREW}} ({{ROMAN}})
Origin: {{ORIGIN}}
Definition: {{DEFINITION}}

Provide detailed etymological analysis including source languages, historical development, and cognates in related languages."#.to_string(),
        },
        PromptTemplate {
            name: "custom".to_string(),
            template: "{{CUSTOM}}".to_string(),
        },
    ]
}

// ============================================================================
// LEXEME CONTEXT
// ============================================================================

/// Full context for a lexeme including all related data
#[derive(Debug, Clone)]
pub struct LexemeContext {
    pub lexeme: Lexeme,
    pub wordforms: Vec<Wordform>,
    pub senses: Vec<Sense>,
}

impl LexemeContext {
    /// Create a formatted summary of the full context
    pub fn format_full(&self) -> String {
        format!(
            "=== LEXEME ===\n{}\n{}\n\n{}\n\n{}",
            self.lexeme.canonical_hebrew,
            self.lexeme.canonical_roman,
            format_wordforms(&self.wordforms),
            format_senses(&self.senses)
        )
    }
}

// ============================================================================
// MODS CLIENT
// ============================================================================

pub struct ModsClient {
    /// Path to mods CLI (defaults to "mods" from PATH)
    mods_path: String,
}

impl ModsClient {
    pub fn new() -> Self {
        Self {
            mods_path: "mods".to_string(),
        }
    }

    pub fn with_path(path: String) -> Self {
        Self { mods_path: path }
    }

    /// Send a query to mods with the given prompt
    pub fn query(&self, prompt: &str) -> Result<String> {
        let output = Command::new(&self.mods_path)
            .arg(prompt)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(stdout)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(anyhow!("mods failed: {}", stderr))
                }
            }
            Err(e) => {
                Err(anyhow!("Failed to run mods: {}. Is 'mods' installed and in PATH?", e))
            }
        }
    }

    /// Send a lexeme with context using a named template
    pub fn query_lexeme(&self, context: &LexemeContext, template_name: &str) -> Result<String> {
        let templates = builtin_templates();
        let template = templates
            .iter()
            .find(|t| t.name == template_name)
            .ok_or_else(|| anyhow!("Unknown template: {}", template_name))?;

        let prompt = template.format(context);
        self.query(&prompt)
    }

    /// Send a custom prompt with lexeme context
    pub fn query_lexeme_custom(&self, context: &LexemeContext, custom_prompt: &str) -> Result<String> {
        let prompt = custom_prompt
            .replace("{{HEBREW}}", &context.lexeme.canonical_hebrew)
            .replace("{{ROMAN}}", &context.lexeme.canonical_roman)
            .replace("{{IPA}}", &context.lexeme.canonical_ipa.as_deref().unwrap_or("N/A"))
            .replace("{{POS}}", &context.lexeme.part_of_speech)
            .replace("{{ORIGIN}}", &context.lexeme.origin)
            .replace("{{DEFINITION}}", &context.lexeme.english_definition.as_deref().unwrap_or("N/A"))
            .replace("{{WORDFORMS}}", &format_wordforms(&context.wordforms))
            .replace("{{SENSES}}", &format_senses(&context.senses));

        self.query(&prompt)
    }

    /// Send raw text to mods
    pub fn query_raw(&self, text: &str) -> Result<String> {
        self.query(text)
    }
}

impl Default for ModsClient {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// List available prompt templates
pub fn list_templates() {
    let templates = builtin_templates();
    println!("Available prompt templates:\n");
    for t in templates {
        println!("  • {} - {}", t.name, template_description(&t.name));
    }
    println!("\nUse 'custom' to provide your own prompt.");
}

fn template_description(name: &str) -> &'static str {
    match name {
        "analyze" => "Full linguistic analysis",
        "examples" => "Generate usage examples",
        "synonyms" => "Find synonyms and related words",
        "etymology" => "Etymological analysis",
        "custom" => "Custom prompt (use --prompt flag)",
        _ => "Unknown template",
    }
}
