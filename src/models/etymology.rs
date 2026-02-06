use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// ETYMOLOGY MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Etymology {
    pub id: i64,
    pub lexeme_id: Uuid,

    // Source info
    pub source_language: String,
    pub source_word: Option<String>,
    pub source_romanization: Option<String>,
    pub source_meaning: Option<String>,

    // Borrowing info
    pub borrowing_period: Option<String>,
    pub borrowing_route: Vec<String>,
    pub semantic_shift: Option<String>,
    pub phonological_shift: Option<String>,

    // Attestation
    pub earliest_attestation: Option<i32>,
    pub attestation_reference: Option<String>,

    // Confidence
    pub confidence_score: Option<f32>,
    pub is_certain: bool,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
}

impl Etymology {
    /// Format etymology as tree
    pub fn format_tree(&self) -> String {
        let mut lines = vec![];

        // Source language and word
        if let Some(source_word) = &self.source_word {
            if let Some(romanization) = &self.source_romanization {
                lines.push(format!(
                    "{} → {} ({})",
                    self.source_language, source_word, romanization
                ));
            } else {
                lines.push(format!("{} → {}", self.source_language, source_word));
            }
        } else {
            lines.push(self.source_language.to_string());
        }

        // Meaning
        if let Some(meaning) = &self.source_meaning {
            lines.push(format!("  \"{}\"", meaning));
        }

        // Borrowing route
        if !self.borrowing_route.is_empty() {
            lines.push(format!("  via: {}", self.borrowing_route.join(" → ")));
        }

        // Period
        if let Some(period) = &self.borrowing_period {
            lines.push(format!("  ({})", period));
        }

        // Shifts
        if let Some(semantic) = &self.semantic_shift {
            lines.push(format!("  Semantic shift: {}", semantic));
        }

        if let Some(phonological) = &self.phonological_shift {
            lines.push(format!("  Phonological shift: {}", phonological));
        }

        // Confidence
        let certainty = if self.is_certain {
            "✓ Certain".to_string()
        } else {
            format!(
                "? {:.0}% confidence",
                self.confidence_score.unwrap_or(0.5) * 100.0
            )
        };
        lines.push(format!("  {}", certainty));

        lines.join("\n")
    }

    /// Format compact (one-line)
    pub fn format_compact(&self) -> String {
        let default = "?".to_string();
        let word = self.source_word.as_ref().unwrap_or(&default);

        let route = if self.borrowing_route.len() > 1 {
            format!(" via {}", self.borrowing_route.join(" → "))
        } else {
            String::new()
        };

        format!(
            "{} ({}{}{})",
            word,
            self.source_language,
            route,
            if self.is_certain { "" } else { " ?" }
        )
    }

    /// Check if this is a direct borrowing
    pub fn is_direct_borrowing(&self) -> bool {
        self.borrowing_route.len() <= 1
    }

    /// Get confidence level as icon
    pub fn confidence_icon(&self) -> &'static str {
        if self.is_certain {
            "✓"
        } else if let Some(score) = self.confidence_score {
            if score > 0.8 {
                "✓"
            } else if score > 0.5 {
                "~"
            } else {
                "?"
            }
        } else {
            "?"
        }
    }
}

// ============================================================================
// HASIDIC COMMUNITY MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasidicCommunity {
    pub id: i64,
    pub name: String,
    pub english_name: Option<String>,
    pub geographic_origin: Option<String>,
    pub founding_year: Option<i32>,

    // Dialect features
    pub phonological_features: serde_json::Value,
    pub lexical_specificities: Vec<String>,

    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl HasidicCommunity {
    /// Format community name
    pub fn format_name(&self) -> String {
        if let Some(english) = &self.english_name {
            format!("{} ({})", self.name, english)
        } else {
            self.name.clone()
        }
    }

    /// Format with origin
    pub fn format_with_origin(&self) -> String {
        if let Some(origin) = &self.geographic_origin {
            format!("{} — {}", self.format_name(), origin)
        } else {
            self.format_name()
        }
    }
}

// ============================================================================
// LEXEME-HASIDIC USAGE (JUNCTION TABLE)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexemeHasidicUsage {
    pub lexeme_id: Uuid,
    pub community_id: i64,
    pub is_specific: bool,
    pub frequency: Option<f32>,
    pub notes: Option<String>,
}
