// ============================================================================
// INTEGRATION TESTS - Database Operations
// ============================================================================
//
// These tests verify the foundational database operations work correctly.
// Run with: cargo test --test integration_test
//
// Requires: PostgreSQL database running at DATABASE_URL
// ============================================================================

use judiw_terminal::db::Database;
use uuid::Uuid;

// ============================================================================
// TEST HELPERS
// ============================================================================

async fn get_test_db() -> Database {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://601ere@localhost:5432/iddish".to_string());

    // Test connection first
    let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .expect("Failed to connect to test database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Verify we can query the database
    client
        .query_one("SELECT 1", &[])
        .await
        .expect("Failed to execute test query");

    drop(client);

    Database::connect()
        .await
        .expect("Failed to create database connection")
}

// ============================================================================
// DATABASE CONNECTION TESTS
// ============================================================================

#[tokio::test]
async fn test_database_connection() {
    let db = get_test_db().await;

    // Test that we can execute a simple query
    let result = db.get_stats().await;
    assert!(result.is_ok(), "get_stats should succeed");

    let (lexeme_count, wordform_count, sense_count) = result.unwrap();
    assert!(lexeme_count > 0, "Should have lexemes in database");
    assert!(wordform_count > 0, "Should have wordforms in database");
    assert!(sense_count > 0, "Should have senses in database");

    println!(
        "✅ Database connection test passed: {} lexemes, {} wordforms, {} senses",
        lexeme_count, wordform_count, sense_count
    );
}

// ============================================================================
// LEXEME QUERY TESTS
// ============================================================================

#[tokio::test]
async fn test_lookup_lexeme() {
    let db = get_test_db().await;

    // Test looking up a common Yiddish word
    let result = db.lookup_lexeme("god").await;
    assert!(result.is_ok(), "lookup_lexeme should succeed");

    let lexemes = result.unwrap();
    assert!(
        !lexemes.is_empty(),
        "Should find at least one lexeme for 'god'"
    );

    let first = &lexemes[0];
    assert!(
        !first.canonical_hebrew.is_empty(),
        "Lexeme should have Hebrew text"
    );
    assert!(
        !first.canonical_roman.is_empty(),
        "Lexeme should have romanization"
    );
    assert!(!first.part_of_speech.is_empty(), "Lexeme should have POS");

    // Verify toggleable feature columns exist
    assert!(
        first.example_count >= 0,
        "example_count should be non-negative"
    );
    assert!(
        first.pos_confidence >= 0.0 && first.pos_confidence <= 1.0,
        "pos_confidence should be between 0.0 and 1.0"
    );
    assert!(
        first.morphology_richness >= 0.0 && first.morphology_richness <= 1.0,
        "morphology_richness should be between 0.0 and 1.0"
    );
    assert!(
        first.etymology_completeness >= 0.0 && first.etymology_completeness <= 1.0,
        "etymology_completeness should be between 0.0 and 1.0"
    );

    println!(
        "✅ Lookup test passed: found {} lexeme(s), first has example_count={}, pos_confidence={}",
        lexemes.len(),
        first.example_count,
        first.pos_confidence
    );
}

#[tokio::test]
async fn test_lookup_lexeme_not_found() {
    let db = get_test_db().await;

    // Test looking up a non-existent word
    let result = db.lookup_lexeme("xyznonexistent123").await;
    assert!(
        result.is_ok(),
        "lookup_lexeme should succeed even if no results"
    );

    let lexemes = result.unwrap();
    assert!(
        lexemes.is_empty(),
        "Should find no lexemes for non-existent word"
    );

    println!("✅ Not found test passed: correctly returns empty results");
}

#[tokio::test]
async fn test_get_lexeme_details() {
    let db = get_test_db().await;

    // First get a lexeme ID from lookup
    let lexemes = db.lookup_lexeme("god").await.unwrap();
    assert!(!lexemes.is_empty(), "Should find at least one lexeme");

    let lexeme_id = lexemes[0].id;

    // Now get full details
    let result = db.get_lexeme_details(lexeme_id).await;
    assert!(result.is_ok(), "get_lexeme_details should succeed");

    let (lexeme, wordforms, senses) = result.unwrap();

    assert_eq!(lexeme.id, lexeme_id, "Should return correct lexeme");
    assert!(
        !lexeme.canonical_hebrew.is_empty(),
        "Lexeme should have Hebrew text"
    );

    // Wordforms and senses might be empty, but the result should succeed
    println!(
        "✅ Get details test passed: lexeme has {} wordform(s), {} sense(s)",
        wordforms.len(),
        senses.len()
    );
}

#[tokio::test]
async fn test_get_lexeme_details_invalid_uuid() {
    let db = get_test_db().await;

    // Test with an invalid UUID
    let invalid_uuid = Uuid::new_v4(); // Random UUID, likely doesn't exist
    let result = db.get_lexeme_details(invalid_uuid).await;

    // This might fail or return empty, depending on implementation
    // The important thing is it shouldn't panic
    assert!(result.is_err() || result.unwrap().0.canonical_hebrew.is_empty());

    println!("✅ Invalid UUID test passed: handles non-existent lexeme gracefully");
}

// ============================================================================
// CACHE TESTS
// ============================================================================

#[tokio::test]
async fn test_cache_hit() {
    let db = get_test_db().await;

    // First query should cache the result
    let query = "test_cache_hit";
    let result1 = db.lookup_lexeme(query).await.unwrap();

    // Get cache stats before second query
    let stats_before = db.cache.stats();

    // Second query should hit cache
    let result2 = db.lookup_lexeme(query).await.unwrap();

    // Results should be identical
    assert_eq!(
        result1.len(),
        result2.len(),
        "Cached result should be identical"
    );

    // Cache should have more hits
    let stats_after = db.cache.stats();
    assert_eq!(
        stats_after.hits,
        stats_before.hits + 1,
        "Cache hit count should increase"
    );

    println!(
        "✅ Cache test passed: hit rate {:.1}%",
        db.cache.hit_rate() * 100.0
    );
}

// ============================================================================
// TOGGLEABLE FEATURES TESTS
// ============================================================================

#[tokio::test]
async fn test_toggleable_features_exist() {
    let db = get_test_db().await;

    let lexemes = db.lookup_lexeme("a").await.unwrap();
    assert!(!lexemes.is_empty(), "Should find at least one lexeme");

    let lexeme = &lexemes[0];

    // All toggleable features should have values
    assert!(lexeme.example_count >= 0, "example_count should exist");
    assert!(
        lexeme.pos_confidence >= 0.0 && lexeme.pos_confidence <= 1.0,
        "pos_confidence should be valid"
    );
    assert!(
        lexeme.morphology_richness >= 0.0 && lexeme.morphology_richness <= 1.0,
        "morphology_richness should be valid"
    );
    assert!(
        lexeme.etymology_completeness >= 0.0 && lexeme.etymology_completeness <= 1.0,
        "etymology_completeness should be valid"
    );

    println!("✅ Toggleable features test passed: all columns exist and valid");
}

#[tokio::test]
async fn test_initial_toggleable_values() {
    let db = get_test_db().await;

    // For newly migrated columns, check initial values
    let lexemes = db.lookup_lexeme("a").await.unwrap();
    let lexeme = &lexemes[0];

    // Initial values should be defaults (no examples, full confidence, no coverage)
    assert_eq!(lexeme.example_count, 0, "Initial example_count should be 0");
    assert_eq!(
        lexeme.pos_confidence, 1.0,
        "Initial pos_confidence should be 1.0"
    );
    assert_eq!(
        lexeme.morphology_richness, 0.0,
        "Initial morphology_richness should be 0.0"
    );
    assert_eq!(
        lexeme.etymology_completeness, 0.0,
        "Initial etymology_completeness should be 0.0"
    );
    assert!(
        !lexeme.has_corpus_examples,
        "Initial has_corpus_examples should be false"
    );

    println!("✅ Initial values test passed: all toggleable features at expected defaults");
}

// ============================================================================
// FUZZY SEARCH TESTS
// ============================================================================

#[tokio::test]
async fn test_fuzzy_search() {
    let db = get_test_db().await;

    let result = db.fuzzy_search_canonical("god", 0.5).await;
    assert!(result.is_ok(), "fuzzy_search should succeed");

    let lexemes = result.unwrap();
    assert!(!lexemes.is_empty(), "Should find lexemes with fuzzy search");

    println!(
        "✅ Fuzzy search test passed: found {} lexeme(s)",
        lexemes.len()
    );
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_query_performance() {
    let db = get_test_db().await;

    let start = std::time::Instant::now();

    // Execute multiple queries
    for _ in 0..10 {
        let _ = db.lookup_lexeme("a").await.unwrap();
    }

    let duration = start.elapsed();

    // All queries should complete in reasonable time (< 1 second total)
    assert!(
        duration.as_secs() < 1,
        "10 queries should complete in less than 1 second"
    );

    println!(
        "✅ Performance test passed: 10 queries in {:?} ({:?} per query)",
        duration,
        duration / 10
    );
}
