// ============================================================================
// TYPE-SAFE ERROR HANDLING
// ============================================================================

use miette::{Diagnostic, SourceSpan};
use std::path::PathBuf;
use thiserror::Error;

/// Base error type for all Judiw errors
#[derive(Error, Diagnostic, Debug)]
pub enum JudiwError {
    /// Database connection or query errors
    #[error("Database error: {0}")]
    #[diagnostic(
        code(judiw::db::connection),
        help("Check your database connection string and ensure PostgreSQL is running")
    )]
    Database(#[from] tokio_postgres::Error),

    /// UUID parsing errors
    #[error("Invalid UUID: {0}")]
    #[diagnostic(
        code(judiw::validation::uuid),
        help("UUIDs must be in format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
    )]
    InvalidUuid(#[from] uuid::Error),

    /// Lexeme not found
    #[error("Lexeme not found: {id}")]
    #[diagnostic(
        code(judiw::db::not_found),
        help("Try searching for the lexeme first using the lookup command")
    )]
    LexemeNotFound { id: String },

    /// Wordform not found
    #[error("Wordform not found: {id}")]
    #[diagnostic(code(judiw::db::not_found))]
    WordformNotFound { id: i64 },

    /// Morphological analysis errors
    #[error("Morphological analysis failed: {message}")]
    #[diagnostic(
        code(judiw::morphology::analysis),
        help("Ensure the wordform has valid morphological features")
    )]
    MorphologyError { message: String },

    /// Normalization errors
    #[error("Text normalization failed: {message}")]
    #[diagnostic(code(judiw::normalization::failed))]
    NormalizationError { message: String },

    /// Schema validation errors
    #[error("Schema validation failed: {field} = {value}")]
    #[diagnostic(
        code(judiw::validation::schema),
        help("Ensure the value matches the expected schema type")
    )]
    ValidationError {
        field: String,
        value: String,
        #[source_code]
        input: String,
        #[label("invalid value")]
        span: SourceSpan,
    },

    /// Cache errors
    #[error("Cache error: {0}")]
    #[diagnostic(code(judiw::cache::operation))]
    Cache(String),

    /// IO errors
    #[error("IO error: {path}")]
    #[diagnostic(code(judiw::io::error), help("Check file permissions and disk space"))]
    Io {
        path: PathBuf,
        #[source]
        error: std::io::Error,
    },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    #[diagnostic(
        code(judiw::config::invalid),
        help("Check your .env file and environment variables")
    )]
    Config(String),

    /// Parse errors
    #[error("Failed to parse {what}: {input}")]
    #[diagnostic(
        code(judiw::parse::error),
        help("Ensure the input is in the correct format")
    )]
    ParseError {
        what: String,
        input: String,
        #[source]
        error: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Result type alias for Judiw operations
pub type Result<T> = std::result::Result<T, JudiwError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = JudiwError::InvalidUuid(uuid::Error::InvalidVersion(0));
        assert!(err.to_string().contains("Invalid UUID"));
    }
}
