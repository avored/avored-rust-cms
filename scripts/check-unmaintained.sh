#!/bin/bash
# Unmaintained Dependencies Checker
# Provides detailed analysis of unmaintained dependencies

set -euo pipefail

echo "🔍 AvoRed CMS - Unmaintained Dependencies Analysis"
echo "=================================================="
echo

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if required tools are available
check_tools() {
    local missing_tools=()
    
    if ! command -v cargo >/dev/null 2>&1; then
        missing_tools+=("cargo")
    fi
    
    if ! command -v jq >/dev/null 2>&1; then
        missing_tools+=("jq")
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        echo -e "${RED}❌ Missing required tools: ${missing_tools[*]}${NC}"
        echo "Please install the missing tools and try again."
        exit 1
    fi
}

# Run cargo audit and extract unmaintained warnings
check_unmaintained() {
    echo -e "${BLUE}📊 Running cargo audit...${NC}"
    
    # Run cargo audit and capture output
    if cargo audit --format json --quiet > audit-output.json 2>/dev/null; then
        echo -e "${GREEN}✅ No security vulnerabilities found${NC}"
    else
        echo -e "${YELLOW}⚠️ Audit completed with warnings${NC}"
    fi
    
    # Check for unmaintained dependencies
    local unmaintained_count
    unmaintained_count=$(jq -r '.warnings // [] | map(select(.kind == "unmaintained")) | length' audit-output.json 2>/dev/null || echo "0")
    
    if [ "$unmaintained_count" != "0" ] && [ "$unmaintained_count" -gt 0 ] 2>/dev/null; then
        echo -e "${YELLOW}⚠️ Found $unmaintained_count unmaintained dependencies:${NC}"
        echo
        
        # Extract and display unmaintained dependencies
        jq -r '.warnings[]? | select(.kind == "unmaintained") | 
               "📦 \(.package.name) v\(.package.version)\n" +
               "   📋 \(.advisory.title)\n" +
               "   🔗 \(.advisory.url)\n" +
               "   📅 \(.advisory.date)\n"' audit-output.json 2>/dev/null || {
            echo -e "${RED}❌ Failed to parse audit output${NC}"
            return 1
        }
        
        # Show dependency trees for unmaintained crates
        echo -e "${BLUE}🌳 Dependency Trees:${NC}"
        jq -r '.warnings[]? | select(.kind == "unmaintained") | .package.name' audit-output.json 2>/dev/null | while read -r crate; do
            echo -e "${YELLOW}└── $crate dependency tree:${NC}"
            cargo tree --invert "$crate" 2>/dev/null | head -10 || echo "   Unable to generate tree"
            echo
        done
        
    else
        echo -e "${GREEN}✅ No unmaintained dependencies detected${NC}"
    fi
    
    # Cleanup
    rm -f audit-output.json
}

# Check deny.toml configuration
check_deny_config() {
    echo -e "${BLUE}⚙️ Checking deny.toml configuration...${NC}"
    
    if [ -f "deny.toml" ]; then
        local unmaintained_policy
        unmaintained_policy=$(grep -E "^unmaintained\s*=" deny.toml | cut -d'"' -f2 2>/dev/null || echo "not configured")
        echo -e "   Unmaintained policy: ${YELLOW}$unmaintained_policy${NC}"
        
        if [ "$unmaintained_policy" = "workspace" ]; then
            echo -e "   ${GREEN}✅ Configured to track transitive unmaintained dependencies${NC}"
        elif [ "$unmaintained_policy" = "all" ]; then
            echo -e "   ${YELLOW}⚠️ Configured to fail on all unmaintained dependencies${NC}"
        else
            echo -e "   ${RED}❌ Unmaintained policy not properly configured${NC}"
        fi
    else
        echo -e "   ${RED}❌ deny.toml not found${NC}"
    fi
    echo
}

# Show summary and recommendations
show_summary() {
    echo -e "${BLUE}📋 Summary and Recommendations:${NC}"
    echo
    echo "1. Review unmaintained dependencies quarterly"
    echo "2. Monitor upstream projects for replacements"
    echo "3. Consider alternatives for direct unmaintained dependencies"
    echo "4. Update documentation in docs/security/unmaintained-dependencies.md"
    echo
    echo -e "${GREEN}📚 For more information, see: docs/security/unmaintained-dependencies.md${NC}"
}

# Main execution
main() {
    check_tools
    echo
    check_unmaintained
    echo
    check_deny_config
    show_summary
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
