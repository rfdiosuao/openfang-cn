//! In-memory TTL cache for web search and fetch results.
//!
//! Thread-safe via `DashMap`. Lazy eviction on `get()` — expired entries
//! are only cleaned up when accessed. A `Duration::ZERO` TTL disables
//! caching entirely (zero-cost passthrough).

use dashmap::DashMap;
use std::time::{Duration, Instant};

/// A cached entry with its insertion timestamp.
struct CacheEntry {
    value: String,
    inserted_at: Instant,
}

/// Thread-safe in-memory cache with configurable TTL.
pub struct WebCache {
    entries: DashMap<String, CacheEntry>,
    ttl: Duration,
}

impl WebCache {
    /// Create a new cache with the given TTL. A TTL of `Duration::ZERO` disables caching.
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: DashMap::new(),
            ttl,
        }
    }

    /// Get a cached value by key. Returns `None` if missing or expired.
    /// Expired entries are lazily evicted on access.
    pub fn get(&self, key: &str) -> Option<String> {
        if self.ttl.is_zero() {
            return None;
        }
        let entry = self.entries.get(key)?;
        if entry.inserted_at.elapsed() > self.ttl {
            drop(entry); // release read lock before removing
            self.entries.remove(key);
            None
        } else {
            Some(entry.value.clone())
        }
    }

    /// Store a value in the cache. No-op if TTL is zero.
    pub fn put(&self, key: String, value: String) {
        if self.ttl.is_zero() {
            return;
        }
        self.entries.insert(
            key,
            CacheEntry {
                value,
                inserted_at: Instant::now(),
            },
        );
    }

    /// Remove all expired entries. Called periodically or on demand.
    pub fn evict_expired(&self) {
        self.entries
            .retain(|_, entry| entry.inserted_at.elapsed() <= self.ttl);
    }

    /// Number of entries currently in the cache (including possibly expired).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let cache = WebCache::new(Duration::from_secs(60));
        cache.put("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_cache_miss() {
        let cache = WebCache::new(Duration::from_secs(60));
        assert_eq!(cache.get("nonexistent"), None);
    }

    #[test]
    fn test_expired_entry() {
        let cache = WebCache::new(Duration::from_millis(1));
        cache.put("key1".to_string(), "value1".to_string());
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_evict_expired() {
        let cache = WebCache::new(Duration::from_millis(1));
        cache.put("a".to_string(), "1".to_string());
        cache.put("b".to_string(), "2".to_string());
        std::thread::sleep(Duration::from_millis(10));
        cache.evict_expired();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_zero_ttl_disables_caching() {
        let cache = WebCache::new(Duration::ZERO);
        cache.put("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), None);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_overwrite() {
        let cache = WebCache::new(Duration::from_secs(60));
        cache.put("key1".to_string(), "old".to_string());
        cache.put("key1".to_string(), "new".to_string());
        assert_eq!(cache.get("key1"), Some("new".to_string()));
    }

    #[test]
    fn test_len() {
        let cache = WebCache::new(Duration::from_secs(60));
        assert_eq!(cache.len(), 0);
        cache.put("a".to_string(), "1".to_string());
        cache.put("b".to_string(), "2".to_string());
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let cache = WebCache::new(Duration::from_secs(60));
        assert!(cache.is_empty());
        cache.put("a".to_string(), "1".to_string());
        assert!(!cache.is_empty());
    }
}
