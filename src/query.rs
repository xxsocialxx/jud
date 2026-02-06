// ============================================================================
// TYPE-SAFE QUERY BUILDER
// ============================================================================

use crate::error::{JudiwError, Result};
use crate::models::{Lexeme, Sense, Wordform};
use uuid::Uuid;

/// Type-safe SQL query builder
pub struct QueryBuilder {
    tables: Vec<String>,
    where_clauses: Vec<String>,
    params: Vec<String>,
    order_by: Option<String>,
    limit: Option<usize>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            tables: vec![],
            where_clauses: vec![],
            params: vec![],
            order_by: None,
            limit: None,
        }
    }

    /// Add a table to the query (compile-time checked)
    pub fn from<T: QueryTable>(mut self) -> Self {
        self.tables.push(T::TABLE_NAME.to_string());
        self
    }

    /// Join with another table
    pub fn join<T: QueryTable>(mut self, on: &str) -> Self {
        self.tables
            .push(format!("JOIN {} ON {}", T::TABLE_NAME, on));
        self
    }

    /// Add WHERE clause with parameter binding
    pub fn where_(mut self, clause: &str, param: String) -> Self {
        self.where_clauses.push(clause.to_string());
        self.params.push(param);
        self
    }

    /// Add ORDER BY
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order_by = Some(format!("{} {}", column, direction.as_str()));
        self
    }

    /// Add LIMIT
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Build the final SQL query
    pub fn build(self) -> Result<String> {
        if self.tables.is_empty() {
            return Err(JudiwError::Config("No table specified".to_string()));
        }

        let mut sql = format!("SELECT * FROM {}", self.tables.join(" "));

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        if let Some(order) = self.order_by {
            sql.push_str(" ORDER BY ");
            sql.push_str(&order);
        }

        if let Some(n) = self.limit {
            sql.push_str(&format!(" LIMIT {}", n));
        }

        Ok(sql)
    }

    /// Get parameter count for this query
    pub fn param_count(&self) -> usize {
        self.params.len()
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// ORDER BY direction (type-safe)
#[derive(Debug, Clone, Copy)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl OrderDirection {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

/// Trait for tables that can be queried
pub trait QueryTable {
    const TABLE_NAME: &'static str;
}

impl QueryTable for Lexeme {
    const TABLE_NAME: &'static str = "linguayi_lexeme";
}

impl QueryTable for Wordform {
    const TABLE_NAME: &'static str = "linguayi_wordform";
}

impl QueryTable for Sense {
    const TABLE_NAME: &'static str = "linguayi_sense";
}

// ============================================================================
// SPECIALIZED BUILDERS
// ============================================================================

/// Builder for lexeme queries
pub struct LexemeQueryBuilder {
    builder: QueryBuilder,
}

impl LexemeQueryBuilder {
    pub fn new() -> Self {
        Self {
            builder: QueryBuilder::new().from::<Lexeme>(),
        }
    }

    pub fn by_id(mut self, id: Uuid) -> Self {
        self.builder = self.builder.where_("id = $1", id.to_string());
        self
    }

    pub fn by_hebrew(mut self, hebrew: &str) -> Self {
        self.builder = self
            .builder
            .where_("canonical_hebrew ILIKE $1", format!("%{}%", hebrew));
        self
    }

    pub fn by_roman(mut self, roman: &str) -> Self {
        self.builder = self
            .builder
            .where_("canonical_roman ILIKE $1", format!("%{}%", roman));
        self
    }

    pub fn with_wordforms(mut self) -> Self {
        self.builder = self
            .builder
            .join::<Wordform>("lexeme_id = linguayi_lexeme.id");
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.builder = self.builder.limit(n);
        self
    }

    pub fn build(self) -> Result<String> {
        self.builder.build()
    }
}

impl Default for LexemeQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder_basic() {
        let sql = QueryBuilder::new()
            .from::<Lexeme>()
            .where_("id = $1", "test".to_string())
            .limit(10)
            .build()
            .unwrap();

        assert!(sql.contains("FROM linguayi_lexeme"));
        assert!(sql.contains("WHERE id = $1"));
        assert!(sql.contains("LIMIT 10"));
    }

    #[test]
    fn test_lexeme_builder() {
        let id = Uuid::new_v4();
        let sql = LexemeQueryBuilder::new()
            .by_id(id)
            .with_wordforms()
            .build()
            .unwrap();

        assert!(sql.contains("linguayi_lexeme"));
        assert!(sql.contains("linguayi_wordform"));
        assert!(sql.contains("lexeme_id = linguayi_lexeme.id"));
    }
}
