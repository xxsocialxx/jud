use super::shared::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// ENHANCED LEXEME MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lexeme {
    // Core identity
    pub id: Uuid,
    pub canonical_hebrew: String,
    pub canonical_roman: String,
    pub canonical_ipa: Option<String>,

    // Classification
    pub part_of_speech: String,
    pub gender: Option<String>,

    // Weinreich-style metadata
    pub register: Register,
    pub usage_category: UsageCategory,
    pub usage_frequency_score: Option<f32>,

    // Etymology hints (full etymology in separate table)
    pub is_borrowed: bool,
    pub etymology_source_language: Option<String>,
    pub notes: Option<String>,

    // Original fields (for compatibility)
    pub skeleton_key: Option<String>,
    pub phonetic_key: Option<String>,
    pub skeleton_collision: bool,
    pub romanization_status: String,
    pub romanization_confidence: f64,
    pub ipa_confidence: f64,
    pub romanization_source: String,
    pub romanization_version: i32,
    pub origin: String,
    pub english_definition: Option<String>,
    pub status: String,
    pub analyzed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // Computed counts (from views)
    pub wordform_count: Option<i64>,
    pub sense_count: Option<i64>,
    pub etymology_count: Option<i64>,
    pub idiom_count: Option<i64>,

    // Toggleable features (Foundation v0.2.0)
    /// Number of usage examples available for this lexeme
    pub example_count: i32,
    /// Confidence score (0.0-1.0) for POS tagging; 1.0 = human-verified
    pub pos_confidence: f32,
    /// Percentage (0.0-1.0) of possible inflections with morphology
    pub morphology_richness: f32,
    /// Percentage (0.0-1.0) of etymology fields populated
    pub etymology_completeness: f32,
    /// Timestamp of last LLM enhancement
    pub last_llm_update: Option<DateTime<Utc>>,
    /// True if lexeme has verified corpus examples
    pub has_corpus_examples: bool,

    // Rich metadata
    pub semantic_tags: Vec<String>,
    pub cross_references: serde_json::Value,
}

impl Lexeme {
    /// Format lexeme header for display
    pub fn format_header(&self) -> String {
        format!("װ{} ({})", self.canonical_hebrew, self.canonical_roman)
    }

    /// Format metadata line
    pub fn format_metadata(&self) -> String {
        let parts = vec![
            Some(self.part_of_speech.clone()),
            Some(format_register(self.register)),
            self.usage_frequency_score
                .map(|score| format!("{:.0}/100", score)),
        ];

        parts.into_iter().flatten().collect::<Vec<_>>().join(" • ")
    }

    /// Get frequency bar visualization
    pub fn frequency_bar(&self) -> String {
        format_frequency_bar(self.usage_frequency_score)
    }

    /// Check if lexeme has rich metadata
    pub fn has_rich_metadata(&self) -> bool {
        self.semantic_tags.iter().any(|t| !t.is_empty()) || self.notes.is_some() || self.is_borrowed
    }

    /// Format etymology hint
    pub fn format_etymology_hint(&self) -> Option<String> {
        if self.is_borrowed {
            Some(format!(
                "↳ {} ({})",
                self.etymology_source_language
                    .as_ref()
                    .unwrap_or(&"unknown".to_string()),
                self.origin
            ))
        } else {
            None
        }
    }

    /// Get display priority (for sorting)
    pub fn display_priority(&self) -> i32 {
        let mut score = 0;

        // High frequency words get priority
        if let Some(freq) = self.usage_frequency_score {
            score += (freq / 10.0) as i32;
        }

        // Neutral register gets priority
        if self.register == Register::Neutral {
            score += 5;
        }

        // General usage gets priority
        if self.usage_category == UsageCategory::General {
            score += 3;
        }

        score
    }
}

// ============================================================================
// LEXEME SEARCH RESULT (with aggregates)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexemeSearchResult {
    pub lexeme: Lexeme,
    pub wordforms: Vec<String>,
    pub senses: Vec<String>,
}

impl LexemeSearchResult {
    /// Format as compact search result
    pub fn format_compact(&self) -> String {
        let freq = self.lexeme.frequency_bar();
        format!(
            "{} → {}\n    {} • {}",
            self.lexeme.canonical_hebrew,
            self.lexeme.canonical_roman,
            self.lexeme.format_metadata(),
            freq
        )
    }

    /// Format as detailed search result
    pub fn format_detailed(&self) -> String {
        let mut lines = vec![];

        lines.push(format!(
            "{} ({})",
            self.lexeme.canonical_hebrew, self.lexeme.canonical_roman
        ));
        lines.push(format!("  {}", self.lexeme.format_metadata()));
        lines.push(format!("  {}", self.lexeme.frequency_bar()));

        if let Some(def) = &self.lexeme.english_definition {
            lines.push(format!("  {}", def));
        }

        if !self.wordforms.is_empty() {
            let wf_preview = if self.wordforms.len() > 3 {
                format!("{}...", self.wordforms[..3].join(", "))
            } else {
                self.wordforms.join(", ")
            };
            lines.push(format!("  Wordforms: {}", wf_preview));
        }

        if !self.senses.is_empty() {
            let sense_preview = if self.senses.len() > 2 {
                format!("{}...", self.senses[..2].join(" | "))
            } else {
                self.senses.join(" | ")
            };
            lines.push(format!("  Senses: {}", sense_preview));
        }

        lines.join("\n")
    }
}
