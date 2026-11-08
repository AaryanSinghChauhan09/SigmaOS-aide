//! Command-line options for aide.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Initialize database
    Init,
    /// Check integrity
    Check,
    /// Update database
    Update,
    /// Generate report
    Report,
    /// Configure
    Config,
    /// Database operations
    Database,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Database path
    pub database_path: String,
    /// Paths to check
    pub paths: Vec<String>,
    /// Output file
    pub output_file: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Check,
            config_path: String::new(),
            database_path: String::new(),
            paths: vec![],
            output_file: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
