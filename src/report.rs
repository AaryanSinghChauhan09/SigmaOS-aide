//! Integrity report for aide.

use crate::checker::CheckStatus;

/// Report entry
#[derive(Debug, Clone)]
pub struct ReportEntry {
    /// File path
    pub path: String,
    /// Check status
    pub status: CheckStatus,
    /// Timestamp
    pub timestamp: u64,
    /// Details
    pub details: String,
}

/// Report summary
#[derive(Debug, Clone)]
pub struct ReportSummary {
    /// Total files
    pub total_files: u64,
    /// Unchanged files
    pub unchanged_files: u64,
    /// Changed files
    pub changed_files: u64,
    /// Added files
    pub added_files: u64,
    /// Removed files
    pub removed_files: u64,
    /// Error files
    pub error_files: u64,
    /// Integrity percentage
    pub integrity_percentage: f64,
}

/// Integrity report
#[derive(Debug, Clone)]
pub struct IntegrityReport {
    /// Generated timestamp
    pub generated_at: u64,
    /// Report entries
    pub entries: Vec<ReportEntry>,
    /// Report summary
    pub summary: ReportSummary,
}

impl IntegrityReport {
    /// Create new integrity report
    pub fn new(generated_at: u64, entries: Vec<ReportEntry>, summary: ReportSummary) -> Self {
        Self {
            generated_at,
            entries,
            summary,
        }
    }
    
    /// Get report as text
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        
        text.push_str(&format!("Integrity Report Generated: {}\n", self.generated_at));
        text.push_str(&format!("Total Files: {}\n", self.summary.total_files));
        text.push_str(&format!("Unchanged: {}\n", self.summary.unchanged_files));
        text.push_str(&format!("Changed: {}\n", self.summary.changed_files));
        text.push_str(&format!("Added: {}\n", self.summary.added_files));
        text.push_str(&format!("Removed: {}\n", self.summary.removed_files));
        text.push_str(&format!("Errors: {}\n", self.summary.error_files));
        text.push_str(&format!("Integrity: {:.2}%\n", self.summary.integrity_percentage));
        text.push_str("\nDetails:\n");
        
        for entry in &self.entries {
            let status_str = match entry.status {
                CheckStatus::Unchanged => "UNCHANGED",
                CheckStatus::Changed => "CHANGED",
                CheckStatus::Added => "ADDED",
                CheckStatus::Removed => "REMOVED",
                CheckStatus::Error => "ERROR",
            };
            
            text.push_str(&format!("{}: {} - {}\n", status_str, entry.path, entry.details));
        }
        
        text
    }
    
    /// Get report as JSON
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"generated_at\": {},\n", self.generated_at));
        json.push_str("  \"summary\": {\n");
        json.push_str(&format!("    \"total_files\": {},\n", self.summary.total_files));
        json.push_str(&format!("    \"unchanged_files\": {},\n", self.summary.unchanged_files));
        json.push_str(&format!("    \"changed_files\": {},\n", self.summary.changed_files));
        json.push_str(&format!("    \"added_files\": {},\n", self.summary.added_files));
        json.push_str(&format!("    \"removed_files\": {},\n", self.summary.removed_files));
        json.push_str(&format!("    \"error_files\": {},\n", self.summary.error_files));
        json.push_str(&format!("    \"integrity_percentage\": {:.2}\n", self.summary.integrity_percentage));
        json.push_str("  },\n");
        json.push_str("  \"entries\": [\n");
        
        for (i, entry) in self.entries.iter().enumerate() {
            let status_str = match entry.status {
                CheckStatus::Unchanged => "unchanged",
                CheckStatus::Changed => "changed",
                CheckStatus::Added => "added",
                CheckStatus::Removed => "removed",
                CheckStatus::Error => "error",
            };
            
            json.push_str("    {\n");
            json.push_str(&format!("      \"path\": \"{}\",\n", entry.path));
            json.push_str(&format!("      \"status\": \"{}\",\n", status_str));
            json.push_str(&format!("      \"timestamp\": {},\n", entry.timestamp));
            json.push_str(&format!("      \"details\": \"{}\"\n", entry.details));
            json.push_str("    }");
            
            if i < self.entries.len() - 1 {
                json.push_str(",");
            }
            json.push('\n');
        }
        
        json.push_str("  ]\n");
        json.push_str("}\n");
        
        json
    }
    
    /// Get entries by status
    pub fn get_entries_by_status(&self, status: &CheckStatus) -> Vec<&ReportEntry> {
        self.entries.iter()
            .filter(|e| e.status == *status)
            .collect()
    }
    
    /// Get changed entries
    pub fn get_changed_entries(&self) -> Vec<&ReportEntry> {
        self.get_entries_by_status(&CheckStatus::Changed)
    }
    
    /// Get added entries
    pub fn get_added_entries(&self) -> Vec<&ReportEntry> {
        self.get_entries_by_status(&CheckStatus::Added)
    }
    
    /// Get removed entries
    pub fn get_removed_entries(&self) -> Vec<&ReportEntry> {
        self.get_entries_by_status(&CheckStatus::Removed)
    }
    
    /// Get error entries
    pub fn get_error_entries(&self) -> Vec<&ReportEntry> {
        self.get_entries_by_status(&CheckStatus::Error)
    }
}
