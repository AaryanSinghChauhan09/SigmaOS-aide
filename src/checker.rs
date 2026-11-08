//! Integrity checker for aide.

use std::collections::HashMap;
use crate::file::{FileEntry, FileAttributes, FileHash, FileType};
use crate::database::{IntegrityDatabase, DatabaseEntry};

/// Check status
#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    /// File unchanged
    Unchanged,
    /// File changed
    Changed,
    /// File added
    Added,
    /// File removed
    Removed,
    /// Error occurred
    Error,
}

/// Check result
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Number of files checked
    pub checked: usize,
    /// Number of files changed
    pub changed: usize,
    /// Number of files added
    pub added: usize,
    /// Number of files removed
    pub removed: usize,
    /// Number of errors
    pub errors: usize,
}

/// Integrity checker
#[derive(Debug, Clone)]
pub struct IntegrityChecker {
    /// Database
    pub database: IntegrityDatabase,
    /// Check results
    pub results: HashMap<String, CheckStatus>,
    /// Last check timestamp
    pub last_check: u64,
    /// Check statistics
    pub stats: CheckStatistics,
}

/// Check statistics
#[derive(Debug, Clone, Default)]
pub struct CheckStatistics {
    /// Total checks performed
    pub total_checks: u64,
    /// Total files checked
    pub total_files_checked: u64,
    /// Total files changed
    pub total_files_changed: u64,
    /// Total files added
    pub total_files_added: u64,
    /// Total files removed
    pub total_files_removed: u64,
    /// Total errors
    pub total_errors: u64,
    /// Last check timestamp
    pub last_check: u64,
}

impl Default for IntegrityChecker {
    fn default() -> Self {
        Self {
            database: IntegrityDatabase::new(),
            results: HashMap::new(),
            last_check: 0,
            stats: CheckStatistics::default(),
        }
    }
}

impl IntegrityChecker {
    /// Create new integrity checker
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create checker with database
    pub fn with_database(database: IntegrityDatabase) -> Self {
        Self {
            database,
            results: HashMap::new(),
            last_check: 0,
            stats: CheckStatistics::default(),
        }
    }
    
    /// Initialize database with files
    pub fn initialize(&mut self, files: Vec<FileEntry>) -> usize {
        let mut count = 0;
        
        for file in files {
            let entry = DatabaseEntry {
                path: file.path.clone(),
                attributes: file.attributes,
                hash: file.hash,
                added_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                last_checked: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            self.database.add_entry(entry);
            count += 1;
        }
        
        count
    }
    
    /// Check files
    pub fn check_files(&mut self, files: Vec<String>) -> CheckResult {
        let mut result = CheckResult {
            checked: 0,
            changed: 0,
            added: 0,
            removed: 0,
            errors: 0,
        };
        
        for file_path in files {
            match self.check_file(&file_path) {
                Ok(status) => {
                    self.results.insert(file_path, status.clone());
                    result.checked += 1;
                    
                    match status {
                        CheckStatus::Unchanged => {}
                        CheckStatus::Changed => result.changed += 1,
                        CheckStatus::Added => result.added += 1,
                        CheckStatus::Removed => result.removed += 1,
                        CheckStatus::Error => result.errors += 1,
                    }
                }
                Err(_) => {
                    self.results.insert(file_path.clone(), CheckStatus::Error);
                    result.errors += 1;
                }
            }
        }
        
        // Update last check timestamp
        self.last_check = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Update statistics
        self.stats.total_checks += 1;
        self.stats.total_files_checked += result.checked as u64;
        self.stats.total_files_changed += result.changed as u64;
        self.stats.total_files_added += result.added as u64;
        self.stats.total_files_removed += result.removed as u64;
        self.stats.total_errors += result.errors as u64;
        self.stats.last_check = self.last_check;
        
        result
    }
    
    /// Check single file
    fn check_file(&mut self, file_path: &str) -> Result<CheckStatus, String> {
        // Get current file info
        let current_file = self.get_file_info(file_path)?;
        
        // Get database entry
        let db_entry = self.database.get_entry(file_path);
        
        if let Some(db_entry) = db_entry {
            // Compare current file with database entry
            let status = self.compare_file(&current_file, db_entry);
            
            // Update database entry
            self.database.update_entry(
                file_path,
                current_file.attributes.clone(),
                current_file.hash.clone()
            );
            
            Ok(status)
        } else {
            // File not in database - add it
            let entry = DatabaseEntry {
                path: file_path.to_string(),
                attributes: current_file.attributes,
                hash: current_file.hash,
                added_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                last_checked: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            self.database.add_entry(entry);
            Ok(CheckStatus::Added)
        }
    }
    
    /// Get file information
    fn get_file_info(&self, file_path: &str) -> Result<FileEntry, String> {
        // Simulate file info retrieval
        // In real implementation, would use actual filesystem calls
        Ok(FileEntry {
            path: file_path.to_string(),
            attributes: FileAttributes {
                permissions: "755".into(),
                owner: "root".into(),
                group: "root".into(),
                size: 1024,
                mtime: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                inode: 12345,
                device: 2048,
                links: 1,
            },
            hash: FileHash {
                md5: "d41d8cd98f00b204e9800998ecf8427e".into(),
                sha1: "da39a3ee5e6b4b0d3255bfef95601890afd80709".into(),
                sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into(),
                sha512: "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e".into(),
            },
            file_type: FileType::File,
            executable: true,
            writable: true,
            readable: true,
        })
    }
    
    /// Compare file with database entry
    fn compare_file(&self, current: &FileEntry, db_entry: &DatabaseEntry) -> CheckStatus {
        // Compare attributes
        if current.attributes.permissions != db_entry.attributes.permissions ||
           current.attributes.owner != db_entry.attributes.owner ||
           current.attributes.group != db_entry.attributes.group ||
           current.attributes.size != db_entry.attributes.size {
            return CheckStatus::Changed;
        }
        
        // Compare hashes
        if current.hash.md5 != db_entry.hash.md5 ||
           current.hash.sha1 != db_entry.hash.sha1 ||
           current.hash.sha256 != db_entry.hash.sha256 ||
           current.hash.sha512 != db_entry.hash.sha512 {
            return CheckStatus::Changed;
        }
        
        CheckStatus::Unchanged
    }
    
    /// Check for removed files
    pub fn check_removed_files(&mut self) -> Vec<String> {
        let mut removed = vec![];
        
        for entry in self.database.get_entries() {
            if !self.results.contains_key(&entry.path) {
                // File in database but not checked - might be removed
                removed.push(entry.path.clone());
            }
        }
        
        removed
    }
    
    /// Get check result for file
    pub fn get_check_result(&self, file_path: &str) -> Option<&CheckStatus> {
        self.results.get(file_path)
    }
    
    /// Get all check results
    pub fn get_results(&self) -> Vec<(&String, &CheckStatus)> {
        self.results.iter().collect()
    }
    
    /// Get check statistics
    pub fn get_statistics(&self) -> &CheckStatistics {
        &self.stats
    }
    
    /// Generate integrity report
    pub fn generate_report(&self) -> crate::report::IntegrityReport {
        let mut entries = vec![];
        let mut summary = crate::report::ReportSummary {
            total_files: self.results.len() as u64,
            unchanged_files: 0,
            changed_files: 0,
            added_files: 0,
            removed_files: 0,
            error_files: 0,
            integrity_percentage: 0.0,
        };
        
        for (path, status) in &self.results {
            let entry = crate::report::ReportEntry {
                path: path.clone(),
                status: status.clone(),
                timestamp: self.last_check,
                details: String::new(),
            };
            
            entries.push(entry);
            
            match status {
                CheckStatus::Unchanged => summary.unchanged_files += 1,
                CheckStatus::Changed => summary.changed_files += 1,
                CheckStatus::Added => summary.added_files += 1,
                CheckStatus::Removed => summary.removed_files += 1,
                CheckStatus::Error => summary.error_files += 1,
            }
        }
        
        summary.integrity_percentage = if summary.total_files > 0 {
            (summary.unchanged_files as f64 / summary.total_files as f64) * 100.0
        } else {
            100.0
        };
        
        crate::report::IntegrityReport {
            generated_at: self.last_check,
            entries,
            summary,
        }
    }
    
    /// Export checker data
    pub fn export_data(&self) -> CheckerExport {
        CheckerExport {
            database: self.database.export_data(),
            results: self.results.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }
    
    /// Import checker data
    pub fn import_data(&mut self, export: CheckerExport) {
        self.database.import_data(export.database);
        
        self.results.clear();
        for (path, status) in export.results {
            self.results.insert(path, status);
        }
    }
}

/// Checker export data
#[derive(Debug, Clone)]
pub struct CheckerExport {
    /// Database export
    pub database: crate::database::DatabaseExport,
    /// Check results
    pub results: Vec<(String, CheckStatus)>,
}
