//! Configuration for aide.

/// Rule type
#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    /// Monitor specific file
    File,
    /// Monitor directory
    Directory,
    /// Monitor directory recursively
    Recursive,
    /// Exclude path
    Exclude,
    /// Include path
    Include,
}

/// Monitoring rule
#[derive(Debug, Clone)]
pub struct MonitoringRule {
    /// Rule path
    pub path: String,
    /// Rule type
    pub rule_type: RuleType,
    /// Rule enabled
    pub enabled: bool,
    /// Rule description
    pub description: String,
    /// Rule priority
    pub priority: u32,
}

impl MonitoringRule {
    /// Create new monitoring rule
    pub fn new(path: String, rule_type: RuleType) -> Self {
        Self {
            path,
            rule_type,
            enabled: true,
            description: String::new(),
            priority: 0,
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    
    /// Enable rule
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable rule
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Set priority
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }
}

/// Aide configuration
#[derive(Debug, Clone)]
pub struct AideConfig {
    /// Database path
    pub database_path: String,
    /// Config file path
    pub config_path: String,
    /// Monitoring rules
    pub rules: Vec<MonitoringRule>,
    /// Hash algorithms to use
    pub hash_algorithms: Vec<String>,
    /// Check interval in seconds
    pub check_interval: u64,
    /// Enable real-time monitoring
    pub real_time_monitoring: bool,
    /// Enable email notifications
    pub email_notifications: bool,
    /// Email recipients
    pub email_recipients: Vec<String>,
    /// Report output directory
    pub report_output_dir: String,
    /// Verbose logging
    pub verbose_logging: bool,
}

impl Default for AideConfig {
    fn default() -> Self {
        Self {
            database_path: "/var/lib/aide/aide.db".into(),
            config_path: "/etc/aide.conf".into(),
            rules: vec![],
            hash_algorithms: vec!["md5".into(), "sha256".into()],
            check_interval: 86400, // 24 hours
            real_time_monitoring: false,
            email_notifications: false,
            email_recipients: vec![],
            report_output_dir: "/var/log/aide".into(),
            verbose_logging: false,
        }
    }
}

impl AideConfig {
    /// Create new aide config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add monitoring rule
    pub fn add_rule(&mut self, rule: MonitoringRule) {
        self.rules.push(rule);
    }
    
    /// Remove monitoring rule
    pub fn remove_rule(&mut self, path: &str) -> bool {
        if let Some(pos) = self.rules.iter().position(|r| r.path == path) {
            self.rules.remove(pos);
            return true;
        }
        false
    }
    
    /// Get monitoring rules
    pub fn get_rules(&self) -> &[MonitoringRule] {
        &self.rules
    }
    
    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&MonitoringRule> {
        self.rules.iter().filter(|r| r.enabled).collect()
    }
    
    /// Add hash algorithm
    pub fn add_hash_algorithm(&mut self, algorithm: String) {
        if !self.hash_algorithms.contains(&algorithm) {
            self.hash_algorithms.push(algorithm);
        }
    }
    
    /// Remove hash algorithm
    pub fn remove_hash_algorithm(&mut self, algorithm: &str) -> bool {
        if let Some(pos) = self.hash_algorithms.iter().position(|a| a == algorithm) {
            self.hash_algorithms.remove(pos);
            return true;
        }
        false
    }
    
    /// Set check interval
    pub fn set_check_interval(&mut self, interval: u64) {
        self.check_interval = interval;
    }
    
    /// Enable real-time monitoring
    pub fn enable_real_time_monitoring(&mut self) {
        self.real_time_monitoring = true;
    }
    
    /// Disable real-time monitoring
    pub fn disable_real_time_monitoring(&mut self) {
        self.real_time_monitoring = false;
    }
    
    /// Enable email notifications
    pub fn enable_email_notifications(&mut self) {
        self.email_notifications = true;
    }
    
    /// Disable email notifications
    pub fn disable_email_notifications(&mut self) {
        self.email_notifications = false;
    }
    
    /// Add email recipient
    pub fn add_email_recipient(&mut self, email: String) {
        if !self.email_recipients.contains(&email) {
            self.email_recipients.push(email);
        }
    }
    
    /// Remove email recipient
    pub fn remove_email_recipient(&mut self, email: &str) -> bool {
        if let Some(pos) = self.email_recipients.iter().position(|e| e == email) {
            self.email_recipients.remove(pos);
            return true;
        }
        false
    }
    
    /// Set report output directory
    pub fn set_report_output_dir(&mut self, dir: String) {
        self.report_output_dir = dir;
    }
    
    /// Enable verbose logging
    pub fn enable_verbose_logging(&mut self) {
        self.verbose_logging = true;
    }
    
    /// Disable verbose logging
    pub fn disable_verbose_logging(&mut self) {
        self.verbose_logging = false;
    }
}
