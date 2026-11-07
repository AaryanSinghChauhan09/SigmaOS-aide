//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Init,
    Check,
    Update,
    Config,
    Database,
}

#[derive(Debug, Clone)]
pub struct AideOptions {
    pub mode: Mode,
    pub config: Option<String>,
}

impl Default for AideOptions {
    fn default() -> Self { Self { mode: Mode::Check, config: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(AideOptions::default().mode, Mode::Check); }
}
