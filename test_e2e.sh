#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Temporary directory for tests
TEST_DIR=$(mktemp -d)
export XDG_CONFIG_HOME="$TEST_DIR/config"
SPM="cargo run --quiet --"

echo -e "${YELLOW}=== SPM End-to-End Tests ===${NC}"
echo "Test directory: $TEST_DIR"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}Cleaning up...${NC}"
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# Counters
PASSED=0
FAILED=0

# Test function
run_test() {
    local name="$1"
    local cmd="$2"
    local expected="$3"

    echo -n "Testing: $name... "

    if output=$(eval "$cmd" 2>&1); then
        if [[ -z "$expected" ]] || echo "$output" | grep -q "$expected"; then
            echo -e "${GREEN}PASSED${NC}"
            ((PASSED++))
            return 0
        fi
    fi

    echo -e "${RED}FAILED${NC}"
    echo "  Command: $cmd"
    echo "  Expected: $expected"
    echo "  Got: $output"
    ((FAILED++))
    return 1
}

# Test function for expected failures
run_test_fail() {
    local name="$1"
    local cmd="$2"

    echo -n "Testing: $name... "

    if ! eval "$cmd" 2>&1 >/dev/null; then
        echo -e "${GREEN}PASSED${NC}"
        ((PASSED++))
        return 0
    fi

    echo -e "${RED}FAILED${NC} (expected failure)"
    ((FAILED++))
    return 1
}

# Create test projects
mkdir -p "$TEST_DIR/projects/project-alpha"
mkdir -p "$TEST_DIR/projects/project-beta"
mkdir -p "$TEST_DIR/projects/rust-cli"

echo ""
echo -e "${YELLOW}--- Test Help ---${NC}"
run_test "help command" "$SPM --help" "Side Project Manager"
run_test "add help" "$SPM add --help" "Add a project"

echo ""
echo -e "${YELLOW}--- Test Add ---${NC}"
run_test "add project" "$SPM add $TEST_DIR/projects/project-alpha" "Project 'project-alpha' added"
run_test "add with name" "$SPM add $TEST_DIR/projects/project-beta --name beta" "Project 'beta' added"
run_test "add with tags" "$SPM add $TEST_DIR/projects/rust-cli --tags rust,cli" "Project 'rust-cli' added"

echo ""
echo -e "${YELLOW}--- Test List ---${NC}"
run_test "list all" "$SPM list" "project-alpha"
run_test "list shows beta" "$SPM list" "beta"
run_test "list shows tags" "$SPM list" "rust, cli"
run_test "list filter by tag" "$SPM list --tags rust" "rust-cli"
run_test "list filter no match" "$SPM list --tags nonexistent" "No projects found"

echo ""
echo -e "${YELLOW}--- Test Tag ---${NC}"
run_test "add tag" "$SPM tag project-alpha work" "Tags added to"
run_test "verify tag added" "$SPM list" "work"
run_test "remove tag" "$SPM tag project-alpha work --remove" "Tags removed from"

echo ""
echo -e "${YELLOW}--- Test Config ---${NC}"
run_test "config set default_shell" "$SPM config set default_shell zsh" "default_shell=zsh"
run_test "config get default_shell" "$SPM config get default_shell" "zsh"

echo ""
echo -e "${YELLOW}--- Test Init ---${NC}"
run_test "init zsh" "$SPM init zsh" "function sp()"
run_test "init bash" "$SPM init bash" "function sp()"
run_test "init fish" "$SPM init fish" "function sp"

echo ""
echo -e "${YELLOW}--- Test Remove ---${NC}"
run_test "remove project" "$SPM rm beta" "Project 'beta' removed"
run_test_fail "remove nonexistent" "$SPM rm nonexistent"

echo ""
echo -e "${YELLOW}--- Test Duplicates ---${NC}"
run_test_fail "add duplicate name" "$SPM add $TEST_DIR/projects/project-alpha"
run_test_fail "add duplicate path" "$SPM add $TEST_DIR/projects/project-alpha --name different-name"

echo ""
echo -e "${YELLOW}=== Results ===${NC}"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"

if [ $FAILED -gt 0 ]; then
    exit 1
fi

echo ""
echo -e "${GREEN}All tests passed!${NC}"
