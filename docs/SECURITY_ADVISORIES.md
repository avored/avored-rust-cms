# Security Advisories Tracking

This document tracks security advisories and their resolution status for the AvoRed Rust CMS project.

## Currently Ignored Advisories

### RUSTSEC-2024-0436: paste crate unmaintained

**Status**: Ignored (Low Risk)  
**Date Added**: 2025-07-19  
**Review Date**: 2025-Q2  

**Description**: The `paste` crate (v1.0.15) has been marked as unmaintained by its author.

**Risk Assessment**: 
- **Severity**: Low
- **Impact**: No runtime security impact
- **Usage**: Only used for procedural macros at compile time
- **Exposure**: Transitive dependency only, not directly used in our codebase

**Dependency Chain**:
```
avored-rust-cms 
└── surrealdb 2.3.7
    └── surrealdb-core 2.3.7
        └── ext-sort 0.1.5
            └── rmp-serde 1.3.0
                └── rmp 0.8.14
                    └── paste 1.0.15 (unmaintained)
```

**Mitigation Strategy**:
1. Monitor SurrealDB releases for migration away from `rmp` dependency
2. Consider alternative serialization libraries if SurrealDB doesn't address this
3. Regular review of this advisory status

**Resolution Plan**:
- **Short-term**: Advisory ignored in audit.toml due to low risk
- **Medium-term**: Monitor SurrealDB 3.x releases for dependency updates
- **Long-term**: Evaluate alternative database solutions if issue persists

## Monitoring Process

1. **Quarterly Review**: Review all ignored advisories every quarter
2. **Dependency Updates**: Check for SurrealDB updates that might resolve transitive dependencies
3. **Risk Reassessment**: Reassess risk levels when new information becomes available

## Contact

For security concerns, contact: security@avored.com

---
*Last Updated: 2025-07-19*
*Next Review: 2025-Q2*
