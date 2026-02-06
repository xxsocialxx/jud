// ============================================================================
// MODELS MODULE - ENHANCED LEXICOGRAPHIC DATA MODELS
// ============================================================================

#![allow(dead_code)]

pub mod etymology;
pub mod idiom;
pub mod lexeme;
pub mod morph_features;
pub mod sense;
pub mod shared;
pub mod wordform;

// Re-export commonly used types
pub use shared::{Register, UsageCategory};

pub use lexeme::Lexeme;
pub use sense::Sense;
pub use wordform::Wordform;
