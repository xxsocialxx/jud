// ============================================================================
// MORPHOLOGICAL ANALYZER
// ============================================================================

#![allow(dead_code)]

use crate::models::morph_features::MorphFeatures;
use crate::models::shared::*;
use crate::models::{Lexeme, Wordform};

#[derive(Debug, Clone)]
pub struct MorphAnalysis {
    pub features: MorphFeatures,
    pub derivation: Option<DerivationType>,
    pub is_base_form: bool,
    pub confidence: f32,
}

pub struct MorphologicalGenerator {
    noun_rules: NounInflectionRules,
    verb_rules: VerbInflectionRules,
    adjective_rules: AdjectiveInflectionRules,
}

impl MorphologicalGenerator {
    pub fn new() -> Self {
        Self {
            noun_rules: NounInflectionRules::new(),
            verb_rules: VerbInflectionRules::new(),
            adjective_rules: AdjectiveInflectionRules::new(),
        }
    }

    pub fn analyze(&self, wordform: &Wordform, lexeme: &Lexeme) -> MorphAnalysis {
        let mut features = MorphFeatures::default();
        let mut is_base = false;

        match lexeme.part_of_speech.as_str() {
            "Noun" | "noun" | "N" => {
                self.analyze_noun(wordform, &mut features, &mut is_base);
            }
            "Verb" | "verb" | "V" => {
                self.analyze_verb(wordform, &mut features, &mut is_base);
            }
            "Adjective" | "adjective" | "Adj" => {
                self.analyze_adjective(wordform, &mut features, &mut is_base);
            }
            _ => {}
        }

        MorphAnalysis {
            features,
            derivation: None,
            is_base_form: is_base,
            confidence: 0.8,
        }
    }

    pub fn generate_inflections(&self, lexeme: &Lexeme) -> Vec<InflectedForm> {
        match lexeme.part_of_speech.as_str() {
            "Noun" | "noun" | "N" => self.noun_rules.inflect(lexeme),
            "Verb" | "verb" | "V" => self.verb_rules.inflect(lexeme),
            _ => vec![],
        }
    }

    fn analyze_noun(&self, wordform: &Wordform, features: &mut MorphFeatures, is_base: &mut bool) {
        if wordform.text.ends_with("ס") || wordform.text.ends_with("עס") {
            features.number = Some(Number::Plural);
        } else {
            features.number = Some(Number::Singular);
            *is_base = true;
        }
    }

    fn analyze_verb(&self, wordform: &Wordform, features: &mut MorphFeatures, is_base: &mut bool) {
        if wordform.is_infinitive {
            features.tense = Some(Tense::Infinitive);
            *is_base = true;
        }
    }

    fn analyze_adjective(
        &self,
        wordform: &Wordform,
        features: &mut MorphFeatures,
        _is_base: &mut bool,
    ) {
        if wordform.is_explicit_plural {
            features.number = Some(Number::Plural);
        } else {
            features.number = Some(Number::Singular);
        }
    }
}

impl Default for MorphologicalGenerator {
    fn default() -> Self {
        Self::new()
    }
}

struct NounInflectionRules;

impl NounInflectionRules {
    fn new() -> Self {
        Self
    }

    fn inflect(&self, lexeme: &Lexeme) -> Vec<InflectedForm> {
        vec![InflectedForm {
            form: lexeme.canonical_hebrew.clone(),
            features: MorphFeatures {
                number: Some(Number::Singular),
                ..Default::default()
            },
            is_standard: true,
            is_productive: true,
        }]
    }
}

struct VerbInflectionRules;

impl VerbInflectionRules {
    fn new() -> Self {
        Self
    }

    fn inflect(&self, lexeme: &Lexeme) -> Vec<InflectedForm> {
        vec![InflectedForm {
            form: lexeme.canonical_hebrew.clone(),
            features: MorphFeatures {
                tense: Some(Tense::Infinitive),
                ..Default::default()
            },
            is_standard: true,
            is_productive: true,
        }]
    }
}

struct AdjectiveInflectionRules;

impl AdjectiveInflectionRules {
    fn new() -> Self {
        Self
    }

    fn inflect(&self, _lexeme: &Lexeme) -> Vec<InflectedForm> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct InflectedForm {
    pub form: String,
    pub features: MorphFeatures,
    pub is_standard: bool,
    pub is_productive: bool,
}
