# Unmaintained Dependencies Management

This document outlines our approach to handling unmaintained dependencies in the AvoRed Rust CMS project.

## Current Status

### Tracked Unmaintained Dependencies

| Crate | Version | Advisory | Source | Status | Review Date |
|-------|---------|----------|---------|---------|-------------|
| `paste` | 1.0.15 | RUSTSEC-2024-0436 | SurrealDB → rmp | Monitoring | 2025-Q2 |

## Policy

### Direct Dependencies
- **Action**: Replace immediately with maintained alternatives
- **Timeline**: Within 30 days of unmaintained status
- **Approval**: Required from security team

### Transitive Dependencies
- **Action**: Monitor and track via upstream dependencies
- **Timeline**: Review quarterly or when upstream updates available
- **Escalation**: If no upstream movement after 6 months, consider alternatives

## Monitoring Process

### Automated Tracking
- Security audit workflow tracks all unmaintained dependencies
- Reports generated on every CI run
- Artifacts uploaded for historical tracking

### Manual Review Process
1. **Quarterly Review** (Q1, Q2, Q3, Q4)
   - Assess current unmaintained dependencies
   - Check for upstream progress on replacements
   - Evaluate risk vs. effort of replacement

2. **Risk Assessment Criteria**
   - Functionality criticality
   - Security exposure surface
   - Availability of maintained alternatives
   - Effort required for replacement

### Decision Matrix

| Risk Level | Criteria | Action |
|------------|----------|---------|
| **Low** | Simple utility, stable, low exposure | Monitor |
| **Medium** | Core functionality, some exposure | Active tracking + upstream pressure |
| **High** | Security-critical, high exposure | Immediate replacement planning |
| **Critical** | Known vulnerabilities + unmaintained | Emergency replacement |

## Current Assessments

### paste v1.0.15 (RUSTSEC-2024-0436)
- **Risk Level**: Low
- **Rationale**: 
  - Simple procedural macro for token pasting
  - Stable functionality unlikely to need updates
  - Transitive dependency via SurrealDB
  - No known security vulnerabilities
- **Action**: Monitor SurrealDB for migration away from `rmp` crate
- **Next Review**: 2025-Q2

## Escalation Process

1. **Low → Medium**: Unmaintained for 6+ months with no upstream movement
2. **Medium → High**: Security advisory issued or critical bug found
3. **High → Critical**: Active exploitation or severe vulnerability

## Tools and Automation

### cargo-audit Configuration
- Configured in `audit.toml`
- Tracks but doesn't fail CI on unmaintained transitive deps
- Generates JSON reports for analysis

### cargo-deny Configuration  
- Configured in `deny.toml`
- Policy: `unmaintained = "workspace"` (fail on direct, warn on transitive)
- Comprehensive tracking of all dependency policies

### CI Integration
- Automated reporting in security audit workflow
- Historical tracking via artifacts
- Integration with security summary reports

## Resources

- [RustSec Advisory Database](https://rustsec.org/advisories/)
- [cargo-audit Documentation](https://docs.rs/cargo-audit/)
- [cargo-deny Documentation](https://embarkstudios.github.io/cargo-deny/)
- [Rust Security Response WG](https://www.rust-lang.org/governance/wgs/wg-security-response)
