use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

// ============================================================================
// REGISTER & USAGE (Weinreich-style classification)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Register {
    Literary,
    Colloquial,
    #[default]
    Neutral,
    Archaic,
    Slang,
    Technical,
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literary => write!(f, "Literary"),
            Self::Colloquial => write!(f, "Colloquial"),
            Self::Neutral => write!(f, "Neutral"),
            Self::Archaic => write!(f, "Archaic"),
            Self::Slang => write!(f, "Slang"),
            Self::Technical => write!(f, "Technical"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum UsageCategory {
    General,
    NonHasidic,
    HasidicSpecific,
    #[default]
    Neutral,
}

impl Display for UsageCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General => write!(f, "General"),
            Self::NonHasidic => write!(f, "Non-Hasidic"),
            Self::HasidicSpecific => write!(f, "Hasidic-Specific"),
            Self::Neutral => write!(f, "Neutral"),
        }
    }
}

// ============================================================================
// PART OF SPEECH
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Particle,
    Interjection,
    Numeral,
    Determiner,
    Auxiliary,
}

impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Noun => write!(f, "Noun"),
            Self::Verb => write!(f, "Verb"),
            Self::Adjective => write!(f, "Adjective"),
            Self::Adverb => write!(f, "Adverb"),
            Self::Pronoun => write!(f, "Pronoun"),
            Self::Preposition => write!(f, "Preposition"),
            Self::Conjunction => write!(f, "Conjunction"),
            Self::Particle => write!(f, "Particle"),
            Self::Interjection => write!(f, "Interjection"),
            Self::Numeral => write!(f, "Numeral"),
            Self::Determiner => write!(f, "Determiner"),
            Self::Auxiliary => write!(f, "Auxiliary"),
        }
    }
}

// ============================================================================
// CONNOTATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Connotation {
    #[default]
    Neutral,
    Positive,
    Negative,
    Euphemistic,
    Pejorative,
    Ironic,
}

impl Display for Connotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neutral => write!(f, "Neutral"),
            Self::Positive => write!(f, "Positive"),
            Self::Negative => write!(f, "Negative"),
            Self::Euphemistic => write!(f, "Euphemistic"),
            Self::Pejorative => write!(f, "Pejorative"),
            Self::Ironic => write!(f, "Ironic"),
        }
    }
}

// ============================================================================
// SEMANTIC RELATIONSHIP TYPES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticRelationship {
    Synonym,
    Antonym,
    Hypernym,
    Hyponym,
    Meronym,
    Holonym,
    Entailment,
    Causation,
}

impl Display for SemanticRelationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Synonym => write!(f, "Synonym"),
            Self::Antonym => write!(f, "Antonym"),
            Self::Hypernym => write!(f, "Hypernym"),
            Self::Hyponym => write!(f, "Hyponym"),
            Self::Meronym => write!(f, "Meronym"),
            Self::Holonym => write!(f, "Holonym"),
            Self::Entailment => write!(f, "Entailment"),
            Self::Causation => write!(f, "Causation"),
        }
    }
}

// ============================================================================
// MORPHOLOGICAL FEATURE ENUMS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Masculine => write!(f, "Masc"),
            Self::Feminine => write!(f, "Fem"),
            Self::Neuter => write!(f, "Neut"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Number {
    Singular,
    Plural,
    Dual,
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Singular => write!(f, "Sg"),
            Self::Plural => write!(f, "Pl"),
            Self::Dual => write!(f, "Dual"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tense {
    Present,
    Past,
    Future,
    Infinitive,
    Imperative,
    Participle,
}

impl Display for Tense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Present => write!(f, "Pres"),
            Self::Past => write!(f, "Past"),
            Self::Future => write!(f, "Fut"),
            Self::Infinitive => write!(f, "Inf"),
            Self::Imperative => write!(f, "Impr"),
            Self::Participle => write!(f, "Part"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Person {
    First,
    Second,
    Third,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "1st"),
            Self::Second => write!(f, "2nd"),
            Self::Third => write!(f, "3rd"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Case {
    Nominative,
    Accusative,
    Dative,
    Genitive,
    Locative,
}

impl Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nominative => write!(f, "Nom"),
            Self::Accusative => write!(f, "Acc"),
            Self::Dative => write!(f, "Dat"),
            Self::Genitive => write!(f, "Gen"),
            Self::Locative => write!(f, "Loc"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Aspect {
    Perfective,
    Imperfective,
    Neutral,
}

impl Display for Aspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Perfective => write!(f, "Perf"),
            Self::Imperfective => write!(f, "Imperf"),
            Self::Neutral => write!(f, "Neutral"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Definiteness {
    Definite,
    Indefinite,
}

impl Display for Definiteness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definite => write!(f, "Def"),
            Self::Indefinite => write!(f, "Indef"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum DerivationType {
    Diminutive,
    Augmentative,
    Nominalizer,
    Verbalizer,
    Adjectival,
    Frequentative,
    Privative,
    #[default]
    None,
}

impl Display for DerivationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Diminutive => write!(f, "Diminutive"),
            Self::Augmentative => write!(f, "Augmentative"),
            Self::Nominalizer => write!(f, "Nominalizer"),
            Self::Verbalizer => write!(f, "Verbalizer"),
            Self::Adjectival => write!(f, "Adjectival"),
            Self::Frequentative => write!(f, "Frequentative"),
            Self::Privative => write!(f, "Privative"),
            Self::None => write!(f, "-"),
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Format a frequency score as a visual bar
pub fn format_frequency_bar(score: Option<f32>) -> String {
    match score {
        Some(score) if score > 0.0 => {
            let filled = (score / 100.0 * 20.0).round() as usize;
            let empty = 20 - filled;
            format!(
                "█{} ░{} ({:.0}%)",
                "█".repeat(filled.saturating_sub(1)),
                "░".repeat(empty),
                score
            )
        }
        _ => "░░░░░░░░░░░░░░░░░░░░ (N/A)".to_string(),
    }
}

/// Format register with icon
pub fn format_register(register: Register) -> String {
    match register {
        Register::Literary => "📚 Literary".to_string(),
        Register::Colloquial => "💬 Colloquial".to_string(),
        Register::Neutral => "⚪ Neutral".to_string(),
        Register::Archaic => "📜 Archaic".to_string(),
        Register::Slang => "🎭 Slang".to_string(),
        Register::Technical => "🔧 Technical".to_string(),
    }
}
