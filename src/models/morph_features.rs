use super::shared::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// MORPHOLOGICAL FEATURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphFeatures {
    pub id: i64,

    // Nominal features
    pub number: Option<Number>,
    pub gender: Option<Gender>,
    pub definiteness: Option<Definiteness>,

    // Verbal features
    pub tense: Option<Tense>,
    pub aspect: Option<Aspect>,
    pub person: Option<Person>,

    // Case
    pub case: Option<Case>,

    // Other features
    pub is_reflexive: bool,
    pub is_passive: bool,
    pub is_negated: bool,

    // Derivational
    pub derivation_type: DerivationType,
}

impl Default for MorphFeatures {
    fn default() -> Self {
        Self {
            id: 0,
            number: None,
            gender: None,
            definiteness: None,
            tense: None,
            aspect: None,
            person: None,
            case: None,
            is_reflexive: false,
            is_passive: false,
            is_negated: false,
            derivation_type: DerivationType::None,
        }
    }
}

impl MorphFeatures {
    /// Format morphological features as a compact string
    pub fn format_compact(&self) -> String {
        let parts: Vec<String> = vec![
            self.gender.map(|g| g.to_string()),
            self.number.map(|n| n.to_string()),
            self.case.map(|c| c.to_string()),
            self.tense.map(|t| t.to_string()),
            self.person.map(|p| p.to_string()),
            self.aspect.map(|a| a.to_string()),
            if self.is_reflexive {
                Some("Refl".to_string())
            } else {
                None
            },
            if self.is_passive {
                Some("Pass".to_string())
            } else {
                None
            },
            if self.is_negated {
                Some("Neg".to_string())
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect();

        if parts.is_empty() {
            "-".to_string()
        } else {
            parts.join(", ")
        }
    }

    /// Format morphological features as detailed display
    pub fn format_detailed(&self) -> String {
        let mut lines = vec![];

        if let Some(gender) = self.gender {
            lines.push(format!("  Gender: {}", gender));
        }
        if let Some(number) = self.number {
            lines.push(format!("  Number: {}", number));
        }
        if let Some(case) = self.case {
            lines.push(format!("  Case: {}", case));
        }
        if let Some(tense) = self.tense {
            lines.push(format!("  Tense: {}", tense));
        }
        if let Some(person) = self.person {
            lines.push(format!("  Person: {}", person));
        }
        if let Some(aspect) = self.aspect {
            lines.push(format!("  Aspect: {}", aspect));
        }
        if self.is_reflexive {
            lines.push("  Reflexive: Yes".to_string());
        }
        if self.is_passive {
            lines.push("  Passive: Yes".to_string());
        }
        if self.is_negated {
            lines.push("  Negated: Yes".to_string());
        }
        if self.derivation_type != DerivationType::None {
            lines.push(format!("  Derivation: {}", self.derivation_type));
        }

        if lines.is_empty() {
            "  (no features)".to_string()
        } else {
            lines.join("\n")
        }
    }
}
