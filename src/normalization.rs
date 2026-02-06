// ============================================================================
// HEBREW TEXT NORMALIZATION
// ============================================================================

#![allow(dead_code)]

/// Normalize Yiddish text for fuzzy search and comparison
///
/// This module handles:
/// - Unicode normalization (NFC/NFD)
/// - Removal of diacritical marks (optional)
/// - Standardization of variant characters
/// - Case-insensitive comparison (for Latin script)
use unicode_normalization::UnicodeNormalization;

// ============================================================================
// NORMALIZATION OPTIONS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationLevel {
    /// Full normalization - remove all vowels/diacritics
    Aggressive,
    /// Remove only some diacritics, keep distinctions
    Moderate,
    /// Minimal normalization - Unicode only
    Minimal,
}

#[derive(Debug, Clone)]
pub struct NormalizationOptions {
    pub level: NormalizationLevel,
    pub remove_vowels: bool,
    pub normalize_kaf: bool,
    pub normalize_final_letters: bool,
}

impl Default for NormalizationOptions {
    fn default() -> Self {
        Self {
            level: NormalizationLevel::Moderate,
            remove_vowels: false,
            normalize_kaf: true,
            normalize_final_letters: true,
        }
    }
}

// ============================================================================
// CHARACTER MAPPINGS
// ============================================================================

/// Yiddish/Hebrew character equivalences
const FINAL_FORM_MAP: &[(char, char)] = &[
    ('ך', 'כ'), // Final kaf → kaf
    ('ם', 'מ'), // Final mem → mem
    ('ן', 'נ'), // Final nun → nun
    ('ף', 'פ'), // Final pe → pe
    ('ץ', 'צ'), // Final tsade → tsade
];

/// Alternative character forms in Yiddish
const VARIANT_FORMS: &[(&str, &str)] = &[
    ("ױ", "וי"), // Vav yod → vav + yod
    ("ײ", "יי"), // Yir yod → yod + yod
    ("װ", "וו"), // Vav vav → vav + vav
];

/// Points (vowels/diacritics) to remove in aggressive mode
const POINTS_TO_REMOVE: &[char] = &[
    '\u{05B0}', '\u{05B1}', '\u{05B2}', '\u{05B3}', '\u{05B4}', // Hataf vowels
    '\u{05B5}', '\u{05B6}', '\u{05B7}', '\u{05B8}', '\u{05B9}', // Sheva, Hataf
    '\u{05BB}', '\u{05BC}', '\u{05C7}', // Other points
    '\u{0591}', '\u{0592}', '\u{0593}', '\u{0594}', // Accent marks
];

// ============================================================================
// NORMALIZATION FUNCTIONS
// ============================================================================

/// Normalize Yiddish text according to options
pub fn normalize_yiddish(text: &str, options: &NormalizationOptions) -> String {
    let mut result = text.to_string();

    // 1. Unicode normalization (NFC - canonical composition)
    result = result.nfc().collect::<String>();

    // 2. Replace variant forms
    for (variant, standard) in VARIANT_FORMS {
        result = result.replace(variant, standard);
    }

    // 3. Normalize final letters to standard forms
    if options.normalize_final_letters {
        for (final_char, standard_char) in FINAL_FORM_MAP {
            result = result.replace(
                final_char.to_string().as_str(),
                standard_char.to_string().as_str(),
            );
        }
    }

    // 4. Remove diacritical points based on level
    match options.level {
        NormalizationLevel::Aggressive => {
            // Remove all points
            result = result
                .chars()
                .filter(|c| !POINTS_TO_REMOVE.contains(c))
                .collect();
        }
        NormalizationLevel::Moderate => {
            // Keep shva and hataf, remove others
            // (This is a simplified version - can be refined)
            if options.remove_vowels {
                result = remove_vowels(&result);
            }
        }
        NormalizationLevel::Minimal => {
            // Just Unicode normalization, keep everything
        }
    }

    // 5. Normalize kaf/khof variants
    if options.normalize_kaf {
        // Already standard, could add more kaf-specific logic here
    }

    result
}

/// Remove vowel points (nikud) from Hebrew/Yiddish text
fn remove_vowels(text: &str) -> String {
    text.chars()
        .filter(|c| !POINTS_TO_REMOVE.contains(c))
        .collect()
}

/// Normalize Latin/YIVO romanization for comparison
pub fn normalize_romanization(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| match c {
            // Replace similar-looking characters
            'ä' => 'a',
            'ö' => 'o',
            'ü' => 'u',
            'ë' => 'e',
            'ï' => 'i',
            'š' => 's',
            'ž' => 'z',
            // Remove apostrophes and other diacritics for comparison
            '\'' | 'ʼ' | 'ʻ' | '′' => ' ',
            _ => c,
        })
        .collect()
}

/// Create a search key from text (for indexing)
pub fn create_search_key(text: &str) -> String {
    let options = NormalizationOptions {
        level: NormalizationLevel::Aggressive,
        remove_vowels: true,
        normalize_final_letters: true,
        normalize_kaf: true,
    };

    normalize_yiddish(text, &options)
}

/// Calculate similarity ratio between two strings (for fuzzy matching)
pub fn similarity_ratio(s1: &str, s2: &str) -> f64 {
    let normalized1 = normalize_yiddish(s1, &NormalizationOptions::default());
    let normalized2 = normalize_yiddish(s2, &NormalizationOptions::default());

    if normalized1 == normalized2 {
        return 1.0;
    }

    // Levenshtein distance
    let len1 = normalized1.chars().count();
    let len2 = normalized2.chars().count();
    let distance = levenshtein_distance(&normalized1, &normalized2);

    let max_len = len1.max(len2);
    if max_len == 0 {
        return 1.0;
    }

    1.0 - (distance as f64 / max_len as f64)
}

/// Calculate Levenshtein distance between two strings
#[allow(clippy::needless_range_loop)]
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = [
                matrix[i - 1][j] + 1,        // deletion
                matrix[i][j - 1] + 1,        // insertion
                matrix[i - 1][j - 1] + cost, // substitution
            ]
            .into_iter()
            .min()
            .unwrap();
        }
    }

    matrix[len1][len2]
}

/// Check if two strings match with tolerance for fuzziness
pub fn fuzzy_match(query: &str, target: &str, threshold: f64) -> bool {
    similarity_ratio(query, target) >= threshold
}

/// Split text into searchable tokens (words)
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_final_letters() {
        let options = NormalizationOptions {
            normalize_final_letters: true,
            ..Default::default()
        };

        assert_eq!(normalize_yiddish("שבת", &options), "שבת");
        // Add more test cases
    }

    #[test]
    fn test_similarity_ratio() {
        assert!(similarity_ratio("שבת", "שבת") > 0.9);
    }

    #[test]
    fn test_romanization_normalization() {
        // Test that normalization handles diacritics and case
        assert_eq!(normalize_romanization("Farshteyn"), "farshteyn");
        assert_eq!(normalize_romanization("farštejn"), "farstejn");
    }
}
