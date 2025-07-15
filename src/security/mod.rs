/// Security module for comprehensive security management
/// This module provides zero trust security validation and monitoring
pub mod invariants;

pub use invariants::{
    RuntimeSecurityMonitor, SecurityHealthReport, SecurityInvariantChecker, SecurityStatus,
};

// Re-export security macros
pub use crate::{ensure_input_validated, ensure_timing_consistency, security_invariant};
