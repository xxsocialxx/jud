use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// ENHANCED SENSE MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sense {
    pub id: i64,
    pub lexeme_id: Uuid,

    // Definition
    pub definition_number: Option<i32>,
    pub definition: String,
    pub definition_yi: Option<String>,
    pub definition_english: Option<String>,

    // Semantic classification
    pub semantic_field: Vec<String>,
    pub register_specific: Option<String>,
    pub dialect_specific: Vec<String>,
    pub domain_specific: Vec<String>,

    // Pragmatics
    pub connotation: Option<String>,
    pub frequency_in_sense: Option<f32>,
    pub is_primary_sense: bool,

    // Metadata
    pub flow_state: String,
    pub source_id: Option<i64>,
    pub usage_notes: Option<String>,

    // Related data (lazy loaded)
    pub examples: Vec<UsageExample>,
    pub relationships: Vec<SenseRelationship>,
}

impl Sense {
    /// Format sense header with number
    pub fn format_header(&self) -> String {
        match self.definition_number {
            Some(num) => format!("{}. {}", num, self.definition),
            None => self.definition.clone(),
        }
    }

    /// Format sense with Yiddish definition
    pub fn format_bilingual(&self) -> String {
        if let Some(def_yi) = &self.definition_yi {
            format!("{} (יידיש: {})", self.definition, def_yi)
        } else {
            self.definition.clone()
        }
    }

    /// Format sense metadata
    pub fn format_metadata(&self) -> String {
        let parts: Vec<String> = vec![
            self.is_primary_sense.then(|| "⭐ Primary".to_string()),
            self.connotation.clone(),
            (!self.semantic_field.is_empty())
                .then(|| format!("Fields: {}", self.semantic_field.join(", "))),
            (!self.dialect_specific.is_empty())
                .then(|| format!("Dialect: {}", self.dialect_specific.join(", "))),
            self.frequency_in_sense.map(|f| format!("Freq: {:.0}%", f)),
        ]
        .into_iter()
        .flatten()
        .collect();

        if parts.is_empty() {
            "[No metadata]".to_string()
        } else {
            parts.join(" | ")
        }
    }

    /// Get examples formatted
    pub fn format_examples(&self) -> Vec<String> {
        self.examples
            .iter()
            .map(|ex| ex.format_bilingual())
            .collect()
    }

    /// Check if sense has rich data
    pub fn has_examples(&self) -> bool {
        !self.examples.is_empty()
    }

    /// Check if sense has relationships
    pub fn has_relationships(&self) -> bool {
        !self.relationships.is_empty()
    }
}

// ============================================================================
// USAGE EXAMPLE MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageExample {
    pub id: i64,
    pub sense_id: i64,

    // Example text
    pub example_yiddish: String,
    pub example_romanized: Option<String>,
    pub example_english: String,

    // Source attribution
    pub source_reference: Option<String>,
    pub source_type: Option<String>,
    pub year: Option<i32>,

    // Metadata
    pub is_colloquial: bool,
    pub register: String,
    pub notes: Option<String>,
}

impl UsageExample {
    /// Format as bilingual example
    pub fn format_bilingual(&self) -> String {
        let yiddish = if let Some(romanized) = &self.example_romanized {
            format!("{} / {}", self.example_yiddish, romanized)
        } else {
            self.example_yiddish.clone()
        };

        format!("• {}\n  \"{}\"", yiddish, self.example_english)
    }

    /// Format with source
    pub fn format_with_source(&self) -> String {
        let mut lines = vec![self.format_bilingual()];

        if let Some(source) = &self.source_reference {
            lines.push(format!("  — {}", source));
        }

        if let Some(year) = self.year {
            lines.push(format!("  ({})", year));
        }

        lines.join("\n")
    }
}

// ============================================================================
// SENSE RELATIONSHIP MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseRelationship {
    pub id: i64,
    pub from_sense_id: i64,
    pub to_sense_id: i64,
    pub relationship_type: String,
    pub confidence_score: Option<f32>,
    pub notes: Option<String>,
}

impl SenseRelationship {
    /// Format relationship as string
    pub fn format(&self) -> String {
        let conf = self
            .confidence_score
            .map(|c| format!(" ({:.0}%)", c * 100.0))
            .unwrap_or_default();

        format!("{} {}", self.relationship_type, conf)
    }
}
