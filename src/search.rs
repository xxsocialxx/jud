// ============================================================================
// FUZZY SEARCH INFRASTRUCTURE
// ============================================================================

#![allow(dead_code)]

use crate::models::Lexeme;
use crate::normalization::{
    create_search_key, normalize_romanization, normalize_yiddish, NormalizationLevel,
    NormalizationOptions,
};
use anyhow::Result;

/// Search result with relevance score
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub lexeme: Lexeme,
    pub relevance_score: f64,
    pub matched_in: Vec<MatchField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchField {
    CanonicalHebrew,
    CanonicalRoman,
    Wordform(String),
    SenseDefinition,
    SemanticTag(String),
}

/// Search configuration
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub fuzzy_threshold: f64,
    pub max_results: usize,
    pub include_wordforms: bool,
    pub include_senses: bool,
    pub search_semantic_tags: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            fuzzy_threshold: 0.6,
            max_results: 20,
            include_wordforms: true,
            include_senses: true,
            search_semantic_tags: true,
        }
    }
}

/// Search database for lexemes matching query
pub async fn search_lexemes(
    db: &crate::db::Database,
    query: &str,
    options: &SearchOptions,
) -> Result<Vec<SearchResult>> {
    let mut results = Vec::new();

    // Try exact match first (fast path)
    let exact_matches = db.lookup_lexeme(query).await?;

    for lexeme in exact_matches {
        results.push(SearchResult {
            relevance_score: 1.0,
            matched_in: vec![MatchField::CanonicalHebrew],
            lexeme,
        });
    }

    // If we have enough exact matches, return early
    if results.len() >= options.max_results {
        return Ok(results);
    }

    // Fuzzy search on canonical forms
    let fuzzy_lexemes = db.fuzzy_search_canonical(query, 0.7).await?;

    for lexeme in fuzzy_lexemes {
        // Avoid duplicates
        if results.iter().any(|r| r.lexeme.id == lexeme.id) {
            continue;
        }

        let score = calculate_relevance(&lexeme, query);
        if score >= options.fuzzy_threshold {
            results.push(SearchResult {
                relevance_score: score,
                matched_in: vec![MatchField::CanonicalRoman],
                lexeme,
            });
        }
    }

    // Sort by relevance
    results.sort_by(|a, b| {
        b.relevance_score
            .partial_cmp(&a.relevance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Limit results
    results.truncate(options.max_results);

    Ok(results)
}

/// Calculate relevance score for a lexeme relative to query
fn calculate_relevance(lexeme: &Lexeme, query: &str) -> f64 {
    let mut score = 0.0;

    // Match in canonical Hebrew (highest weight)
    let hebrew_key = create_search_key(&lexeme.canonical_hebrew);
    let query_key = create_search_key(query);
    if hebrew_key.contains(&query_key) || query_key.contains(&hebrew_key) {
        score += 0.9;
    }

    // Match in canonical romanization
    let roman_normalized = normalize_romanization(&lexeme.canonical_roman);
    let query_normalized = normalize_romanization(query);
    if roman_normalized.contains(&query_normalized) || query_normalized.contains(&roman_normalized)
    {
        score += 0.8;
    }

    // Partial match bonus
    if lexeme.canonical_hebrew.starts_with(query) || lexeme.canonical_roman.starts_with(query) {
        score += 0.1;
    }

    // Frequency score boost (common words are more relevant)
    if let Some(freq) = lexeme.usage_frequency_score {
        score += ((freq as f64) / 1000.0).min(0.1);
    }

    score.min(1.0)
}

/// Normalize query for database search
pub fn prepare_search_query(query: &str) -> (String, String, String) {
    // Hebrew version
    let hebrew_query = normalize_yiddish(
        query,
        &NormalizationOptions {
            level: NormalizationLevel::Moderate,
            remove_vowels: false,
            normalize_final_letters: true,
            normalize_kaf: true,
        },
    );

    // Romanization version
    let roman_query = normalize_romanization(query);

    // Aggressive version (for fuzzy matching)
    let fuzzy_query = create_search_key(query);

    (hebrew_query, roman_query, fuzzy_query)
}

/// Format search results for display
pub fn format_search_results(results: &[SearchResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }

    let mut output = String::new();

    for (i, result) in results.iter().enumerate() {
        let confidence = if result.relevance_score >= 0.9 {
            "✓"
        } else if result.relevance_score >= 0.7 {
            "~"
        } else {
            "?"
        };

        output.push_str(&format!(
            "{}.\n{} {} ({})\n",
            i + 1,
            confidence,
            result.lexeme.canonical_hebrew,
            result.lexeme.canonical_roman
        ));

        if let Some(def) = &result.lexeme.english_definition {
            output.push_str(&format!("   {}\n", def));
        }

        if result.relevance_score < 1.0 {
            output.push_str(&format!(
                "   [Relevance: {:.0}%]\n",
                result.relevance_score * 100.0
            ));
        }

        output.push('\n');
    }

    output.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_search_query() {
        let (hebrew, roman, _fuzzy) = prepare_search_query("שבת");
        assert!(!hebrew.is_empty());
        assert!(!roman.is_empty());
    }
}
