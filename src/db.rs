// ============================================================================
// DATABASE MODULE - POSTGRESQL CONNECTION & QUERIES
// ============================================================================

use anyhow::Result;
use tokio_postgres::NoTls;
use uuid::Uuid;

// Import all model types
use super::cache::{CacheKey, CachedResult, QueryCache};
use super::models::{Lexeme, Register, Sense, UsageCategory, Wordform};

// ============================================================================
// DATABASE CONNECTION
// ============================================================================

pub struct Database {
    client: tokio_postgres::Client,
    pub cache: QueryCache,
}

impl Database {
    pub async fn connect() -> Result<Self> {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://601ere@localhost:5432/iddish".to_string());

        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        // Create cache with 1000 entry capacity
        let cache = QueryCache::new(1000);

        Ok(Self { client, cache })
    }

    /// Fuzzy search on canonical forms
    pub async fn fuzzy_search_canonical(
        &self,
        query: &str,
        _min_threshold: f64,
    ) -> Result<Vec<Lexeme>> {
        // Use trigram-based similarity search
        let sql = r#"
            SELECT
                id, canonical_hebrew, canonical_roman, canonical_ipa,
                part_of_speech, gender,
                skeleton_key, phonetic_key, skeleton_collision,
                romanization_status, romanization_confidence, ipa_confidence,
                romanization_source, romanization_version, origin,
                english_definition, status, analyzed,
                created_at, updated_at,
                example_count, pos_confidence, morphology_richness,
                etymology_completeness, last_llm_update, has_corpus_examples
            FROM linguayi_lexeme
            WHERE canonical_hebrew % $1
               OR canonical_roman % $2
            ORDER BY similarity(canonical_hebrew, $1) DESC,
                     similarity(canonical_roman, $2) DESC
            LIMIT 50
        "#;

        let rows = self.client.query(sql, &[&query, &query]).await?;

        let mut lexemes = Vec::new();
        for row in rows {
            lexemes.push(Lexeme {
                id: row.get::<_, Uuid>("id"),
                canonical_hebrew: row.get::<_, String>("canonical_hebrew"),
                canonical_roman: row.get::<_, String>("canonical_roman"),
                canonical_ipa: row.try_get::<_, String>("canonical_ipa").ok(),
                part_of_speech: row.get::<_, String>("part_of_speech"),
                gender: row.try_get::<_, String>("gender").ok(),
                register: Register::Neutral,
                usage_category: UsageCategory::Neutral,
                usage_frequency_score: None,
                is_borrowed: false,
                etymology_source_language: None,
                notes: None,
                skeleton_key: row.try_get::<_, String>("skeleton_key").ok(),
                phonetic_key: row.try_get::<_, String>("phonetic_key").ok(),
                skeleton_collision: row.get::<_, bool>("skeleton_collision"),
                romanization_status: row.get::<_, String>("romanization_status"),
                romanization_confidence: row.get::<_, f64>("romanization_confidence"),
                ipa_confidence: row.get::<_, f64>("ipa_confidence"),
                romanization_source: row.get::<_, String>("romanization_source"),
                romanization_version: row.get::<_, i32>("romanization_version"),
                origin: row.get::<_, String>("origin"),
                english_definition: row.try_get::<_, String>("english_definition").ok(),
                status: row.get::<_, String>("status"),
                analyzed: row.get::<_, bool>("analyzed"),
                created_at: row.get::<_, chrono::DateTime<chrono::Utc>>("created_at"),
                updated_at: row.get::<_, chrono::DateTime<chrono::Utc>>("updated_at"),
                wordform_count: None,
                sense_count: None,
                etymology_count: None,
                idiom_count: None,
                example_count: row.try_get::<_, i32>("example_count").unwrap_or(0),
                pos_confidence: row.try_get::<_, f32>("pos_confidence").unwrap_or(1.0),
                morphology_richness: row.try_get::<_, f32>("morphology_richness").unwrap_or(0.0),
                etymology_completeness: row
                    .try_get::<_, f32>("etymology_completeness")
                    .unwrap_or(0.0),
                last_llm_update: row
                    .try_get::<_, chrono::DateTime<chrono::Utc>>("last_llm_update")
                    .ok(),
                has_corpus_examples: row
                    .try_get::<_, bool>("has_corpus_examples")
                    .unwrap_or(false),
                semantic_tags: vec![],
                cross_references: serde_json::json!({}),
            });
        }

        Ok(lexemes)
    }

    pub async fn get_stats(&self) -> Result<(i64, i64, i64)> {
        let lexeme_row = self
            .client
            .query_one("SELECT COUNT(*) FROM linguayi_lexeme", &[])
            .await?;
        let lexeme_count: i64 = lexeme_row.get(0);

        let wordform_row = self
            .client
            .query_one("SELECT COUNT(*) FROM linguayi_wordform", &[])
            .await?;
        let wordform_count: i64 = wordform_row.get(0);

        let sense_row = self
            .client
            .query_one("SELECT COUNT(*) FROM linguayi_sense", &[])
            .await?;
        let sense_count: i64 = sense_row.get(0);

        Ok((lexeme_count, wordform_count, sense_count))
    }

    // ========================================================================
    // LEXEME QUERIES
    // ========================================================================

    pub async fn lookup_lexeme(&self, query: &str) -> Result<Vec<Lexeme>> {
        let key = CacheKey::Lookup(query.to_string());

        if let Some(CachedResult::Lexemes(lexemes)) = self.cache.get(&key) {
            return Ok(lexemes);
        }

        let sql = r#"
            SELECT
                id, canonical_hebrew, canonical_roman, canonical_ipa,
                part_of_speech, gender,
                skeleton_key, phonetic_key, skeleton_collision,
                romanization_status, romanization_confidence, ipa_confidence,
                romanization_source, romanization_version, origin,
                english_definition, status, analyzed,
                created_at, updated_at,
                example_count, pos_confidence, morphology_richness,
                etymology_completeness, last_llm_update, has_corpus_examples
            FROM linguayi_lexeme
            WHERE canonical_hebrew ILIKE $1
               OR canonical_roman ILIKE $1
            ORDER BY romanization_confidence DESC
            LIMIT 20
        "#;

        let query_pattern = format!("%{}%", query);
        let rows = self.client.query(sql, &[&query_pattern]).await?;

        let mut lexemes = Vec::new();
        for row in rows {
            lexemes.push(Lexeme {
                id: row.get::<_, Uuid>("id"),
                canonical_hebrew: row.get::<_, String>("canonical_hebrew"),
                canonical_roman: row.get::<_, String>("canonical_roman"),
                canonical_ipa: row.try_get::<_, String>("canonical_ipa").ok(),
                part_of_speech: row.get::<_, String>("part_of_speech"),
                gender: row.try_get::<_, String>("gender").ok(),
                register: Register::Neutral,
                usage_category: UsageCategory::Neutral,
                usage_frequency_score: None,
                is_borrowed: false,
                etymology_source_language: None,
                notes: None,
                skeleton_key: row.try_get::<_, String>("skeleton_key").ok(),
                phonetic_key: row.try_get::<_, String>("phonetic_key").ok(),
                skeleton_collision: row.get::<_, bool>("skeleton_collision"),
                romanization_status: row.get::<_, String>("romanization_status"),
                romanization_confidence: row.get::<_, f64>("romanization_confidence"),
                ipa_confidence: row.get::<_, f64>("ipa_confidence"),
                romanization_source: row.get::<_, String>("romanization_source"),
                romanization_version: row.get::<_, i32>("romanization_version"),
                origin: row.get::<_, String>("origin"),
                english_definition: row.try_get::<_, String>("english_definition").ok(),
                status: row.get::<_, String>("status"),
                analyzed: row.get::<_, bool>("analyzed"),
                created_at: row.get::<_, chrono::DateTime<chrono::Utc>>("created_at"),
                updated_at: row.get::<_, chrono::DateTime<chrono::Utc>>("updated_at"),
                wordform_count: None,
                sense_count: None,
                etymology_count: None,
                idiom_count: None,
                example_count: row.try_get::<_, i32>("example_count").unwrap_or(0),
                pos_confidence: row.try_get::<_, f32>("pos_confidence").unwrap_or(1.0),
                morphology_richness: row.try_get::<_, f32>("morphology_richness").unwrap_or(0.0),
                etymology_completeness: row
                    .try_get::<_, f32>("etymology_completeness")
                    .unwrap_or(0.0),
                last_llm_update: row
                    .try_get::<_, chrono::DateTime<chrono::Utc>>("last_llm_update")
                    .ok(),
                has_corpus_examples: row
                    .try_get::<_, bool>("has_corpus_examples")
                    .unwrap_or(false),
                semantic_tags: vec![],
                cross_references: serde_json::json!({}),
            });
        }

        self.cache.put(key, CachedResult::Lexemes(lexemes.clone()));
        Ok(lexemes)
    }

    pub async fn get_lexeme_details(
        &self,
        lexeme_id: Uuid,
    ) -> Result<(Lexeme, Vec<Wordform>, Vec<Sense>)> {
        let key = CacheKey::LexemeDetails(lexeme_id);

        if let Some(CachedResult::LexemeDetails(details)) = self.cache.get(&key) {
            return Ok((*details).clone());
        }

        let lexeme = self.get_lexeme(lexeme_id).await?;
        let wordforms = self.get_wordforms(lexeme_id).await?;
        let senses = self.get_senses(lexeme_id).await?;

        let result = (lexeme, wordforms, senses);
        self.cache
            .put(key, CachedResult::LexemeDetails(Box::new(result.clone())));

        Ok(result)
    }

    async fn get_lexeme(&self, lexeme_id: Uuid) -> Result<Lexeme> {
        let sql = r#"
            SELECT
                id, canonical_hebrew, canonical_roman, canonical_ipa,
                part_of_speech, gender,
                skeleton_key, phonetic_key, skeleton_collision,
                romanization_status, romanization_confidence, ipa_confidence,
                romanization_source, romanization_version, origin,
                english_definition, status, analyzed,
                created_at, updated_at,
                example_count, pos_confidence, morphology_richness,
                etymology_completeness, last_llm_update, has_corpus_examples
            FROM linguayi_lexeme
            WHERE id = $1
        "#;

        let row = self.client.query_one(sql, &[&lexeme_id]).await?;

        Ok(Lexeme {
            id: row.get::<_, Uuid>("id"),
            canonical_hebrew: row.get::<_, String>("canonical_hebrew"),
            canonical_roman: row.get::<_, String>("canonical_roman"),
            canonical_ipa: row.try_get::<_, String>("canonical_ipa").ok(),
            part_of_speech: row.get::<_, String>("part_of_speech"),
            gender: row.try_get::<_, String>("gender").ok(),
            register: Register::Neutral,
            usage_category: UsageCategory::Neutral,
            usage_frequency_score: None,
            is_borrowed: false,
            etymology_source_language: None,
            notes: None,
            skeleton_key: row.try_get::<_, String>("skeleton_key").ok(),
            phonetic_key: row.try_get::<_, String>("phonetic_key").ok(),
            skeleton_collision: row.get::<_, bool>("skeleton_collision"),
            romanization_status: row.get::<_, String>("romanization_status"),
            romanization_confidence: row.get::<_, f64>("romanization_confidence"),
            ipa_confidence: row.get::<_, f64>("ipa_confidence"),
            romanization_source: row.get::<_, String>("romanization_source"),
            romanization_version: row.get::<_, i32>("romanization_version"),
            origin: row.get::<_, String>("origin"),
            english_definition: row.try_get::<_, String>("english_definition").ok(),
            status: row.get::<_, String>("status"),
            analyzed: row.get::<_, bool>("analyzed"),
            created_at: row.get::<_, chrono::DateTime<chrono::Utc>>("created_at"),
            updated_at: row.get::<_, chrono::DateTime<chrono::Utc>>("updated_at"),
            wordform_count: None,
            sense_count: None,
            etymology_count: None,
            idiom_count: None,
            example_count: row.try_get::<_, i32>("example_count").unwrap_or(0),
            pos_confidence: row.try_get::<_, f32>("pos_confidence").unwrap_or(1.0),
            morphology_richness: row.try_get::<_, f32>("morphology_richness").unwrap_or(0.0),
            etymology_completeness: row
                .try_get::<_, f32>("etymology_completeness")
                .unwrap_or(0.0),
            last_llm_update: row
                .try_get::<_, chrono::DateTime<chrono::Utc>>("last_llm_update")
                .ok(),
            has_corpus_examples: row
                .try_get::<_, bool>("has_corpus_examples")
                .unwrap_or(false),
            semantic_tags: vec![],
            cross_references: serde_json::json!({}),
        })
    }

    async fn get_wordforms(&self, lexeme_id: Uuid) -> Result<Vec<Wordform>> {
        let sql = r#"
            SELECT
                id, lexeme_id, text, is_standard, script, dialect,
                is_verified, origin, is_canonical_lk,
                person, grammatical_number, grammatical_gender,
                tense, is_infinitive, is_inflected_form, is_explicit_plural
            FROM linguayi_wordform
            WHERE lexeme_id = $1
            ORDER BY is_standard DESC, text
        "#;

        let rows = self.client.query(sql, &[&lexeme_id]).await?;

        let mut wordforms = Vec::new();
        for row in rows {
            wordforms.push(Wordform {
                id: row.get::<_, i64>("id"),
                lexeme_id: row.get::<_, Uuid>("lexeme_id"),
                text: row.get::<_, String>("text"),
                script: row.get::<_, String>("script"),
                morph_features: None,
                morph_features_id: None,
                is_standard: row.get::<_, bool>("is_standard"),
                is_canonical_lk: row.get::<_, bool>("is_canonical_lk"),
                is_verified: row.get::<_, bool>("is_verified"),
                is_inflected_form: row.get::<_, bool>("is_inflected_form"),
                is_infinitive: row.get::<_, bool>("is_infinitive"),
                is_explicit_plural: row.get::<_, bool>("is_explicit_plural"),
                base_form_id: None,
                is_derived: false,
                derivation_type: None,
                origin: row.get::<_, String>("origin"),
                dialect: row.try_get::<_, String>("dialect").ok(),
                dialect_specific: vec![],
                pronunciation_variant: None,
                corpus_frequency: 0,
                verified_by_id: None,
                verified_at: None,
            });
        }

        Ok(wordforms)
    }

    async fn get_senses(&self, lexeme_id: Uuid) -> Result<Vec<Sense>> {
        let sql = r#"
            SELECT id, lexeme_id, definition, definition_yi, flow_state
            FROM linguayi_sense
            WHERE lexeme_id = $1
        "#;

        let rows = self.client.query(sql, &[&lexeme_id]).await?;

        let mut senses = Vec::new();
        for row in rows {
            senses.push(Sense {
                id: row.get::<_, i64>("id"),
                lexeme_id: row.get::<_, Uuid>("lexeme_id"),
                definition_number: None,
                definition: row.get::<_, String>("definition"),
                definition_yi: row.try_get::<_, String>("definition_yi").ok(),
                definition_english: None,
                semantic_field: vec![],
                register_specific: None,
                dialect_specific: vec![],
                domain_specific: vec![],
                connotation: None,
                frequency_in_sense: None,
                is_primary_sense: false,
                flow_state: row.get::<_, String>("flow_state"),
                source_id: None,
                usage_notes: None,
                examples: vec![],
                relationships: vec![],
            });
        }

        Ok(senses)
    }
}
