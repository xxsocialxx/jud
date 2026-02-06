// ============================================================================
// SCHEMA VALIDATION
// ============================================================================

use schemars::{gen::SchemaGenerator, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
use std::collections::HashMap;

/// Schema validator for all data models
pub struct SchemaValidator {
    schemas: HashMap<String, Value>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            schemas: HashMap::new(),
        };

        validator.register_core_schemas();
        validator
    }

    fn register_core_schemas(&mut self) {
        // Lexeme schema
        self.schemas.insert(
            "Lexeme".to_string(),
            json_schema_for::<crate::models::Lexeme>(),
        );

        // Wordform schema
        self.schemas.insert(
            "Wordform".to_string(),
            json_schema_for::<crate::models::Wordform>(),
        );

        // Sense schema
        self.schemas.insert(
            "Sense".to_string(),
            json_schema_for::<crate::models::Sense>(),
        );
    }

    /// Validate JSON against schema
    pub fn validate_json(&self, type_name: &str, data: &Value) -> Result<(), ValidationError> {
        let schema = self
            .schemas
            .get(type_name)
            .ok_or_else(|| ValidationError::SchemaNotFound(type_name.to_string()))?;

        // Perform validation (simplified - use jsonschema crate for full validation)
        self.validate_value(schema, data)
    }

    /// Validate Rust struct
    pub fn validate<T>(&self, value: &T) -> Result<(), ValidationError>
    where
        T: Serialize,
    {
        let json =
            to_value(value).map_err(|e| ValidationError::SerializationError(e.to_string()))?;

        let type_name = std::any::type_name::<T>();
        self.validate_json(type_name, &json)
    }

    fn validate_value(&self, schema: &Value, data: &Value) -> Result<(), ValidationError> {
        // Check required fields
        if let Some(obj) = schema.as_object() {
            if let Some(required) = obj.get("required") {
                if let Some(required_array) = required.as_array() {
                    if let Some(data_obj) = data.as_object() {
                        for field in required_array {
                            if let Some(field_str) = field.as_str() {
                                if !data_obj.contains_key(field_str) {
                                    return Err(ValidationError::MissingField(
                                        field_str.to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate JSON schema for a type
fn json_schema_for<T>() -> Value
where
    T: JsonSchema,
{
    let mut gen = SchemaGenerator::default();
    gen.generate_root_schema::<T>().into()
}

/// Validation errors
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Schema not found: {0}")]
    SchemaNotFound(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation failed: {0}")]
    Custom(String),
}

/// Compile-time schema validator (const generic)
pub trait ValidateSchema {
    fn validate_schema() -> Result<(), ValidationError>;
}

// Implement for core models
impl ValidateSchema for crate::models::Lexeme {
    fn validate_schema() -> Result<(), ValidationError> {
        // Compile-time checks could go here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_validation() {
        let validator = SchemaValidator::new();
        let data = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "canonical_hebrew": "שלום",
            "canonical_roman": "shalom",
            "part_of_speech": "Noun",
            "gender": "Masc",
            "origin": "Hebrew",
            "status": "active",
            "analyzed": false,
            "skeleton_collision": false,
            "romanization_status": "done",
            "romanization_confidence": 0.95,
            "ipa_confidence": 0.9,
            "romanization_source": "rules",
            "romanization_version": 1,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        });

        let result = validator.validate_json("Lexeme", &data);
        assert!(result.is_ok());
    }
}
