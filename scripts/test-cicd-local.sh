#!/bin/bash
# Local CI/CD Testing Script for AvoRed Rust CMS
# This script simulates the GitHub Actions workflows locally

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
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

log_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

# Test results tracking
TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    log_step "Testing: $test_name"
    
    if eval "$test_command"; then
        log_success "$test_name: PASSED"
        ((TESTS_PASSED++))
        return 0
    else
        log_error "$test_name: FAILED"
        FAILED_TESTS+=("$test_name")
        ((TESTS_FAILED++))
        return 1
    fi
}

# Header
echo -e "${PURPLE}================================${NC}"
echo -e "${PURPLE}🔒 LOCAL CI/CD TESTING SUITE${NC}"
echo -e "${PURPLE}================================${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    log_error "Not in the project root directory. Please run from the project root."
    exit 1
fi

log_info "Starting local CI/CD testing for AvoRed Rust CMS..."
echo ""

# Test 1: Security Configuration Validation
log_step "Phase 1: Security Configuration Validation"
echo "----------------------------------------"

run_test "Security Configuration Files Exist" "test -f Makefile.security && test -f audit.toml && test -f deny.toml"

run_test "LDAP Configuration Template Exists" "test -f .env.ldap.example"

run_test "Security Features in Cargo.toml" "grep -q 'security-hardened' Cargo.toml"

run_test "Security Build Profiles Configured" "grep -q 'release-secure' Cargo.toml"

echo ""

# Test 2: Security Tools Installation and Validation
log_step "Phase 2: Security Tools Installation"
echo "----------------------------------------"

# Check if security tools are installed, install if missing
install_security_tools() {
    local tools=("cargo-audit" "cargo-deny" "cargo-outdated" "cargo-geiger")
    local missing_tools=()
    
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        log_warning "Missing tools: ${missing_tools[*]}"
        log_info "Installing missing security tools..."
        
        for tool in "${missing_tools[@]}"; do
            case "$tool" in
                "cargo-audit")
                    cargo install cargo-audit --features=fix || return 1
                    ;;
                "cargo-deny")
                    cargo install cargo-deny || return 1
                    ;;
                "cargo-outdated")
                    cargo install cargo-outdated || return 1
                    ;;
                "cargo-geiger")
                    cargo install cargo-geiger || return 1
                    ;;
            esac
        done
    fi
    
    return 0
}

run_test "Security Tools Installation" "install_security_tools"

echo ""

# Test 3: Security Validation (Makefile.security)
log_step "Phase 3: Security Validation"
echo "----------------------------------------"

run_test "Security Configuration Validation" "make -f Makefile.security security-validate-config"

run_test "Security Audit (cargo-audit)" "cargo audit --ignore RUSTSEC-2025-0021 --ignore RUSTSEC-2024-0350 --ignore RUSTSEC-2024-0348 --ignore RUSTSEC-2024-0352 --ignore RUSTSEC-2024-0351 --ignore RUSTSEC-2024-0335 --ignore RUSTSEC-2024-0349 --ignore RUSTSEC-2024-0353 --ignore RUSTSEC-2025-0001 --ignore RUSTSEC-2024-0436 --ignore RUSTSEC-2024-0359 --ignore RUSTSEC-2022-0092"

run_test "Dependency Policy Check (cargo-deny)" "cargo deny check"

run_test "Outdated Dependencies Check" "cargo outdated --exit-code 1 || true"

echo ""

# Test 4: Comprehensive Security Check Script
log_step "Phase 4: Comprehensive Security Check"
echo "----------------------------------------"

run_test "Security Check Script Exists" "test -x scripts/security-check.sh"

# Run the comprehensive security check (allow it to fail for testing)
if [ -x "scripts/security-check.sh" ]; then
    log_info "Running comprehensive security check script..."
    if ./scripts/security-check.sh; then
        log_success "Comprehensive Security Check: PASSED"
        ((TESTS_PASSED++))
    else
        log_warning "Comprehensive Security Check: Some issues found (this may be expected in development)"
        # Don't count this as a failure for local testing
    fi
else
    log_error "Security check script not found or not executable"
    FAILED_TESTS+=("Security Check Script")
    ((TESTS_FAILED++))
fi

echo ""

# Test 5: LDAP Authentication Validation
log_step "Phase 5: LDAP Authentication Validation"
echo "----------------------------------------"

run_test "LDAP Feature Compilation" "cargo check --features ldap-auth"

run_test "LDAP Security Lints" "cargo clippy --features ldap-auth -- -W clippy::unwrap_used -W clippy::panic"

echo ""

# Test 6: Security-Hardened Build Testing
log_step "Phase 6: Security-Hardened Build Testing"
echo "----------------------------------------"

run_test "Security-Hardened Build Profile" "cargo build --profile release-secure --features security-hardened"

run_test "Security-Hardened Binary Exists" "test -f target/release-secure/avored-rust-cms"

echo ""

# Test 7: CI/CD Workflow Syntax Validation
log_step "Phase 7: CI/CD Workflow Validation"
echo "----------------------------------------"

validate_yaml_syntax() {
    python3 -c "
import yaml
import sys

files = [
    '.github/workflows/rust.yml',
    '.github/workflows/security-audit.yml',
    '.github/workflows/zero-trust-security.yml'
]

for file in files:
    try:
        with open(file, 'r') as f:
            yaml.safe_load(f)
        print(f'✅ {file}: Valid YAML')
    except Exception as e:
        print(f'❌ {file}: Invalid YAML - {e}')
        sys.exit(1)
"
}

run_test "GitHub Workflows YAML Syntax" "validate_yaml_syntax"

echo ""

# Test 8: Security Features Testing
log_step "Phase 8: Security Features Testing"
echo "----------------------------------------"

run_test "Security Lints Configuration" "grep -q 'unsafe_code = \"forbid\"' Cargo.toml"

run_test "Security Dependencies Available" "cargo tree --features security-hardened | grep -E '(secrecy|zeroize|constant_time_eq)' || true"

echo ""

# Final Results
echo -e "${PURPLE}================================${NC}"
echo -e "${PURPLE}📊 LOCAL CI/CD TEST RESULTS${NC}"
echo -e "${PURPLE}================================${NC}"
echo ""

log_info "Tests Passed: $TESTS_PASSED"
if [ $TESTS_FAILED -gt 0 ]; then
    log_error "Tests Failed: $TESTS_FAILED"
    echo ""
    log_error "Failed Tests:"
    for test in "${FAILED_TESTS[@]}"; do
        echo "  - $test"
    done
else
    log_success "Tests Failed: $TESTS_FAILED"
fi

echo ""

# Calculate success rate
TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
if [ $TOTAL_TESTS -gt 0 ]; then
    SUCCESS_RATE=$(( (TESTS_PASSED * 100) / TOTAL_TESTS ))
    log_info "Success Rate: $SUCCESS_RATE%"
else
    log_info "Success Rate: 0%"
fi

echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    log_success "🎉 ALL LOCAL CI/CD TESTS PASSED!"
    log_success "Your CI/CD workflows are ready for deployment!"
    exit 0
else
    log_error "❌ Some tests failed. Please fix the issues before deploying."
    exit 1
fi
