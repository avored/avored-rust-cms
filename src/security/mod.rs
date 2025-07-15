/// Security module for comprehensive security management
/// This module provides zero trust security validation and monitoring

pub mod invariants;

pub use invariants::{
    SecurityInvariantChecker,
    RuntimeSecurityMonitor,
    SecurityHealthReport,
    SecurityStatus,
};

// Re-export security macros
pub use crate::{
    security_invariant,
    ensure_input_validated,
    ensure_timing_consistency,
};
