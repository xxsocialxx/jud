// ============================================================================
// SEMANTIC FIELD SYSTEM
// ============================================================================

use crate::models::{Lexeme, Sense};
use crate::error::{JudiwError, Result};
use crate::db::Database;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Semantic field tags (type-safe, not arbitrary strings!)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticField {
    // Time & Existence
    Time,
    Existence,
    Creation,
    Destruction,

    // Human & Society
    Family,
    Community,
    Religion,
    Politics,
    Commerce,

    // Nature & World
    Nature,
    Animals,
    Plants,
    Geography,
    Weather,

    // Actions & States
    Motion,
    Cognition,
    Emotion,
    Communication,
    Perception,

    // Material World
    Food,
    Clothing,
    Housing,
    Tools,
    Artifacts,

    // Abstract Concepts
    Truth,
    Beauty,
    Justice,
    Virtue,
    Vice,

    // Language & Metalinguistics
    Language,
    Grammar,
    Metalinguistics,
}

impl SemanticField {
    /// All semantic fields
    pub fn all() -> Vec<Self> {
        vec![
            Self::Time, Self::Existence, Self::Creation, Self::Destruction,
            Self::Family, Self::Community, Self::Religion, Self::Politics, Self::Commerce,
            Self::Nature, Self::Animals, Self::Plants, Self::Geography, Self::Weather,
            Self::Motion, Self::Cognition, Self::Emotion, Self::Communication, Self::Perception,
            Self::Food, Self::Clothing, Self::Housing, Self::Tools, Self::Artifacts,
            Self::Truth, Self::Beauty, Self::Justice, Self::Virtue, Self::Vice,
            Self::Language, Self::Grammar, Self::Metalinguistics,
        ]
    }

    /// Get field hierarchy (parent fields)
    pub fn parents(&self) -> Vec<Self> {
        match self {
            Self::Family => vec![],
            Self::Commerce => vec![],
            Self::Food => vec![Self::Commerce],
            Self::Religion => vec![Self::Community],
            _ => vec![],
        }
    }

    /// Get child fields (more specific)
    pub fn children(&self) -> Vec<Self> {
        match self {
            Self::Commerce => vec![Self::Food],
            Self::Community => vec![Self::Religion, Self::Family],
            _ => vec![],
        }
    }
}

impl std::fmt::Display for SemanticField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", name.to_lowercase())
    }
}

/// Semantic field query builder (type-safe!)
#[derive(Debug, Clone)]
pub struct SemanticQuery {
    fields: Vec<SemanticField>,
    operator: FieldOperator,
    register: Option<crate::models::shared::Register>,
    dialect: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldOperator {
    Any,   // Match ANY of the fields
    All,   // Match ALL of the fields
    None,  // Match NONE of the fields
}

impl SemanticQuery {
    pub fn new() -> Self {
        Self {
            fields: vec![],
            operator: FieldOperator::Any,
            register: None,
            dialect: None,
        }
    }

    /// Require ALL fields
    pub fn all(mut self) -> Self {
        self.operator = FieldOperator::All;
        self
    }

    /// Exclude all fields
    pub fn none(mut self) -> Self {
        self.operator = FieldOperator::None;
        self
    }

    /// Add field
    pub fn field(mut self, field: SemanticField) -> Self {
        self.fields.push(field);
        self
    }

    /// Add multiple fields
    pub fn fields(mut self, fields: Vec<SemanticField>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// Filter by register
    pub fn register(mut self, register: crate::models::shared::Register) -> Self {
        self.register = Some(register);
        self
    }

    /// Filter by dialect
    pub fn dialect(mut self, dialect: &str) -> Self {
        self.dialect = Some(dialect.to_string());
        self
    }

    /// Execute query
    pub async fn execute(&self, db: &Database) -> Result<Vec<Sense>> {
        let sql = self.build_sql()?;
        let rows = db.client.query(&sql, &[]).await?;

        let mut senses = Vec::new();
        for row in rows {
            senses.push(Sense {
                id: row.get::<_, i64>("id")?,
                lexeme_id: row.get::<_, uuid::Uuid>("lexeme_id")?,
                definition_number: row.try_get::<_, i32>("definition_number").ok(),
                definition: row.get::<_, String>("definition")?,
                definition_yi: row.try_get::<_, String>("definition_yi").ok(),
                definition_english: row.try_get::<_, String>("definition_english").ok(),
                semantic_field: row.try_get::<_, Vec<String>>("semantic_field").unwrap_or_default(),
                register_specific: row.try_get::<_, String>("register_specific").ok(),
                dialect_specific: row.try_get::<_, Vec<String>>("dialect_specific").ok(),
                domain_specific: row.try_get::<_, Vec<String>>("domain_specific").ok(),
                connotation: row.try_get::<_, String>("connotation").ok(),
                frequency_in_sense: row.try_get::<_, f32>("frequency_in_sense").ok(),
                is_primary_sense: row.get::<_, bool>("is_primary_sense")?,
                flow_state: row.get::<_, String>("flow_state")?,
                source_id: row.try_get::<_, i64>("source_id").ok(),
                usage_notes: row.try_get::<_, String>("usage_notes").ok(),
                examples: vec![],
                relationships: vec![],
            });
        }

        Ok(senses)
    }

    fn build_sql(&self) -> Result<String> {
        let field_strings: Vec<String> = self.fields.iter()
            .map(|f| format!("'{}'", f))
            .collect();

        let (operator, array_op) = match self.operator {
            FieldOperator::Any => ("&&", "&&"),
            FieldOperator::All => ("&&", "&&"),
            FieldOperator::None => ("!", "&&"),
        };

        let field_condition = if !field_strings.is_empty() {
            format!(
                "semantic_field {} {} [{}]",
                array_op,
                array_op,
                field_strings.join(", ")
            )
        } else {
            "TRUE".to_string()
        };

        let mut conditions = vec![field_condition];

        if let Some(reg) = &self.register {
            let reg_str = format!("{:?}", reg).to_lowercase();
            conditions.push(format!("register_specific = '{}'", reg_str));
        }

        if let Some(dialect) = &self.dialect {
            conditions.push(format!("'{}' = ANY(dialect_specific)", dialect));
        }

        let where_clause = conditions.join(" AND ");

        Ok(format!(
            "SELECT id, lexeme_id, definition_number, definition, definition_yi,
                    definition_english, semantic_field, register_specific, dialect_specific,
                    domain_specific, connotation, frequency_in_sense, is_primary_sense,
                    flow_state, source_id, usage_notes
            FROM linguayi_sense
            WHERE {}
            ORDER BY is_primary_sense DESC, frequency_in_sense DESC",
            where_clause
        ))
    }
}

impl Default for SemanticQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic field analyzer
pub struct SemanticAnalyzer {
    field_hierarchy: HashMap<SemanticField, Vec<SemanticField>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            field_hierarchy: HashMap::new(),
        };

        analyzer.build_hierarchy();
        analyzer
    }

    fn build_hierarchy(&mut self) {
        for field in SemanticField::all() {
            let children = field.children();
            if !children.is_empty() {
                self.field_hierarchy.insert(field.clone(), children);
            }
        }
    }

    /// Get all senses in a semantic field
    pub async fn get_field_senses(
        &self,
        db: &Database,
        field: SemanticField,
    ) -> Result<Vec<Sense>> {
        SemanticQuery::new()
            .field(field)
            .execute(db)
            .await
    }

    /// Get related senses (in same field)
    pub async fn get_related_senses(
        &self,
        db: &Database,
        sense_id: i64,
    ) -> Result<Vec<Sense>> {
        let sql = r#"
            SELECT s1.semantic_field
            FROM linguayi_sense s1
            WHERE s1.id = $1
        "#;

        let row = db.client.query_one(sql, &[&sense_id]).await?;
        let fields: Vec<String> = row.try_get::<_, Vec<String>>("semantic_field")
            .unwrap_or_default();

        if fields.is_empty() {
            return Ok(vec![]);
        }

        // Query other senses with same fields
        let field_condition = fields.iter()
            .map(|f| format!("'{}'", f))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "SELECT * FROM linguayi_sense
            WHERE semantic_field && [{}]
            AND id != $1
            LIMIT 10",
            field_condition
        );

        let rows = db.client.query(&sql, &[&sense_id]).await?;

        let mut senses = Vec::new();
        for row in rows {
            senses.push(Sense {
                id: row.get::<_, i64>("id")?,
                lexeme_id: row.get::<_, uuid::Uuid>("lexeme_id")?,
                definition_number: row.try_get::<_, i32>("definition_number").ok(),
                definition: row.get::<_, String>("definition")?,
                definition_yi: row.try_get::<_, String>("definition_yi").ok(),
                definition_english: row.try_get::<_, String>("definition_english").ok(),
                semantic_field: row.try_get::<_, Vec<String>>("semantic_field").unwrap_or_default(),
                register_specific: row.try_get::<_, String>("register_specific").ok(),
                dialect_specific: row.try_get::<_, Vec<String>>("dialect_specific").ok(),
                domain_specific: row.try_get::<_, Vec<String>>("domain_specific").ok(),
                connotation: row.try_get::<_, String>("connotation").ok(),
                frequency_in_sense: row.try_get::<_, f32>("frequency_in_sense").ok(),
                is_primary_sense: row.get::<_, bool>("is_primary_sense")?,
                flow_state: row.get::<_, String>("flow_state")?,
                source_id: row.try_get::<_, i64>("source_id").ok(),
                usage_notes: row.try_get::<_, String>("usage_notes").ok(),
                examples: vec![],
                relationships: vec![],
            });
        }

        Ok(senses)
    }

    /// Get field hierarchy path
    pub fn get_hierarchy(&self, field: &SemanticField) -> Vec<SemanticField> {
        let mut path = vec![field.clone()];
        let mut current = field.clone();

        while let Some(parent) = self.get_parent(&current) {
            path.push(parent.clone());
            current = parent;
        }

        path.reverse();
        path
    }

    fn get_parent(&self, field: &SemanticField) -> Option<&SemanticField> {
        // Simple implementation - could use hierarchy map
        None
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// COMMAND-LINE INTEGRATION
// ============================================================================

#[derive(clap::Parser)]
pub struct SemanticCommand {
    /// Search by semantic field
    #[arg(short, long)]
    field: Vec<String>,

    /// Require all fields (default: any)
    #[arg(long)]
    all: bool,

    /// Exclude these fields
    #[arg(long)]
    none: bool,

    /// Filter by register
    #[arg(short, long)]
    register: Option<String>,

    /// Filter by dialect
    #[arg(short, long)]
    dialect: Option<String>,

    /// Limit results
    #[arg(short, long, default_value_t = 20)]
    limit: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_field_hierarchy() {
        let analyzer = SemanticAnalyzer::new();

        // Test that food has commerce as parent
        let food_hierarchy = analyzer.get_hierarchy(&SemanticField::Food);
        assert!(food_hierarchy.contains(&SemanticField::Commerce));
    }

    #[test]
    fn test_query_building() {
        let query = SemanticQuery::new()
            .field(SemanticField::Food)
            .field(SemanticField::Family)
            .all();

        let sql = query.build_sql().unwrap();
        assert!(sql.contains("semantic_field"));
    }
}
