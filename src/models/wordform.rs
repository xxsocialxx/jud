use super::morph_features::MorphFeatures;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// ENHANCED WORDFORM MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wordform {
    pub id: i64,
    pub lexeme_id: Uuid,
    pub text: String,
    pub script: String,

    // Morphological features
    pub morph_features: Option<MorphFeatures>,
    pub morph_features_id: Option<i64>,

    // Classification
    pub is_standard: bool,
    pub is_canonical_lk: bool,
    pub is_verified: bool,
    pub is_inflected_form: bool,
    pub is_infinitive: bool,
    pub is_explicit_plural: bool,

    // Derivational info
    pub base_form_id: Option<i64>,
    pub is_derived: bool,
    pub derivation_type: Option<String>,

    // Origin and dialect
    pub origin: String,
    pub dialect: Option<String>,
    pub dialect_specific: Vec<String>,

    // Pronunciation
    pub pronunciation_variant: Option<String>,

    // Corpus frequency
    pub corpus_frequency: i32,

    // Verification metadata
    pub verified_by_id: Option<i64>,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Wordform {
    /// Format wordform with morphological info
    pub fn format_with_morphology(&self) -> String {
        let mut parts = vec![self.text.clone()];

        if let Some(morph) = &self.morph_features {
            let morph_str = morph.format_compact();
            if morph_str != "-" {
                parts.push(format!("({})", morph_str));
            }
        }

        parts.join(" ")
    }

    /// Format wordform as inflection table row
    pub fn format_inflection_row(&self) -> String {
        let standard = if self.is_standard { "⭐" } else { "" };
        let canonical = if self.is_canonical_lk { " [LK]" } else { "" };
        let verified = if self.is_verified { "✓" } else { "" };

        format!(
            "{}{}{} {} {}",
            standard,
            canonical,
            verified,
            self.text,
            self.morph_features
                .as_ref()
                .map(|m| m.format_compact())
                .unwrap_or_else(|| "-".to_string())
        )
    }

    /// Check if this is a base form (not derived)
    pub fn is_base_form(&self) -> bool {
        !self.is_derived && self.base_form_id.is_none()
    }

    /// Get frequency indicator
    pub fn frequency_indicator(&self) -> String {
        if self.corpus_frequency > 100 {
            "█████".to_string()
        } else if self.corpus_frequency > 50 {
            "████░".to_string()
        } else if self.corpus_frequency > 20 {
            "███░░".to_string()
        } else if self.corpus_frequency > 10 {
            "██░░░".to_string()
        } else if self.corpus_frequency > 5 {
            "█░░░░".to_string()
        } else {
            "░░░░░".to_string()
        }
    }

    /// Get dialect badge
    pub fn dialect_badge(&self) -> Option<String> {
        if let Some(dialect) = &self.dialect {
            Some(format!("🌍 {}", dialect))
        } else if !self.dialect_specific.is_empty() {
            Some(format!("🌍 {}", self.dialect_specific.join(", ")))
        } else {
            None
        }
    }

    /// Check if wordform is equivalent to another (same morphology)
    pub fn is_morphologically_equivalent(&self, other: &Wordform) -> bool {
        self.morph_features_id == other.morph_features_id
    }
}
