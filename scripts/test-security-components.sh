#!/bin/bash
# Individual Security Component Testing Script
# Test specific components of the security infrastructure

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Function to test individual components
test_component() {
    local component="$1"
    
    case "$component" in
        "security-config")
            log_info "Testing security configuration..."
            make -f Makefile.security security-validate-config
            ;;
        "security-audit")
            log_info "Testing security audit..."
            cargo audit --ignore RUSTSEC-2025-0021 --ignore RUSTSEC-2024-0350 --ignore RUSTSEC-2024-0348 --ignore RUSTSEC-2024-0352 --ignore RUSTSEC-2024-0351 --ignore RUSTSEC-2024-0335 --ignore RUSTSEC-2024-0349 --ignore RUSTSEC-2024-0353 --ignore RUSTSEC-2025-0001 --ignore RUSTSEC-2024-0436 --ignore RUSTSEC-2024-0359 --ignore RUSTSEC-2022-0092
            ;;
        "dependency-policy")
            log_info "Testing dependency policy..."
            cargo deny check
            ;;
        "ldap-build")
            log_info "Testing LDAP authentication build..."
            cargo check --features ldap-auth
            ;;
        "security-build")
            log_info "Testing security-hardened build..."
            cargo build --profile release-secure --features security-hardened
            ;;
        "security-lints")
            log_info "Testing security lints..."
            cargo clippy --profile release-secure --features security-hardened -- \
                -W clippy::unwrap_used \
                -W clippy::panic \
                -W clippy::expect_used
            ;;
        "comprehensive-check")
            log_info "Running comprehensive security check..."
            ./scripts/security-check.sh
            ;;
        "yaml-validation")
            log_info "Validating GitHub workflow YAML files..."
            python3 -c "
import yaml
files = ['.github/workflows/rust.yml', '.github/workflows/security-audit.yml', '.github/workflows/zero-trust-security.yml']
for f in files:
    try:
        yaml.safe_load(open(f))
        print(f'✅ {f}: Valid')
    except Exception as e:
        print(f'❌ {f}: {e}')
        exit(1)
"
            ;;
        "all")
            log_info "Running all component tests..."
            for comp in security-config security-audit dependency-policy ldap-build security-build security-lints yaml-validation; do
                echo ""
                log_info "Testing component: $comp"
                test_component "$comp" || log_error "Component $comp failed"
            done
            ;;
        *)
            log_error "Unknown component: $component"
            echo "Available components:"
            echo "  security-config     - Test security configuration"
            echo "  security-audit      - Test security audit"
            echo "  dependency-policy   - Test dependency policy"
            echo "  ldap-build         - Test LDAP authentication build"
            echo "  security-build     - Test security-hardened build"
            echo "  security-lints     - Test security lints"
            echo "  comprehensive-check - Run comprehensive security check"
            echo "  yaml-validation    - Validate GitHub workflow YAML"
            echo "  all               - Run all tests"
            exit 1
            ;;
    esac
}

# Main execution
if [ $# -eq 0 ]; then
    log_info "Usage: $0 <component>"
    test_component "unknown"
else
    test_component "$1"
fi
