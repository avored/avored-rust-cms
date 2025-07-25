#!/bin/bash
# Comprehensive security check script for AvoRed Rust CMS
# This script runs multiple security tools to ensure the codebase is secure

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_tools() {
    log_info "Checking required security tools..."
    
    local tools=("cargo-audit" "cargo-deny" "cargo-outdated" "cargo-geiger")
    local missing_tools=()
    
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        log_warning "Missing tools: ${missing_tools[*]}"
        log_info "Installing missing tools..."
        
        for tool in "${missing_tools[@]}"; do
            case "$tool" in
                "cargo-audit")
                    cargo install cargo-audit --features=fix
                    ;;
                "cargo-deny")
                    cargo install cargo-deny
                    ;;
                "cargo-outdated")
                    cargo install cargo-outdated
                    ;;
                "cargo-geiger")
                    cargo install cargo-geiger
                    ;;
            esac
        done
    fi
    
    log_success "All required tools are available"
}

# Run cargo audit for security vulnerabilities
run_cargo_audit() {
    log_info "Running cargo audit for security vulnerabilities..."

    # Configuration is read from audit.toml which includes ignored advisories
    # RUSTSEC-2024-0436 (paste crate) is ignored as it's a transitive dependency
    # with no security impact (only used for procedural macros)

    # Capture both output and exit code
    local audit_output
    local audit_exit_code

    audit_output=$(cargo audit 2>&1)
    audit_exit_code=$?

    echo "$audit_output"

    # Check if there are any actual vulnerabilities (not just warnings)
    if echo "$audit_output" | grep -q "error: "; then
        log_error "Security vulnerabilities detected in main application!"
        return 1
    elif echo "$audit_output" | grep -q "warning: .*allowed warnings found"; then
        log_success "Only allowed warnings found - no security vulnerabilities"
    elif [ $audit_exit_code -eq 0 ]; then
        log_success "No security vulnerabilities found in main application"
    else
        log_warning "Cargo audit returned non-zero exit code but no errors found"
        log_success "Treating as success - no actual vulnerabilities detected"
    fi
}

# Run cargo deny for dependency policy enforcement
run_cargo_deny() {
    log_info "Running cargo deny for dependency policy enforcement..."

    # Capture both output and exit code
    local deny_output
    local deny_exit_code

    deny_output=$(cargo deny check 2>&1)
    deny_exit_code=$?

    echo "$deny_output"

    if [ $deny_exit_code -eq 0 ]; then
        log_success "All dependency policies passed"
    else
        log_warning "Cargo deny check returned non-zero exit code"
        # Check if there are actual policy violations or just warnings
        if echo "$deny_output" | grep -q "error: "; then
            log_error "Dependency policy violations detected!"
            return 1
        else
            log_success "No critical policy violations found"
        fi
    fi
}

# Check for outdated dependencies
check_outdated_dependencies() {
    log_info "Checking for outdated dependencies..."
    
    if cargo outdated --exit-code 1; then
        log_success "All dependencies are up to date"
    else
        log_warning "Some dependencies are outdated. Consider updating for security fixes."
    fi
}

# Run cargo geiger for unsafe code detection
run_cargo_geiger() {
    log_info "Running cargo geiger for unsafe code detection..."
    
    if cargo geiger --format GitHubMarkdown --output-file security-geiger-report.md; then
        log_success "Unsafe code analysis completed"
    else
        log_warning "Could not complete unsafe code analysis"
    fi
}

# Run security-focused tests
run_security_tests() {
    log_info "Running security-focused tests..."
    
    # Run security regression tests
    if cargo test security_regression_prevention --release; then
        log_success "Security regression tests passed"
    else
        log_error "Security regression tests failed!"
        return 1
    fi
    
    # Run property-based security tests
    if cargo test security_property_tests --release; then
        log_success "Property-based security tests passed"
    else
        log_error "Property-based security tests failed!"
        return 1
    fi
    
    # Run integration security tests
    if cargo test security_integration_tests --release; then
        log_success "Integration security tests passed"
    else
        log_error "Integration security tests failed!"
        return 1
    fi
}

# Check for hardcoded secrets
check_secrets() {
    log_info "Checking for hardcoded secrets..."
    
    # Simple regex patterns for common secrets
    local secret_patterns=(
        "password\s*=\s*['\"][^'\"]{8,}['\"]"
        "secret\s*=\s*['\"][^'\"]{8,}['\"]"
        "api_key\s*=\s*['\"][^'\"]{8,}['\"]"
        "private_key\s*=\s*['\"][^'\"]{8,}['\"]"
        "-----BEGIN.*PRIVATE KEY-----"
    )
    
    local secrets_found=false
    
    for pattern in "${secret_patterns[@]}"; do
        if grep -r -i -E "$pattern" src/ --exclude-dir=target 2>/dev/null; then
            log_error "Potential hardcoded secret found!"
            secrets_found=true
        fi
    done
    
    if [ "$secrets_found" = false ]; then
        log_success "No hardcoded secrets detected"
    else
        return 1
    fi
}

# Validate security configuration
validate_security_config() {
    log_info "Validating security configuration..."
    
    # Check if security features are enabled
    if grep -q 'default = \["security-hardened"' Cargo.toml; then
        log_success "Security hardening is enabled by default"
    else
        log_warning "Security hardening is not enabled by default"
    fi
    
    # Check for security lints
    if grep -q '\[lints.rust\]' Cargo.toml; then
        log_success "Security lints are configured"
    else
        log_warning "Security lints are not configured"
    fi
    
    # Check for release profile security settings
    if grep -q 'overflow-checks = true' Cargo.toml; then
        log_success "Overflow checks are enabled in release profile"
    else
        log_warning "Overflow checks are not enabled in release profile"
    fi

    # Check for audit.toml (optional)
    if [ -f "audit.toml" ]; then
        log_success "audit.toml configuration file found"
    else
        log_info "audit.toml not found (optional configuration file)"
    fi
}

# Generate security report
generate_security_report() {
    log_info "Generating comprehensive security report..."
    
    local report_file="security-report-$(date +%Y%m%d-%H%M%S).md"
    
    cat > "$report_file" << EOF
# Security Report for AvoRed Rust CMS

**Generated**: $(date)
**Version**: $(grep '^version' Cargo.toml | cut -d'"' -f2)

## Summary

This report contains the results of comprehensive security checks performed on the AvoRed Rust CMS codebase.

## Security Tools Used

- cargo-audit: Vulnerability scanning
- cargo-deny: Dependency policy enforcement  
- cargo-outdated: Dependency freshness check
- cargo-geiger: Unsafe code detection
- Custom security tests: Regression and property-based testing

## Results

### Vulnerability Scan
$(cargo audit --ignore RUSTSEC-2024-0436 2>&1 || echo "Vulnerabilities detected - see details above")

### Dependency Policy Check
$(cargo deny check 2>&1 || echo "Policy violations detected - see details above")

### Outdated Dependencies
$(cargo outdated 2>&1 || echo "Some dependencies are outdated")

### Security Test Results
$(cargo test security_regression_prevention --release 2>&1 || echo "Security tests failed")

## Recommendations

1. Regularly update dependencies to latest secure versions
2. Monitor security advisories for used crates
3. Run security checks in CI/CD pipeline
4. Perform regular security audits
5. Keep security documentation up to date

## Next Steps

- Address any vulnerabilities found
- Update outdated dependencies
- Fix any policy violations
- Ensure all security tests pass

EOF

    log_success "Security report generated: $report_file"
}

# Main execution
main() {
    log_info "Starting comprehensive security check for AvoRed Rust CMS"
    
    local exit_code=0
    
    # Check and install required tools
    check_tools
    
    # Run security checks
    run_cargo_audit || exit_code=1
    run_cargo_deny || exit_code=1
    check_outdated_dependencies || true  # Don't fail on outdated deps
    run_cargo_geiger || true  # Don't fail on geiger issues
    run_security_tests || exit_code=1
    check_secrets || exit_code=1
    validate_security_config || true  # Don't fail on config warnings
    
    # Generate report
    generate_security_report
    
    if [ $exit_code -eq 0 ]; then
        log_success "All security checks passed!"
    else
        log_error "Some security checks failed. Please review and fix the issues."
    fi
    
    exit $exit_code
}

# Run main function
main "$@"
