use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================================
// DATA MODELS - Mirror Django models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lexeme {
    pub id: Uuid,
    pub canonical_hebrew: String,
    pub canonical_roman: Option<String>,
    pub canonical_ipa: Option<String>,
    pub skeleton_key: Option<String>,
    pub phonetic_key: Option<String>,
    pub skeleton_collision: bool,
    pub romanization_status: String,
    pub romanization_confidence: f64,
    pub ipa_confidence: f64,
    pub romanization_source: String,
    pub romanization_version: i32,
    pub part_of_speech: Option<String>,
    pub gender: Option<String>,
    pub origin: String,
    pub english_definition: Option<String>,
    pub status: String,
    pub analyzed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wordform {
    pub id: i64,
    pub lexeme_id: Uuid,
    pub text: String,
    pub is_standard: bool,
    pub script: String,
    pub dialect: Option<String>,
    pub is_verified: bool,
    pub origin: String,
    pub is_canonical_lk: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sense {
    pub id: i64,
    pub lexeme_id: Uuid,
    pub definition: String,
    pub definition_yi: Option<String>,
    pub flow_state: String,
}

// ============================================================================
// DATABASE CONNECTION
// ============================================================================

pub struct Database {
    client: tokio_postgres::Client,
}

impl Database {
    pub async fn connect() -> anyhow::Result<Self> {
        // Load environment variables
        dotenv::dotenv().ok();
        
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres@localhost:5432/iddish".to_string());
        
        let client = tokio_postgres::connect(&database_url).await?;
        
        Ok(Self { client })
    }
    
    pub async fn lookup_lexeme(&self, query: &str) -> anyhow::Result<Vec<Lexeme>> {
        let sql = r#"
            SELECT 
                id, canonical_hebrew, canonical_roman, canonical_ipa,
                skeleton_key, phonetic_key, skeleton_collision,
                romanization_status, romanization_confidence, ipa_confidence,
                romanization_source, romanization_version, part_of_speech,
                gender, origin, english_definition, status, analyzed,
                created_at, updated_at
            FROM linguayi_lexeme
            WHERE canonical_hebrew LIKE $1 
               OR canonical_roman LIKE $1
            ORDER BY romanization_confidence DESC
            LIMIT 20
        "#;
        
        let query_pattern = format!("%{}%", query);
        
        let rows = self.client.query(sql, &[&query_pattern]).await?;
        
        let mut lexemes = Vec::new();
        for row in rows {
            lexemes.push(Lexeme {
                id: row.get::<&str, _>("id")?,
                canonical_hebrew: row.get("canonical_hebrew")?,
                canonical_roman: row.get("canonical_roman")?,
                canonical_ipa: row.get("canonical_ipa")?,
                skeleton_key: row.get("skeleton_key")?,
                phonetic_key: row.get("phonetic_key")?,
                skeleton_collision: row.get("skeleton_collision")?,
                romanization_status: row.get("romanization_status")?,
                romanization_confidence: row.get("romanization_confidence")?,
                ipa_confidence: row.get("ipa_confidence")?,
                romanization_source: row.get("romanization_source")?,
                romanization_version: row.get("romanization_version")?,
                part_of_speech: row.get("part_of_speech")?,
                gender: row.get("gender")?,
                origin: row.get("origin")?,
                english_definition: row.get("english_definition")?,
                status: row.get("status")?,
                analyzed: row.get("analyzed")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            });
        }
        
        Ok(lexemes)
    }
    
    pub async fn get_lexeme_details(&self, lexeme_id: Uuid) -> anyhow::Result<(Lexeme, Vec<Wordform>, Vec<Sense>)> {
        // Get lexeme
        let lexeme_sql = r#"
            SELECT 
                id, canonical_hebrew, canonical_roman, canonical_ipa,
                skeleton_key, phonetic_key, skeleton_collision,
                romanization_status, romanization_confidence, ipa_confidence,
                romanization_source, romanization_version, part_of_speech,
                gender, origin, english_definition, status, analyzed,
                created_at, updated_at
            FROM linguayi_lexeme
            WHERE id = $1
        "#;
        
        let lexeme_rows = self.client.query_one(lexeme_sql, &[&lexeme_id]).await?;
        
        let lexeme = Lexeme {
            id: lexeme_rows.get::<&str, _>("id")?,
            canonical_hebrew: lexeme_rows.get("canonical_hebrew")?,
            canonical_roman: lexeme_rows.get("canonical_roman")?,
            canonical_ipa: lexeme_rows.get("canonical_ipa")?,
            skeleton_key: lexeme_rows.get("skeleton_key")?,
            phonetic_key: lexeme_rows.get("phonetic_key")?,
            skeleton_collision: lexeme_rows.get("skeleton_collision")?,
            romanization_status: lexeme_rows.get("romanization_status")?,
            romanization_confidence: lexeme_rows.get("romanization_confidence")?,
            ipa_confidence: lexeme_rows.get("ipa_confidence")?,
            romanization_source: lexeme_rows.get("romanization_source")?,
            romanization_version: lexeme_rows.get("romanization_version")?,
            part_of_speech: lexeme_rows.get("part_of_speech")?,
            gender: lexeme_rows.get("gender")?,
            origin: lexeme_rows.get("origin")?,
            english_definition: lexeme_rows.get("english_definition")?,
            status: lexeme_rows.get("status")?,
            analyzed: lexeme_rows.get("analyzed")?,
            created_at: lexeme_rows.get("created_at")?,
            updated_at: lexeme_rows.get("updated_at")?,
        };
        
        // Get wordforms
        let wordform_sql = r#"
            SELECT id, lexeme_id, text, is_standard, script, dialect,
                   is_verified, origin, is_canonical_lk
            FROM linguayi_wordform
            WHERE lexeme_id = $1
            ORDER BY is_standard DESC, text
        "#;
        
        let wordform_rows = self.client.query(wordform_sql, &[&lexeme_id]).await?;
        
        let mut wordforms = Vec::new();
        for row in wordform_rows {
            wordforms.push(Wordform {
                id: row.get("id")?,
                lexeme_id: row.get("lexeme_id")?,
                text: row.get("text")?,
                is_standard: row.get("is_standard")?,
                script: row.get("script")?,
                dialect: row.get("dialect")?,
                is_verified: row.get("is_verified")?,
                origin: row.get("origin")?,
                is_canonical_lk: row.get("is_canonical_lk")?,
            });
        }
        
        // Get senses
        let sense_sql = r#"
            SELECT id, lexeme_id, definition, definition_yi, flow_state
            FROM linguayi_sense
            WHERE lexeme_id = $1
        "#;
        
        let sense_rows = self.client.query(sense_sql, &[&lexeme_id]).await?;
        
        let mut senses = Vec::new();
        for row in sense_rows {
            senses.push(Sense {
                id: row.get("id")?,
                lexeme_id: row.get("lexeme_id")?,
                definition: row.get("definition")?,
                definition_yi: row.get("definition_yi")?,
                flow_state: row.get("flow_state")?,
            });
        }
        
        Ok((lexeme, wordforms, senses))
    }
}