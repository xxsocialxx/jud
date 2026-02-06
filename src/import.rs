// ============================================================================
// CSV IMPORT INFRASTRUCTURE
// ============================================================================

use crate::models::{Lexeme, Wordform, Sense, Etymology};
use crate::error::{JudiwError, Result};
use crate::validation::SchemaValidator;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use tracing::{info, warn, error};

/// CSV import configuration
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub validate_schemas: bool,
    pub dry_run: bool,
    pub batch_size: usize,
    pub stop_on_error: bool,
    pub skip_duplicates: bool,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            validate_schemas: true,
            dry_run: false,
            batch_size: 1000,
            stop_on_error: false,
            skip_duplicates: true,
        }
    }
}

/// Import statistics
#[derive(Debug, Clone, Default)]
pub struct ImportStats {
    pub lexemes_imported: usize,
    pub wordforms_imported: usize,
    pub senses_imported: usize,
    pub etymologies_imported: usize,
    pub errors: Vec<ImportError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImportError {
    pub row: usize,
    pub column: String,
    pub message: String,
}

/// CSV importer for Weinreich dictionary data
pub struct CsvImporter {
    config: ImportConfig,
    validator: SchemaValidator,
    stats: ImportStats,
}

impl CsvImporter {
    pub fn new(config: ImportConfig) -> Self {
        Self {
            config: config.clone(),
            validator: SchemaValidator::new(),
            stats: ImportStats::default(),
        }
    }

    /// Import from CSV file
    pub fn import_csv<P: AsRef<Path>>(&mut self, filepath: P) -> Result<ImportStats> {
        info!(path = %filepath.as_ref().display(), "Starting CSV import");

        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        // Detect CSV format (header row)
        let mut lines = reader.lines();
        let header = lines.next()
            .ok_or_else(|| JudiwError::ParseError {
                what: "CSV header".to_string(),
                input: "".to_string(),
                error: "Empty file".into(),
            })??;

        let format = CsvFormat::detect(&header)?;

        match format {
            CsvFormat::WeinreichHeadwords => self.import_weinreich_headwords(lines),
            CsvFormat::WeinreichRegister => self.import_weinreich_register(lines),
            CsvFormat::Unknown => {
                return Err(JowiError::Config(format!(
                    "Unknown CSV format. Header: {}",
                    header
                )))
            }
        }
    }

    fn import_weinreich_headwords(
        &mut self,
        lines: std::io::Lines<BufReader<File>>,
    ) -> Result<ImportStats> {
        info!("Importing Weinreich headwords");

        for (row_num, line_result) in lines.enumerate().skip(1) {
            let line = line_result.map_err(|e| JudiwError::Io {
                path: "STDIN".into(),
                error: e,
            })?;

            if line.trim().is_empty() {
                continue;
            }

            // Parse CSV row
            match self.parse_headword_row(&line, row_num) {
                Ok(Some(lexeme)) => {
                    if !self.config.dry_run {
                        // Insert into database
                        self.insert_lexeme(&lexeme)?;
                    }
                    self.stats.lexemes_imported += 1;
                }
                Ok(None) => {
                    // Skipped (duplicate, etc.)
                }
                Err(e) => {
                    let import_error = ImportError {
                        row: row_num,
                        column: "parse".to_string(),
                        message: e.to_string(),
                    };
                    
                    self.stats.errors.push(import_error);
                    
                    if self.config.stop_on_error {
                        return Err(JudiwError::Config(format!(
                            "Import stopped at row {} due to error",
                            row_num
                        )));
                    }
                }
            }

            // Batch commit
            if row_num % self.config.batch_size == 0 {
                info!(row = row_num, "Processed batch");
            }
        }

        Ok(self.stats.clone())
    }

    fn parse_headword_row(&self, line: &str, row_num: usize) -> Result<Option<Lexeme>> {
        let fields: Vec<&str> = line.split('|').collect();

        if fields.len() < 5 {
            return Err(JudiwError::ParseError {
                what: "headword row".to_string(),
                input: line.to_string(),
                error: "Insufficient fields".into(),
            });
        }

        let hebrew = fields[0].trim();
        let romanized = fields[1].trim();
        let pos = fields[2].trim();
        let gender = fields[3].trim();
        let etymology = fields[4].trim();

        // Validate non-empty
        if hebrew.is_empty() {
            return Ok(None);
        }

        let lexeme = Lexeme {
            id: uuid::Uuid::new_v4(),
            canonical_hebrew: hebrew.to_string(),
            canonical_roman: romanized.to_string(),
            canonical_ipa: None,
            part_of_speech: pos.to_string(),
            gender: if !gender.is_empty() { Some(gender.to_string()) } else { None },
            register: crate::models::shared::Register::Neutral,
            usage_category: crate::models::shared::UsageCategory::Neutral,
            usage_frequency_score: None,
            is_borrowed: !etymology.is_empty(),
            etymology_source_language: if !etymology.is_empty() {
                Some(etymology.to_string())
            } else {
                None
            },
            notes: None,
            skeleton_key: None,
            phonetic_key: None,
            skeleton_collision: false,
            romanization_status: "imported".to_string(),
            romanization_confidence: 0.8,
            ipa_confidence: 0.0,
            romanization_source: "weinreich_import".to_string(),
            romanization_version: 1,
            origin: "weinreich".to_string(),
            english_definition: None,
            status: "imported".to_string(),
            analyzed: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            wordform_count: None,
            sense_count: None,
            etymology_count: None,
            idiom_count: None,
            semantic_tags: vec![],
            cross_references: serde_json::json!({}),
        };

        // Validate schema
        if self.config.validate_schemas {
            self.validator.validate(&lexeme)?;
        }

        Ok(Some(lexeme))
    }

    fn insert_lexeme(&self, lexeme: &Lexeme) -> Result<()> {
        // Database insertion would go here
        info!(
            hebrew = %lexeme.canonical_hebrew,
            roman = %lexeme.canonical_roman,
            "Inserted lexeme"
        );
        Ok(())
    }

    fn import_weinreich_register(
        &mut self,
        _lines: std::io::Lines<BufReader<File>>,
    ) -> Result<ImportStats> {
        info!("Importing Weinreich register data (not implemented yet)");
        Ok(self.stats.clone())
    }
}

/// CSV format detection
#[derive(Debug, Clone, PartialEq, Eq)]
enum CsvFormat {
    WeinreichHeadwords,
    WeinreichRegister,
    Unknown,
}

impl CsvFormat {
    fn detect(header: &str) -> Result<Self> {
        let header_lower = header.to_lowercase();

        if header_lower.contains("headword") || header_lower.contains("yiddish") {
            Ok(Self::WeinreichHeadwords)
        } else if header_lower.contains("register") || header_lower.contains("dialect") {
            Ok(Self::WeinreichRegister)
        } else {
            Ok(Self::Unknown)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        let header = "Yiddish|Romanized|POS|Gender|Etymology";
        assert_eq!(
            CsvFormat::detect(header).unwrap(),
            CsvFormat::WeinreichHeadwords
        );
    }

    #[test]
    fn test_parse_headword() {
        let importer = CsvImporter::new(ImportConfig::default());
        let line = "שבת|shabes|Noun|Fem|Hebrew";

        let result = importer.parse_headword_row(line, 1);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
