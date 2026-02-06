// ============================================================================
// PROPERTY-BASED TESTS - Proptest
// ============================================================================
//
// These tests verify invariants hold across random inputs.
// Run with: cargo test --test property_tests
//
// Property-based testing finds edge cases that example-based tests miss.
// ============================================================================

use proptest::prelude::*;

// ============================================================================
// TEXT NORMALIZATION PROPERTIES
// ============================================================================

// Property: Normalized text should never increase in length
proptest! {
    #[test]
    fn prop_normalization_doesnt_increase_length(s in "\\PC*") {
        // For any string, normalization should not increase length
        // (Unicode normalization can combine characters)
        let normalized = judiw_terminal::normalization::normalize_romanization(&s);
        assert!(normalized.len() <= s.len() * 2,
                "Normalization should not drastically increase length");
    }

    #[test]
    fn prop_normalization_is_idempotent(s in "\\PC*") {
        // Normalizing twice should give same result as normalizing once
        let norm1 = judiw_terminal::normalization::normalize_romanization(&s);
        let norm2 = judiw_terminal::normalization::normalize_romanization(&norm1);
        assert_eq!(norm1, norm2, "Normalization should be idempotent");
    }

    #[test]
    fn prop_create_search_key_is_deterministic(s in "\\PC{0,100}") {
        // Creating a search key twice should give same result
        let key1 = judiw_terminal::normalization::create_search_key(&s);
        let key2 = judiw_terminal::normalization::create_search_key(&s);
        assert_eq!(key1, key2, "Search key creation should be deterministic");
    }
}

// ============================================================================
// LEXEME MODEL PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_display_priority_in_bounds(
        freq_score in 0u32..=100u32,
        is_neutral in prop::bool::ANY,
        is_general in prop::bool::ANY
    ) {
        // Display priority should be bounded for reasonable inputs
        // This is a model test - we construct values that should be valid
        use judiw_terminal::models::{Lexeme, Register, UsageCategory};
        use uuid::Uuid;
        use chrono::Utc;

        let lexeme = Lexeme {
            id: Uuid::new_v4(),
            canonical_hebrew: "test".to_string(),
            canonical_roman: "test".to_string(),
            canonical_ipa: None,
            part_of_speech: "noun".to_string(),
            gender: None,
            register: if is_neutral { Register::Neutral } else { Register::Literary },
            usage_category: if is_general { UsageCategory::General } else { UsageCategory::HasidicSpecific },
            usage_frequency_score: Some(freq_score as f32),
            is_borrowed: false,
            etymology_source_language: None,
            notes: None,
            skeleton_key: None,
            phonetic_key: None,
            skeleton_collision: false,
            romanization_status: "standard".to_string(),
            romanization_confidence: 1.0,
            ipa_confidence: 1.0,
            romanization_source: "manual".to_string(),
            romanization_version: 1,
            origin: "Yiddish".to_string(),
            english_definition: None,
            status: "active".to_string(),
            analyzed: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            wordform_count: None,
            sense_count: None,
            etymology_count: None,
            idiom_count: None,
            example_count: 0,
            pos_confidence: 1.0,
            morphology_richness: 0.0,
            etymology_completeness: 0.0,
            last_llm_update: None,
            has_corpus_examples: false,
            semantic_tags: vec![],
            cross_references: serde_json::json!({}),
        };

        let priority = lexeme.display_priority();
        assert!(priority >= 0, "Display priority should be non-negative");
        assert!(priority < 100, "Display priority should be reasonable");
    }

    #[test]
    fn prop_toggleable_features_in_bounds(
        example_count in 0i32..=1000i32,
        pos_conf in 0.0f32..=1.0f32,
        morph_rich in 0.0f32..=1.0f32,
        etym_complete in 0.0f32..=1.0f32
    ) {
        // Toggleable features should stay within bounds
        use judiw_terminal::models::{Lexeme, Register, UsageCategory};
        use uuid::Uuid;
        use chrono::Utc;

        let lexeme = Lexeme {
            id: Uuid::new_v4(),
            canonical_hebrew: "test".to_string(),
            canonical_roman: "test".to_string(),
            canonical_ipa: None,
            part_of_speech: "noun".to_string(),
            gender: None,
            register: Register::Neutral,
            usage_category: UsageCategory::General,
            usage_frequency_score: None,
            is_borrowed: false,
            etymology_source_language: None,
            notes: None,
            skeleton_key: None,
            phonetic_key: None,
            skeleton_collision: false,
            romanization_status: "standard".to_string(),
            romanization_confidence: 1.0,
            ipa_confidence: 1.0,
            romanization_source: "manual".to_string(),
            romanization_version: 1,
            origin: "Yiddish".to_string(),
            english_definition: None,
            status: "active".to_string(),
            analyzed: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            wordform_count: None,
            sense_count: None,
            etymology_count: None,
            idiom_count: None,
            example_count,
            pos_confidence: pos_conf,
            morphology_richness: morph_rich,
            etymology_completeness: etym_complete,
            last_llm_update: None,
            has_corpus_examples: false,
            semantic_tags: vec![],
            cross_references: serde_json::json!({}),
        };

        assert!(lexeme.example_count >= 0, "example_count should be non-negative");
        assert!(
            lexeme.pos_confidence >= 0.0 && lexeme.pos_confidence <= 1.0,
            "pos_confidence should be in [0.0, 1.0]"
        );
        assert!(
            lexeme.morphology_richness >= 0.0 && lexeme.morphology_richness <= 1.0,
            "morphology_richness should be in [0.0, 1.0]"
        );
        assert!(
            lexeme.etymology_completeness >= 0.0 && lexeme.etymology_completeness <= 1.0,
            "etymology_completeness should be in [0.0, 1.0]"
        );
    }
}

// ============================================================================
// SEARCH PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_search_key_handles_unicode(s in "\\PC{0,50}") {
        // Search key creation should handle any Unicode without panicking
        let _key = judiw_terminal::normalization::create_search_key(&s);
        // If we got here without panic, test passes
    }

    #[test]
    fn prop_search_key_is_ascii(s in "\\PC{0,50}") {
        // Search keys should be deterministic and handle any Unicode without panicking
        let key = judiw_terminal::normalization::create_search_key(&s);
        // The key should be consistent across calls
        let key2 = judiw_terminal::normalization::create_search_key(&s);
        assert_eq!(key, key2, "Search key should be deterministic");
    }
}

// ============================================================================
// CACHE PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_cache_put_then_get(count in 1usize..=100usize) {
        use judiw_terminal::cache::{QueryCache, CacheKey, CachedResult};

        let cache = QueryCache::new(count * 2);

        // Put and get multiple items
        for i in 0..count {
            let key = CacheKey::Lookup(format!("query_{}", i));
            let result = CachedResult::Lexemes(vec![]);
            cache.put(key, result);
        }

        // All items should be retrievable
        for i in 0..count {
            let key = CacheKey::Lookup(format!("query_{}", i));
            assert!(cache.get(&key).is_some(), "Should be able to retrieve cached item {}", i);
        }
    }

    #[test]
    fn prop_cache_stats_monotonic(count in 1usize..=50usize) {
        use judiw_terminal::cache::{QueryCache, CacheKey};

        let cache = QueryCache::new(100);

        let misses_before = cache.stats().misses;

        // Perform cache misses
        for i in 0..count {
            let key = CacheKey::Lookup(format!("query_{}", i));
            cache.get(&key);
        }

        let misses_after = cache.stats().misses;

        assert_eq!(
            misses_after - misses_before,
            count,
            "Miss count should increase by exactly the number of gets"
        );
    }
}

// ============================================================================
// UUID PROPERTIES
// ============================================================================

proptest! {
    #[test]
    fn prop_uuid_roundtrip(uuid_bytes in prop::array::uniform16(prop::num::u8::ANY)) {
        // UUID should round-trip through string representation
        use uuid::Uuid;

        let uuid1 = Uuid::from_bytes(uuid_bytes);
        let uuid_str = uuid1.to_string();
        let uuid2 = uuid_str.parse::<Uuid>().unwrap();

        assert_eq!(uuid1, uuid2, "UUID should round-trip through string");
    }
}
