//! SigmaOS-aide — File integrity checker model.

pub mod util;
pub mod options;

pub use util::{Aide, FileHash, CheckResult};
pub use options::{AideOptions, Mode};
