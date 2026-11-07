//! Aide model — file integrity monitoring.

#[derive(Debug, Clone)]
pub struct FileHash {
    pub path: String,
    pub md5: String,
    pub sha256: String,
    pub size: u64,
    pub mtime: u64,
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub checked: usize,
    pub changed: usize,
    pub added: usize,
    pub removed: usize,
    pub errors: usize,
}

#[derive(Debug, Clone)]
pub struct Aide {
    pub config: Option<String>,
    pub database: Option<String>,
}

impl Default for Aide {
    fn default() -> Self { Self { config: None, database: None } }
}

impl Aide {
    pub fn new() -> Self { Self::default() }

    pub fn init(&self) -> Vec<FileHash> {
        vec![FileHash { path: "/bin/sh".into(), md5: "abc".into(), sha256: "def".into(), size: 100, mtime: 1000 }]
    }

    pub fn check(&self) -> CheckResult {
        CheckResult { checked: 1, changed: 0, added: 0, removed: 0, errors: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_check() {
        let a = Aide::new();
        assert_eq!(a.init().len(), 1);
        assert_eq!(a.check().checked, 1);
    }
}
