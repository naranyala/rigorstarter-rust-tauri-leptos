#!/usr/bin/env bash
# Leptos-specific linting script
# Detects legacy patterns (0.5/0.6), missing move closures, and common mistakes
# Usage: ./scripts/lint-leptos.sh [--fix]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SRC_DIR="$PROJECT_ROOT/src"

RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

ERRORS=0
WARNINGS=0
FIX_MODE=false

if [[ "${1:-}" == "--fix" ]]; then
    FIX_MODE=true
    echo -e "${CYAN}Running in FIX mode - will attempt auto-fixes${NC}"
fi

echo -e "${CYAN}=== Leptos Pattern Linter (v0.7 compatible) ===${NC}"
echo ""

# Helper function to scan files
scan_files() {
    local pattern="$1"
    local message="$2"
    local severity="$3"
    
    local results
    results=$(rg -n "$pattern" "$SRC_DIR" --glob '*.rs' 2>/dev/null || true)
    
    if [[ -n "$results" ]]; then
        if [[ "$severity" == "ERROR" ]]; then
            ERRORS=$((ERRORS + $(echo "$results" | wc -l)))
        else
            WARNINGS=$((WARNINGS + $(echo "$results" | wc -l)))
        fi
        
        echo -e "${CYAN}--- $message ---${NC}"
        echo "$results" | while IFS= read -r line; do
            if [[ "$severity" == "ERROR" ]]; then
                echo -e "  ${RED}[ERROR]${NC} $line"
            else
                echo -e "  ${YELLOW}[WARN]${NC} $line"
            fi
        done
        echo ""
    fi
}

# ============================================================
# SECTION 1: LEGACY LEPTOS 0.5/0.6 PATTERNS (ERRORS)
# ============================================================
echo -e "${RED}Checking for legacy Leptos 0.5/0.6 patterns...${NC}"
echo ""

# create_signal -> signal
scan_files 'create_signal\(' 'create_signal() is legacy, use signal() instead' 'ERROR'

# create_rw_signal -> RwSignal::new
scan_files 'create_rw_signal\(' 'create_rw_signal() is legacy, use RwSignal::new() instead' 'ERROR'

# create_memo -> Memo::new
scan_files 'create_memo\(' 'create_memo() is legacy, use Memo::new() instead' 'ERROR'

# create_effect -> Effect::new
scan_files 'create_effect\(' 'create_effect() is legacy, use Effect::new() instead' 'ERROR'

# create_isomorphic_effect -> Effect::new_isomorphic
scan_files 'create_isomorphic_effect\(' 'create_isomorphic_effect() is legacy, use Effect::new_isomorphic() instead' 'ERROR'

# create_resource -> Resource::new
scan_files 'create_resource\(' 'create_resource() is legacy, use Resource::new() instead' 'ERROR'

# create_local_resource -> LocalResource::new
scan_files 'create_local_resource\(' 'create_local_resource() is legacy, use LocalResource::new() instead' 'ERROR'

# create_trigger -> Trigger::new
scan_files 'create_trigger\(' 'create_trigger() is legacy, use Trigger::new() instead' 'ERROR'

# create_node_ref -> NodeRef
scan_files 'create_node_ref\(' 'create_node_ref() is legacy, use NodeRef::new() instead' 'ERROR'

# Signal::derive -> Memo::new or appropriate alternative
scan_files 'Signal::derive\(' 'Signal::derive() is legacy, use Memo::new() or appropriate alternative' 'ERROR'

# MaybeSignal::derive
scan_files 'MaybeSignal::derive\(' 'MaybeSignal::derive() is legacy, use Memo::new() or Signal::derive()' 'ERROR'

# read_signal() -> .get() or .with()
scan_files '\.read\(\)' 'Signal::read() is legacy, use .get() or .with() instead' 'ERROR'

# write_signal() -> .set() or .update()
scan_files '\.write\(\)' 'Signal::write() is legacy, use .set() or .update() instead' 'ERROR'

# .with_untracked() -> .with_untracked() still exists but check usage
# scan_files '\.with_untracked\(' '.with_untracked() may need review for proper reactivity' 'WARN'

# ============================================================
# SECTION 2: MISSING MOVE CLOSURES (WARNINGS)
# ============================================================
echo -e "${YELLOW}Checking for potential missing 'move' closures...${NC}"
echo ""

# Pattern: when= followed by closure without move that might access signals
# This is a heuristic - not 100% accurate but catches common cases
scan_files 'when=[^m][^o][^v][^e]' 'Potential missing move in when= closure' 'WARN'

# Pattern: fallback= followed by closure without move
scan_files 'fallback=[^m][^o][^v][^e]' 'Potential missing move in fallback= closure' 'WARN'

# Pattern: on:click= without move when followed by signal access
scan_files 'on:click=[^m][^o][^v][^e]' 'Potential missing move in on:click= handler' 'WARN'

# Pattern: on:input= without move
scan_files 'on:input=[^m][^o][^v][^e]' 'Potential missing move in on:input= handler' 'WARN'

# Pattern: on:change= without move
scan_files 'on:change=[^m][^o][^v][^e]' 'Potential missing move in on:change= handler' 'WARN'

# Pattern: on:submit= without move
scan_files 'on:submit=[^m][^o][^v][^e]' 'Potential missing move in on:submit= handler' 'WARN'

# ============================================================
# SECTION 3: COMPONENT DEFINITION ISSUES (WARNINGS)
# ============================================================
echo -e "${YELLOW}Checking component definitions...${NC}"
echo ""

# Functions returning impl IntoView without #[component]
# This is tricky with regex, but we can catch obvious cases
results=$(rg -n 'pub fn \w+\(\) -> impl IntoView' "$SRC_DIR" --glob '*.rs' -B1 2>/dev/null | rg -v '#\[component\]' | rg 'pub fn' || true)
if [[ -n "$results" ]]; then
    WARNINGS=$((WARNINGS + $(echo "$results" | wc -l)))
    echo -e "${CYAN}--- Functions returning impl IntoView without #[component] ---${NC}"
    echo "$results" | while IFS= read -r line; do
        echo -e "  ${YELLOW}[WARN]${NC} $line"
    done
    echo ""
fi

# ============================================================
# SECTION 4: REACTIVITY ANTI-PATTERNS (WARNINGS)
# ============================================================
echo -e "${YELLOW}Checking reactivity anti-patterns...${NC}"
echo ""

# Signal.get() inside a non-reactive context (hard to detect, but flag .get() in non-closure)
# This is a heuristic - look for .get() outside of move || or |_|
results=$(rg -n '\.get\(\)' "$SRC_DIR" --glob '*.rs' 2>/dev/null | rg -v 'move \|\|' | rg -v '\|_\|' | rg -v 'fn ' | rg -v '//' | head -20 || true)
if [[ -n "$results" ]]; then
    WARNINGS=$((WARNINGS + $(echo "$results" | wc -l)))
    echo -e "${CYAN}--- .get() calls outside reactive closures (may not trigger updates) ---${NC}"
    echo "$results" | while IFS= read -r line; do
        echo -e "  ${YELLOW}[WARN]${NC} $line"
    done
    echo ""
fi

# ============================================================
# SECTION 5: VIEW MACRO ISSUES (WARNINGS)
# ============================================================
echo -e "${YELLOW}Checking view! macro patterns...${NC}"
echo ""

# Check for string interpolation that should use {}
scan_files '\{[a-zA-Z_][a-zA-Z0-9_]*\}' 'Signal access in view! - ensure wrapped in move || or is reactive' 'WARN'

# Check for class= without move when using signal
scan_files 'class:[a-zA-Z_]+=[^m]' 'Potential missing move in class: directive' 'WARN'

# ============================================================
# SECTION 6: AUTO-FIXES (if --fix flag)
# ============================================================
if [[ "$FIX_MODE" == true ]]; then
    echo -e "${CYAN}Applying auto-fixes...${NC}"
    echo ""
    
    # Fix create_signal -> signal
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_signal(/signal(/g' {} +
    echo -e "  ${GREEN}Fixed: create_signal() -> signal()${NC}"
    
    # Fix create_rw_signal -> RwSignal::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_rw_signal(/RwSignal::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_rw_signal() -> RwSignal::new()${NC}"
    
    # Fix create_memo -> Memo::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_memo(/Memo::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_memo() -> Memo::new()${NC}"
    
    # Fix create_effect -> Effect::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_effect(/Effect::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_effect() -> Effect::new()${NC}"
    
    # Fix create_isomorphic_effect -> Effect::new_isomorphic
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_isomorphic_effect(/Effect::new_isomorphic(/g' {} +
    echo -e "  ${GREEN}Fixed: create_isomorphic_effect() -> Effect::new_isomorphic()${NC}"
    
    # Fix create_resource -> Resource::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_resource(/Resource::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_resource() -> Resource::new()${NC}"
    
    # Fix create_local_resource -> LocalResource::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_local_resource(/LocalResource::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_local_resource() -> LocalResource::new()${NC}"
    
    # Fix create_trigger -> Trigger::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_trigger(/Trigger::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_trigger() -> Trigger::new()${NC}"
    
    # Fix create_node_ref -> NodeRef::new
    find "$SRC_DIR" -name '*.rs' -exec sed -i 's/create_node_ref(/NodeRef::new(/g' {} +
    echo -e "  ${GREEN}Fixed: create_node_ref() -> NodeRef::new()${NC}"
    
    echo ""
    echo -e "${GREEN}Auto-fixes applied. Review changes with git diff.${NC}"
fi

# ============================================================
# SUMMARY
# ============================================================
echo ""
echo -e "${CYAN}=== Summary ===${NC}"
if [[ $ERRORS -gt 0 ]]; then
    echo -e "${RED}$ERRORS error(s) found${NC}"
fi
if [[ $WARNINGS -gt 0 ]]; then
    echo -e "${YELLOW}$WARNINGS warning(s) found${NC}"
fi
if [[ $ERRORS -eq 0 && $WARNINGS -eq 0 ]]; then
    echo -e "${GREEN}No issues found!${NC}"
fi

exit $ERRORS
