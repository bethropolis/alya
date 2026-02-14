//! Result type aliases.

use super::VmError;

/// Convenience Result alias for Alya VM operations.
pub type VmResult<T> = Result<T, VmError>;
