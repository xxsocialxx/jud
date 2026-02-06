// ============================================================================
// QUERY RESULT CACHING
// ============================================================================

#![allow(dead_code)]

use crate::models::{Lexeme, Sense, Wordform};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Cache key for different query types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    /// Lookup lexemes by query string
    Lookup(String),
    /// Get full lexeme details by UUID
    LexemeDetails(Uuid),
    /// Search query (limit only - threshold affects scoring, not results)
    Search(String, usize),
    /// Get wordforms for lexeme
    Wordforms(Uuid),
    /// Get senses for lexeme
    Senses(Uuid),
}

// SearchParams kept for reference but not used in cache key
#[derive(Debug, Clone)]
pub struct SearchParams {
    pub limit: usize,
    pub threshold: f64,
}

/// Cached query result
#[derive(Debug, Clone)]
pub enum CachedResult {
    Lexemes(Vec<Lexeme>),
    LexemeDetails(Box<(Lexeme, Vec<Wordform>, Vec<Sense>)>),
    SearchResults(Vec<crate::search::SearchResult>),
    Wordforms(Vec<Wordform>),
    Senses(Vec<Sense>),
}

/// LRU cache for database query results
pub struct QueryCache {
    lexemes: Arc<Mutex<LruCache<CacheKey, CachedResult>>>,
    stats: Arc<Mutex<CacheStats>>,
}

#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub size: usize,
}

impl QueryCache {
    /// Create new cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        let size = NonZeroUsize::new(capacity).unwrap();
        Self {
            lexemes: Arc::new(Mutex::new(LruCache::new(size))),
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    /// Get cached result if exists
    pub fn get(&self, key: &CacheKey) -> Option<CachedResult> {
        let mut cache = self.lexemes.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        match cache.get(key) {
            Some(result) => {
                stats.hits += 1;
                Some(result.clone())
            }
            None => {
                stats.misses += 1;
                None
            }
        }
    }

    /// Put result in cache
    pub fn put(&self, key: CacheKey, value: CachedResult) {
        let mut cache = self.lexemes.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        cache.put(key, value);
        stats.size = cache.len();
    }

    /// Invalidate cache entry
    pub fn invalidate(&self, key: &CacheKey) {
        let mut cache = self.lexemes.lock().unwrap();
        cache.pop(key);
    }

    /// Clear entire cache
    pub fn clear(&self) {
        let mut cache = self.lexemes.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        cache.clear();
        stats.size = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        let stats = self.stats.lock().unwrap();
        let total = stats.hits + stats.misses;

        if total == 0 {
            return 0.0;
        }

        stats.hits as f64 / total as f64
    }

    /// Format cache statistics for display
    pub fn format_stats(&self) -> String {
        let stats = self.stats();
        let hit_rate = self.hit_rate();

        format!(
            "Cache: {} entries | Hits: {} | Misses: {} | Hit rate: {:.1}%",
            stats.size,
            stats.hits,
            stats.misses,
            hit_rate * 100.0
        )
    }
}

/// Macro to check cache before executing query
#[macro_export]
macro_rules! cached_query {
    ($cache:expr, $key:expr, $query:expr) => {{
        if let Some(result) = $cache.get(&$key) {
            return Ok(result);
        }

        let result = $query.await?;
        $cache.put($key, result.clone());
        Ok(result)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit_miss() {
        let cache = QueryCache::new(100);

        let key = CacheKey::Lookup("test".to_string());
        let result = cache.get(&key);

        assert!(result.is_none());

        let lexemes = vec![];
        cache.put(key.clone(), CachedResult::Lexemes(lexemes.clone()));

        let result = cache.get(&key);
        assert!(result.is_some());
    }

    #[test]
    fn test_cache_stats() {
        let cache = QueryCache::new(10);

        let key = CacheKey::Lookup("test".to_string());

        // Miss
        cache.get(&key);
        assert_eq!(cache.stats().misses, 1);

        // Put and hit
        cache.put(key.clone(), CachedResult::Lexemes(vec![]));
        cache.get(&key);
        assert_eq!(cache.stats().hits, 1);

        // Hit rate
        assert_eq!(cache.hit_rate(), 0.5);
    }
}
