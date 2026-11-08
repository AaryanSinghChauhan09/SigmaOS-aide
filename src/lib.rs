//! SigmaOS-aide — comprehensive file integrity monitoring component.
//!
//! Provides advanced file integrity monitoring with real-time checking,
//! database management, and integrity verification for system files.

pub mod file;
pub mod database;
pub mod checker;
pub mod config;
pub mod report;
pub mod options;

pub use file::{FileEntry, FileAttributes, FileHash};
pub use database::{IntegrityDatabase, DatabaseEntry};
pub use checker::{IntegrityChecker, CheckResult, CheckStatus};
pub use config::{AideConfig, MonitoringRule, RuleType};
pub use report::{IntegrityReport, ReportEntry, ReportSummary};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_attributes_creation() {
        let attrs = FileAttributes {
            permissions: "755".into(),
            owner: "root".into(),
            group: "root".into(),
            size: 1024,
            mtime: 1234567890,
            inode: 12345,
            device: 2048,
            links: 1,
        };
        
        assert_eq!(attrs.permissions, "755");
        assert_eq!(attrs.owner, "root");
        assert_eq!(attrs.size, 1024);
    }
    
    #[test]
    fn test_file_hash_creation() {
        let hash = FileHash {
            md5: "d41d8cd98f00b204e9800998ecf8427e".into(),
            sha1: "da39a3ee5e6b4b0d3255bfef95601890afd80709".into(),
            sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into(),
            sha512: "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e".into(),
        };
        
        assert_eq!(hash.md5.len(), 32);
        assert_eq!(hash.sha256.len(), 64);
    }
    
    #[test]
    fn test_database_entry_creation() {
        let entry = DatabaseEntry {
            path: "/bin/sh".into(),
            attributes: FileAttributes {
                permissions: "755".into(),
                owner: "root".into(),
                group: "root".into(),
                size: 100,
                mtime: 1000,
                inode: 12345,
                device: 2048,
                links: 1,
            },
            hash: FileHash {
                md5: "abc".into(),
                sha1: "def".into(),
                sha256: "ghi".into(),
                sha512: "jkl".into(),
            },
            added_at: 1234567890,
            last_checked: 1234567890,
        };
        
        assert_eq!(entry.path, "/bin/sh");
        assert_eq!(entry.attributes.permissions, "755");
    }
    
    #[test]
    fn test_check_status() {
        let statuses = vec![
            CheckStatus::Unchanged,
            CheckStatus::Changed,
            CheckStatus::Added,
            CheckStatus::Removed,
            CheckStatus::Error,
        ];
        assert_eq!(statuses.len(), 5);
    }
    
    #[test]
    fn test_rule_type() {
        let types = vec![
            RuleType::File,
            RuleType::Directory,
            RuleType::Recursive,
            RuleType::Exclude,
            RuleType::Include,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_report_summary_creation() {
        let summary = ReportSummary {
            total_files: 1000,
            unchanged_files: 950,
            changed_files: 30,
            added_files: 10,
            removed_files: 5,
            error_files: 5,
            integrity_percentage: 95.0,
        };
        
        assert_eq!(summary.total_files, 1000);
        assert_eq!(summary.integrity_percentage, 95.0);
    }
    
    #[test]
    fn test_database_creation() {
        let mut db = IntegrityDatabase::new();
        assert!(db.get_entries().is_empty());
    }
    
    #[test]
    fn test_add_database_entry() {
        let mut db = IntegrityDatabase::new();
        
        let entry = DatabaseEntry {
            path: "/bin/sh".into(),
            attributes: FileAttributes {
                permissions: "755".into(),
                owner: "root".into(),
                group: "root".into(),
                size: 100,
                mtime: 1000,
                inode: 12345,
                device: 2048,
                links: 1,
            },
            hash: FileHash {
                md5: "abc".into(),
                sha1: "def".into(),
                sha256: "ghi".into(),
                sha512: "jkl".into(),
            },
            added_at: 1234567890,
            last_checked: 1234567890,
        };
        
        db.add_entry(entry);
        assert_eq!(db.get_entries().len(), 1);
        assert!(db.get_entry("/bin/sh").is_some());
    }
    
    #[test]
    fn test_checker_creation() {
        let mut checker = IntegrityChecker::new();
        assert!(checker.get_results().is_empty());
    }
    
    #[test]
    fn test_check_files() {
        let mut checker = IntegrityChecker::new();
        
        let result = checker.check_files(vec!["/bin/sh".into(), "/bin/ls".into()]);
        
        assert_eq!(result.checked, 2);
        assert!(checker.get_results().len() > 0);
    }
    
    #[test]
    fn test_config_creation() {
        let mut config = AideConfig::default();
        config.add_rule(MonitoringRule::new("/bin".into(), RuleType::Recursive));
        
        assert_eq!(config.rules.len(), 1);
    }
    
    #[test]
    fn test_report_generation() {
        let mut checker = IntegrityChecker::new();
        checker.check_files(vec!["/bin/sh".into()]);
        
        let report = checker.generate_report();
        
        assert!(report.summary.total_files > 0);
        assert!(report.entries.len() > 0);
    }
    
    #[test]
    fn test_export_import_database() {
        let mut db = IntegrityDatabase::new();
        
        let entry = DatabaseEntry {
            path: "/bin/sh".into(),
            attributes: FileAttributes {
                permissions: "755".into(),
                owner: "root".into(),
                group: "root".into(),
                size: 100,
                mtime: 1000,
                inode: 12345,
                device: 2048,
                links: 1,
            },
            hash: FileHash {
                md5: "abc".into(),
                sha1: "def".into(),
                sha256: "ghi".into(),
                sha512: "jkl".into(),
            },
            added_at: 1234567890,
            last_checked: 1234567890,
        };
        
        db.add_entry(entry);
        
        let export = db.export_data();
        
        let mut db2 = IntegrityDatabase::new();
        db2.import_data(export);
        
        assert_eq!(db2.get_entries().len(), 1);
        assert!(db2.get_entry("/bin/sh").is_some());
    }
}
