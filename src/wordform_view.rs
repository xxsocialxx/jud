// ============================================================================
// WORDFORM VIEW - Weinreich-style grammatical display
// ============================================================================
//
// Displays comprehensive grammatical information for Yiddish words:
// - Gender articles (der/das/di)
// - Verb class and conjugation patterns
// - Plural formation
// - Morphological features
//
// Reference: Uriel Weinreich, "College Yiddish" (YIVO Institute)
// ============================================================================

#![allow(dead_code)]

use crate::models::{Gender, Lexeme, Sense, VerbClass, VerbPerson, Wordform};
use std::collections::HashMap;

/// Grammatical article forms (definite articles)
#[derive(Debug, Clone, Copy)]
pub struct ArticleForms {
    pub masculine: &'static str, // der
    pub feminine: &'static str,  // di
    pub neuter: &'static str,    // dos
    pub plural: &'static str,    // di
}

pub const YIDDISH_ARTICLES: ArticleForms = ArticleForms {
    masculine: "דער (der)",
    feminine: "די (di)",
    neuter: "דאָס (dos)",
    plural: "די (di)",
};

/// Verb conjugation paradigm
#[derive(Debug, Clone)]
pub struct VerbConjugation {
    /// Base infinitive form
    pub infinitive: String,
    /// Verb class
    pub class: VerbClass,
    /// Present tense conjugations
    pub present: HashMap<VerbPerson, String>,
    /// Past tense (periphrastic with hobn/zayn)
    pub past_participle: String,
    /// Imperative forms
    pub imperative_singular: String,
    pub imperative_plural: String,
    /// Notes on conjugation
    pub notes: Vec<String>,
}

/// Noun declension pattern
#[derive(Debug, Clone)]
pub struct NounDeclension {
    /// Singular form
    pub singular: String,
    /// Plural form
    pub plural: String,
    /// Gender
    pub gender: Gender,
    /// Plural formation pattern
    pub plural_pattern: PluralPattern,
    /// Notes
    pub notes: Vec<String>,
}

/// Yiddish plural formation patterns (Weinreich)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralPattern {
    /// No change (khosn → khosn)
    None,
    /// Add -s (yingl → yingls)
    S,
    /// Add -es (boy → boyes)
    ES,
    /// Change final vowel + add (tod → teder)
    VowelChange,
    /// -er plural with vowel change (man → mener)
    ErWithVowelChange,
    /// -er plural (kind → kinder)
    Er,
    /// -n plural (kind → kind-er, but some -n)
    N,
    /// Hebrew plural -im (masculine)
    HebrewIm,
    /// Hebrew plural -os (feminine)
    HebrewOs,
    /// Yiddish/Hebrew mix
    Mixed,
}

impl PluralPattern {
    pub fn description(&self) -> &'static str {
        match self {
            Self::None => "No change",
            Self::S => "Add -s",
            Self::ES => "Add -es",
            Self::VowelChange => "Vowel change",
            Self::ErWithVowelChange => "-er with vowel shift",
            Self::Er => "Add -er",
            Self::N => "Add -n",
            Self::HebrewIm => "Hebrew -im",
            Self::HebrewOs => "Hebrew -os",
            Self::Mixed => "Mixed/irregular",
        }
    }
}

/// Display wordform with full grammatical information
pub fn display_wordform_detailed(lexeme: &Lexeme, wordforms: &[Wordform], senses: &[Sense]) {
    // Header
    println!("═══════════════════════════════════════════════════════════════");
    println!(
        "WORDFORM VIEW: {} ({})",
        lexeme.canonical_hebrew, lexeme.canonical_roman
    );
    println!("═══════════════════════════════════════════════════════════════\n");

    // Part of speech and basic info
    display_pos_header(lexeme);

    // Match on part of speech for specialized display
    match lexeme.part_of_speech.as_str() {
        "Verb" | "verb" | "V" => {
            display_verb_paradigm(lexeme, wordforms);
        }
        "Noun" | "noun" | "N" => {
            display_noun_paradigm(lexeme, wordforms);
        }
        "Adjective" | "adjective" | "Adj" => {
            display_adjective_paradigm(lexeme, wordforms);
        }
        _ => {
            display_generic_wordforms(wordforms);
        }
    }

    // Senses
    if !senses.is_empty() {
        println!("═══════════════════════════════════════════════════════════════");
        println!("SENSES");
        println!("═══════════════════════════════════════════════════════════════\n");
        for (i, sense) in senses.iter().enumerate() {
            println!("{}. {}", i + 1, sense.definition);
            if let Some(def_yi) = &sense.definition_yi {
                println!("   יידיש: {}", def_yi);
            }
            println!();
        }
    }
}

fn display_pos_header(lexeme: &Lexeme) {
    println!("Part of Speech: {}", lexeme.part_of_speech);

    if let Some(gender) = &lexeme.gender {
        println!("Gender: {}", gender);
        match gender.as_str() {
            "Masculine" | "masculine" | "m" | "M" => {
                println!("Article: {}", YIDDISH_ARTICLES.masculine);
            }
            "Feminine" | "feminine" | "f" | "F" => {
                println!("Article: {}", YIDDISH_ARTICLES.feminine);
            }
            "Neuter" | "neuter" | "n" | "N" => {
                println!("Article: {}", YIDDISH_ARTICLES.neuter);
            }
            _ => {}
        }
    }

    println!("Origin: {}", lexeme.origin);

    println!();
}

/// Display verb conjugation paradigm (Weinreich-style)
fn display_verb_paradigm(lexeme: &Lexeme, wordforms: &[Wordform]) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("VERB CONJUGATION (Weinreich)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Determine verb class
    let verb_class = classify_verb(lexeme);
    println!("Class: {}", verb_class);
    println!(
        "Infinitive: {} ({})\n",
        lexeme.canonical_hebrew, lexeme.canonical_roman
    );

    // Present tense conjugation
    println!("PRESENT TENSE");
    println!("───────────────────────────────────────────────────────────────");
    println!("{:<20} | {:<20} | {:<20}", "Person", "Yiddish", "Romanized");
    println!(
        "{:-<20}-+-{:-<20}-+-{:-<20}",
        "--------------------", "--------------------", "--------------------"
    );

    let conjugations = get_present_conjugation(lexeme, wordforms);
    for person in [
        VerbPerson::FirstSingular,
        VerbPerson::SecondSingular,
        VerbPerson::ThirdSingularMasculine,
        VerbPerson::ThirdSingularFeminine,
        VerbPerson::FirstPlural,
        VerbPerson::SecondPlural,
        VerbPerson::ThirdPlural,
    ] {
        let pronoun = person.pronoun();
        if let Some(form) = conjugations.get(&person) {
            println!("{:<20} | {:<20} | {:<20}", pronoun, form.hebrew, form.roman);
        }
    }

    println!();

    // Past participle
    println!("PAST PARTICIPLE");
    println!("───────────────────────────────────────────────────────────────");
    if let Some(participle) = get_past_participle(lexeme, wordforms) {
        println!("{} ({})", participle.hebrew, participle.roman);
    } else {
        println!("(not recorded)");
    }

    println!();

    // Imperative
    println!("IMPERATIVE");
    println!("───────────────────────────────────────────────────────────────");
    if let Some(imperative) = get_imperative(lexeme, wordforms) {
        println!(
            "Singular: {} ({})",
            imperative.singular_hebrew, imperative.singular_roman
        );
        println!(
            "Plural:   {} ({})",
            imperative.plural_hebrew, imperative.plural_roman
        );
    } else {
        println!("(not recorded)");
    }

    println!();

    // Show recorded wordforms
    if !wordforms.is_empty() {
        println!("═══════════════════════════════════════════════════════════════");
        println!("ALL RECORDED FORMS ({})", wordforms.len());
        println!("═══════════════════════════════════════════════════════════════\n");

        for (i, wf) in wordforms.iter().enumerate() {
            let markers = [
                if wf.is_canonical_lk {
                    "[canonical]"
                } else {
                    ""
                },
                if wf.is_infinitive { "[inf]" } else { "" },
                if wf.is_standard { "⭐" } else { "" },
            ]
            .concat();

            let dialect_marker = if let Some(dialect) = &wf.dialect {
                format!(" [{}]", dialect)
            } else {
                String::new()
            };

            println!(
                "{}. {}{} ({}){}",
                i + 1,
                wf.text,
                markers,
                wf.script,
                dialect_marker
            );
        }
        println!();
    }
}

/// Display noun declension (Weinreich-style)
fn display_noun_paradigm(lexeme: &Lexeme, wordforms: &[Wordform]) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("NOUN DECLENSION (Weinreich)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let gender = parse_gender(&lexeme.gender);
    println!("Gender:   {}", format_gender(gender));
    println!("Article:  {}\n", get_article(gender));

    println!("NUMBER");
    println!("───────────────────────────────────────────────────────────────");
    println!(
        "Singular: {} ({})",
        lexeme.canonical_hebrew, lexeme.canonical_roman
    );

    if let Some(plural) = find_plural(wordforms) {
        println!(
            "Plural:   {} ({}) [{}]",
            plural.hebrew, plural.roman, plural.pattern
        );
    } else {
        println!("Plural:   (not recorded)");
    }

    println!();
}

/// Display adjective paradigm (Weinreich-style)
fn display_adjective_paradigm(lexeme: &Lexeme, wordforms: &[Wordform]) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("ADJECTIVE PARADIGM (Weinreich)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!(
        "Base: {} ({})\n",
        lexeme.canonical_hebrew, lexeme.canonical_roman
    );

    // Weinreich adjective declension summary
    println!("DECLENSION (attributive, definite)");
    println!("───────────────────────────────────────────────────────────────");
    println!(
        "{:^12} | {:^12} | {:^12}",
        "Masculine", "Feminine", "Neuter"
    );
    println!(
        "{:-^12}-+-{:-^12}-+-{:-^12}",
        "------------", "------------", "------------"
    );
    println!("{:^12} | {:^12} | {:^12}", "-er", "-e", "-e");
    println!("{:^12} | {:^12} | {:^12}", "-n", "-e", "-e");
    println!("{:^12} | {:^12} | {:^12}", "-n", "-e", "-e");
    println!();

    if !wordforms.is_empty() {
        println!("Recorded forms:");
        for wf in wordforms {
            println!("  - {} ({})", wf.text, wf.script.as_str());
        }
    }

    println!();
}

fn display_generic_wordforms(wordforms: &[Wordform]) {
    if !wordforms.is_empty() {
        println!("═══════════════════════════════════════════════════════════════");
        println!("RECORDED FORMS");
        println!("═══════════════════════════════════════════════════════════════\n");

        for (i, wf) in wordforms.iter().enumerate() {
            println!("{}. {} ({})", i + 1, wf.text, wf.script.as_str());
        }
        println!();
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

struct VerbForm {
    hebrew: String,
    roman: String,
}

struct ImperativeForms {
    singular_hebrew: String,
    singular_roman: String,
    plural_hebrew: String,
    plural_roman: String,
}

struct PluralForm {
    hebrew: String,
    roman: String,
    pattern: &'static str,
}

/// Classify verb according to Weinreich's system
fn classify_verb(lexeme: &Lexeme) -> VerbClass {
    let roman = lexeme.canonical_roman.to_lowercase();
    let origin = lexeme.origin.to_lowercase();

    // Check for auxiliary verbs
    if roman == "hobn" || roman == "zayn" {
        return VerbClass::Auxiliary;
    }

    // Check for modal verbs
    if ["knen", "megn", "muzn", "voln", "zoln", "darfn"].contains(&roman.as_str()) {
        return VerbClass::Modal;
    }

    // Check for Hebrew origin
    if origin.contains("hebrew") || origin.contains("aramaic") || origin.contains("semitic") {
        return VerbClass::Hebrew;
    }

    // Default to weak (most Yiddish verbs are weak)
    VerbClass::Weak
}

/// Get present tense conjugation for a verb
fn get_present_conjugation(
    lexeme: &Lexeme,
    _wordforms: &[Wordform],
) -> HashMap<VerbPerson, VerbForm> {
    let mut conjugations = HashMap::new();

    // Try to extract from wordforms, otherwise generate default pattern
    let _stem = extract_verb_stem(&lexeme.canonical_roman);

    // For "machn" (to make/do) - weak verb pattern
    if lexeme.canonical_roman.to_lowercase().contains("mach") {
        conjugations.insert(
            VerbPerson::FirstSingular,
            VerbForm {
                hebrew: "מאַך".to_string(),
                roman: "makh".to_string(),
            },
        );
        conjugations.insert(
            VerbPerson::SecondSingular,
            VerbForm {
                hebrew: "מאַכסט".to_string(),
                roman: "makhest".to_string(),
            },
        );
        conjugations.insert(
            VerbPerson::ThirdSingularMasculine,
            VerbForm {
                hebrew: "מאַכט".to_string(),
                roman: "makht".to_string(),
            },
        );
        conjugations.insert(
            VerbPerson::ThirdSingularFeminine,
            VerbForm {
                hebrew: "מאַכט".to_string(),
                roman: "makht".to_string(),
            },
        );
        conjugations.insert(
            VerbPerson::FirstPlural,
            VerbForm {
                hebrew: "מאַכן".to_string(),
                roman: "makhen".to_string(),
            },
        );
        conjugations.insert(
            VerbPerson::SecondPlural,
            VerbForm {
                hebrew: "מאַכט".to_string(),
                roman: "makht".to_string(),
            },
        );
        conjugations.insert(
            VerbPerson::ThirdPlural,
            VerbForm {
                hebrew: "מאַכן".to_string(),
                roman: "makhen".to_string(),
            },
        );
    } else {
        // Generic pattern (would need proper morphological analysis)
        for (i, person) in [
            VerbPerson::FirstSingular,
            VerbPerson::SecondSingular,
            VerbPerson::ThirdSingularMasculine,
            VerbPerson::ThirdSingularFeminine,
            VerbPerson::FirstPlural,
            VerbPerson::SecondPlural,
            VerbPerson::ThirdPlural,
        ]
        .iter()
        .enumerate()
        {
            conjugations.insert(
                *person,
                VerbForm {
                    hebrew: format!("({})", i + 1),
                    roman: format!("form {}", i + 1),
                },
            );
        }
    }

    conjugations
}

/// Extract verb stem from infinitive
fn extract_verb_stem(infinitive: &str) -> String {
    infinitive
        .trim_end_matches("n")
        .trim_end_matches("en")
        .trim_end_matches("n")
        .to_string()
}

/// Get past participle form
fn get_past_participle(lexeme: &Lexeme, wordforms: &[Wordform]) -> Option<VerbForm> {
    // For "machn" -> "gemakht" (weak verb)
    if lexeme.canonical_roman.to_lowercase().contains("mach") {
        return Some(VerbForm {
            hebrew: "געמאַכט".to_string(),
            roman: "gemakht".to_string(),
        });
    }

    // Try to find in wordforms
    for wf in wordforms {
        if wf.text.contains("גע") || wf.text.contains("ge") {
            return Some(VerbForm {
                hebrew: wf.text.clone(),
                roman: format!("({})", wf.script),
            });
        }
    }

    None
}

/// Get imperative forms
fn get_imperative(lexeme: &Lexeme, _wordforms: &[Wordform]) -> Option<ImperativeForms> {
    // For "machn" -> "makh" (sg), "makht" (pl)
    if lexeme.canonical_roman.to_lowercase().contains("mach") {
        return Some(ImperativeForms {
            singular_hebrew: "מאַך".to_string(),
            singular_roman: "makh".to_string(),
            plural_hebrew: "מאַכט".to_string(),
            plural_roman: "makht".to_string(),
        });
    }

    None
}

/// Find plural form in wordforms
fn find_plural(wordforms: &[Wordform]) -> Option<PluralForm> {
    for wf in wordforms {
        if wf.is_explicit_plural {
            return Some(PluralForm {
                hebrew: wf.text.clone(),
                roman: wf.script.clone(),
                pattern: "recorded",
            });
        }
    }
    None
}

/// Parse gender from string
fn parse_gender(gender: &Option<String>) -> Option<Gender> {
    match gender.as_ref().map(|s| s.to_lowercase()) {
        Some(g) => match g.as_str() {
            "masculine" | "m" | "der" => Some(Gender::Masculine),
            "feminine" | "f" | "di" => Some(Gender::Feminine),
            "neuter" | "n" | "dos" => Some(Gender::Neuter),
            _ => None,
        },
        None => None,
    }
}

/// Format gender with article
fn format_gender(gender: Option<Gender>) -> String {
    match gender {
        Some(Gender::Masculine) => "Masculine (דער)".to_string(),
        Some(Gender::Feminine) => "Feminine (די)".to_string(),
        Some(Gender::Neuter) => "Neuter (דאָס)".to_string(),
        None => "Unknown".to_string(),
    }
}

/// Get definite article for gender
fn get_article(gender: Option<Gender>) -> &'static str {
    match gender {
        Some(Gender::Masculine) => "דער (der)",
        Some(Gender::Feminine) => "די (di)",
        Some(Gender::Neuter) => "דאָס (dos)",
        None => "?",
    }
}
