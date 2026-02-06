use super::shared::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// IDIOM MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idiom {
    pub id: i64,

    // Idiom text
    pub idiom_yiddish: String,
    pub idiom_romanized: Option<String>,
    pub literal_translation: Option<String>,
    pub idiomatic_meaning: String,

    // Metadata
    pub register: Register,
    pub usage_category: UsageCategory,

    // Relationships
    pub participating_senses: Vec<i64>,
    pub participating_wordforms: Vec<i64>,

    // Examples
    pub example_yiddish: Option<String>,
    pub example_english: Option<String>,

    // Source
    pub source_reference: Option<String>,
    pub notes: Option<String>,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Idiom {
    /// Format idiom header
    pub fn format_header(&self) -> String {
        if let Some(romanized) = &self.idiom_romanized {
            format!("{} / {}", self.idiom_yiddish, romanized)
        } else {
            self.idiom_yiddish.clone()
        }
    }

    /// Format with literal translation
    pub fn format_with_literal(&self) -> String {
        let mut lines = vec![self.format_header()];

        if let Some(literal) = &self.literal_translation {
            lines.push(format!("  lit. \"{}\"", literal));
        }

        lines.push(format!("  → {}", self.idiomatic_meaning));

        lines.join("\n")
    }

    /// Format with example
    pub fn format_with_example(&self) -> String {
        let mut text = self.format_with_literal();

        if let Some(example_yi) = &self.example_yiddish {
            text.push_str(&format!("\n\n  Example: {}", example_yi));
            if let Some(example_en) = &self.example_english {
                text.push_str(&format!("\n  \"{}\"", example_en));
            }
        }

        text
    }

    /// Get register badge
    pub fn register_badge(&self) -> String {
        format_register(self.register)
    }
}
