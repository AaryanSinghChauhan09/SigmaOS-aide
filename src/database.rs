//! Integrity database for aide.

use std::collections::HashMap;
use crate::file::{FileEntry, FileAttributes, FileHash};

/// Database entry
#[derive(Debug, Clone)]
pub struct DatabaseEntry {
    /// File path
    pub path: String,
    /// File attributes
    pub attributes: FileAttributes,
    /// File hash
    pub hash: FileHash,
    /// Added timestamp
    pub added_at: u64,
    /// Last checked timestamp
    pub last_checked: u64,
}

/// Integrity database
#[derive(Debug, Clone)]
pub struct IntegrityDatabase {
    /// Database entries
    entries: HashMap<String, DatabaseEntry>,
    /// Database version
    pub version: String,
    /// Database created timestamp
    pub created_at: u64,
    /// Database modified timestamp
    pub modified_at: u64,
    /// Database statistics
    pub stats: DatabaseStatistics,
}

/// Database statistics
#[derive(Debug, Clone, Default)]
pub struct DatabaseStatistics {
    /// Total entries
    pub total_entries: u64,
    /// Total size in bytes
    pub total_size: u64,
    /// Last update timestamp
    pub last_update: u64,
    /// Database integrity score
    pub integrity_score: f64,
}

impl Default for IntegrityDatabase {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            version: "1.0".into(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            stats: DatabaseStatistics::default(),
        }
    }
}

impl IntegrityDatabase {
    /// Create new integrity database
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add entry to database
    pub fn add_entry(&mut self, entry: DatabaseEntry) {
        self.entries.insert(entry.path.clone(), entry);
        self.update_statistics();
    }
    
    /// Remove entry from database
    pub fn remove_entry(&mut self, path: &str) -> bool {
        if self.entries.remove(path).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get entry by path
    pub fn get_entry(&self, path: &str) -> Option<&DatabaseEntry> {
        self.entries.get(path)
    }
    
    /// Get entry mutably
    pub fn get_entry_mut(&mut self, path: &str) -> Option<&mut DatabaseEntry> {
        self.entries.get_mut(path)
    }
    
    /// Get all entries
    pub fn get_entries(&self) -> Vec<&DatabaseEntry> {
        self.entries.values().collect()
    }
    
    /// Get entries by directory
    pub fn get_entries_by_directory(&self, directory: &str) -> Vec<&DatabaseEntry> {
        self.entries.values()
            .filter(|e| e.path.starts_with(directory))
            .collect()
    }
    
    /// Search entries
    pub fn search_entries(&self, query: &str) -> Vec<&DatabaseEntry> {
        let query_lower = query.to_lowercase();
        self.entries.values()
            .filter(|e| e.path.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Update entry
    pub fn update_entry(&mut self, path: &str, attributes: FileAttributes, hash: FileHash) -> bool {
        if let Some(entry) = self.entries.get_mut(path) {
            entry.attributes = attributes;
            entry.hash = hash;
            entry.last_checked = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Mark entry as checked
    pub fn mark_checked(&mut self, path: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(path) {
            entry.last_checked = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return true;
        }
        false
    }
    
    /// Get entries by last checked time
    pub fn get_entries_by_last_checked(&self, before: u64) -> Vec<&DatabaseEntry> {
        self.entries.values()
            .filter(|e| e.last_checked < before)
            .collect()
    }
    
    /// Get entries by added time
    pub fn get_entries_by_added_time(&self, after: u64) -> Vec<&DatabaseEntry> {
        self.entries.values()
            .filter(|e| e.added_at > after)
            .collect()
    }
    
    /// Clear database
    pub fn clear(&mut self) {
        self.entries.clear();
        self.update_statistics();
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.stats.total_entries = self.entries.len() as u64;
        self.stats.total_size = self.entries.values()
            .map(|e| e.attributes.size)
            .sum();
        self.stats.last_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Calculate integrity score (percentage of entries checked in last 24 hours)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let day_ago = now - 86400;
        
        let checked_entries = self.entries.values()
            .filter(|e| e.last_checked > day_ago)
            .count() as f64;
        
        self.stats.integrity_score = if self.entries.is_empty() {
            100.0
        } else {
            (checked_entries / self.entries.len() as f64) * 100.0
        };
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &DatabaseStatistics {
        &self.stats
    }
    
    /// Export database
    pub fn export_data(&self) -> DatabaseExport {
        DatabaseExport {
            entries: self.entries.values().cloned().collect(),
            version: self.version.clone(),
            created_at: self.created_at,
            modified_at: self.modified_at,
        }
    }
    
    /// Import database
    pub fn import_data(&mut self, export: DatabaseExport) {
        self.entries.clear();
        for entry in export.entries {
            self.entries.insert(entry.path.clone(), entry);
        }
        self.version = export.version;
        self.created_at = export.created_at;
        self.modified_at = export.modified_at;
        self.update_statistics();
    }
}

/// Database export data
#[derive(Debug, Clone)]
pub struct DatabaseExport {
    /// Database entries
    pub entries: Vec<DatabaseEntry>,
    /// Database version
    pub version: String,
    /// Created timestamp
    pub created_at: u64,
    /// Modified timestamp
    pub modified_at: u64,
}
