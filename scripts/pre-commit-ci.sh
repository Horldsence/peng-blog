#!/usr/bin/env bash

##############################################################################
# Pre-commit CI Check Script
# 本地 CI 检查脚本 - 严格遵照 GitHub CI 标准
#
# Usage: ./scripts/pre-commit-ci.sh [options]
#
# Options:
#   --fast         快速模式（跳过 build 和部分测试）
#   --skip-tests   跳过测试
#   --fix          自动修复格式问题
#   --verbose      详细输出
#   --help         显示帮助信息
##############################################################################

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# 配置变量
FAST_MODE=false
SKIP_TESTS=false
AUTO_FIX=false
VERBOSE=false

# 统计变量
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
START_TIME=$(date +%s)

##############################################################################
# 辅助函数
##############################################################################

print_header() {
    echo ""
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${CYAN}  $1${NC}"
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_step() {
    echo ""
    echo -e "${BOLD}${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo -e "${CYAN}ℹ${NC} $1"
}

# 记录检查结果
check_passed() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    PASSED_CHECKS=$((PASSED_CHECKS + 1))
    print_success "$1"
}

check_failed() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    FAILED_CHECKS=$((FAILED_CHECKS + 1))
    print_error "$1"
}

# 显示帮助信息
show_help() {
    cat << EOF
${BOLD}Peng Blog - Local CI Check Script${NC}

${BOLD}USAGE:${NC}
    ./scripts/pre-commit-ci.sh [OPTIONS]

${BOLD}OPTIONS:${NC}
    --fast          快速模式（跳过 build 和部分测试）
    --skip-tests    跳过所有测试
    --fix           自动修复格式问题（运行 cargo fmt）
    --verbose       显示详细输出
    --help          显示此帮助信息

${BOLD}EXAMPLES:${NC}
    # 完整检查（推荐在提交前运行）
    ./scripts/pre-commit-ci.sh

    # 快速检查（开发过程中使用）
    ./scripts/pre-commit-ci.sh --fast

    # 自动修复格式问题
    ./scripts/pre-commit-ci.sh --fix

    # 跳过测试的快速检查
    ./scripts/pre-commit-ci.sh --fast --skip-tests

${BOLD}CI CHECKS:${NC}
    1. Format Check     - 代码格式检查 (cargo fmt)
    2. Clippy          - 代码质量检查 (cargo clippy -D warnings)
    3. Code Check      - 编译检查 (cargo check)
    4. Test Suite      - 单元测试 (cargo test)
    5. Doc Tests       - 文档测试 (cargo test --doc)
    6. Build           - Release 构建 (cargo build --release)

EOF
    exit 0
}

# 解析命令行参数
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --fast)
                FAST_MODE=true
                shift
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --fix)
                AUTO_FIX=true
                shift
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --help|-h)
                show_help
                ;;
            *)
                echo -e "${RED}Error: Unknown option '$1'${NC}"
                echo "Run './scripts/pre-commit-ci.sh --help' for usage information."
                exit 1
                ;;
        esac
    done
}

# 检查依赖
check_dependencies() {
    print_step "Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "cargo not found. Please install Rust toolchain."
        exit 1
    fi
    
    if ! command -v rustfmt &> /dev/null; then
        print_warning "rustfmt not found. Installing..."
        rustup component add rustfmt
    fi
    
    if ! command -v cargo-clippy &> /dev/null; then
        print_warning "clippy not found. Installing..."
        rustup component add clippy
    fi
    
    print_success "All dependencies available"
}

# 运行命令（支持详细模式）
run_cmd() {
    local description=$1
    shift
    
    if [ "$VERBOSE" = true ]; then
        echo -e "${CYAN}Running: $@${NC}"
        "$@"
    else
        "$@" > /tmp/ci-output.log 2>&1
    fi
}

##############################################################################
# CI 检查步骤（严格遵照 GitHub CI）
##############################################################################

# 1. Format Check
check_format() {
    print_step "1/6 Format Check (cargo fmt --check)"
    
    if [ "$AUTO_FIX" = true ]; then
        print_info "Auto-fixing format issues..."
        if run_cmd "cargo fmt" cargo fmt --all; then
            check_passed "Format: All files formatted"
        else
            check_failed "Format: Failed to format files"
            return 1
        fi
    else
        if run_cmd "cargo fmt check" cargo fmt --all -- --check; then
            check_passed "Format: All files are properly formatted"
        else
            check_failed "Format: Some files need formatting"
            print_warning "Run './scripts/pre-commit-ci.sh --fix' to auto-fix"
            return 1
        fi
    fi
}

# 2. Clippy Check
check_clippy() {
    print_step "2/6 Clippy (cargo clippy -D warnings)"
    
    # 设置环境变量（与 CI 一致）
    export CARGO_TERM_COLOR=always
    export RUST_BACKTRACE=1
    
    if run_cmd "cargo clippy" cargo clippy --all-targets --all-features --workspace -- -D warnings; then
        check_passed "Clippy: No warnings or errors"
    else
        check_failed "Clippy: Found warnings or errors"
        if [ "$VERBOSE" = false ]; then
            echo ""
            echo -e "${YELLOW}Showing last 30 lines of output:${NC}"
            tail -n 30 /tmp/ci-output.log
        fi
        return 1
    fi
}

# 3. Code Check
check_code() {
    print_step "3/6 Check (cargo check)"
    
    if run_cmd "cargo check" cargo check --all-features --workspace; then
        check_passed "Check: Code compiles successfully"
    else
        check_failed "Check: Compilation failed"
        if [ "$VERBOSE" = false ]; then
            echo ""
            echo -e "${YELLOW}Showing last 30 lines of output:${NC}"
            tail -n 30 /tmp/ci-output.log
        fi
        return 1
    fi
}

# 4. Test Suite
check_tests() {
    if [ "$SKIP_TESTS" = true ]; then
        print_step "4/6 Test Suite (SKIPPED)"
        print_warning "Tests skipped by user request"
        return 0
    fi
    
    print_step "4/6 Test Suite (cargo test)"
    
    if [ "$FAST_MODE" = true ]; then
        print_info "Fast mode: Running tests without verbose output"
        if run_cmd "cargo test" cargo test --all-features --workspace; then
            check_passed "Tests: All tests passed"
        else
            check_failed "Tests: Some tests failed"
            return 1
        fi
    else
        print_info "Running tests with verbose output (like CI)..."
        if run_cmd "cargo test verbose" cargo test --all-features --workspace --verbose; then
            check_passed "Tests: All tests passed"
        else
            check_failed "Tests: Some tests failed"
            if [ "$VERBOSE" = false ]; then
                echo ""
                echo -e "${YELLOW}Showing last 50 lines of output:${NC}"
                tail -n 50 /tmp/ci-output.log
            fi
            return 1
        fi
    fi
}

# 5. Doc Tests
check_doc_tests() {
    if [ "$SKIP_TESTS" = true ]; then
        print_step "5/6 Doc Tests (SKIPPED)"
        print_warning "Doc tests skipped by user request"
        return 0
    fi
    
    print_step "5/6 Doc Tests (cargo test --doc)"
    
    if run_cmd "cargo test doc" cargo test --doc --workspace; then
        check_passed "Doc Tests: All documentation tests passed"
    else
        check_failed "Doc Tests: Some documentation tests failed"
        return 1
    fi
}

# 6. Build Release
check_build() {
    if [ "$FAST_MODE" = true ]; then
        print_step "6/6 Build (SKIPPED in fast mode)"
        print_warning "Release build skipped in fast mode"
        return 0
    fi
    
    print_step "6/6 Build (cargo build --release)"
    
    print_info "Building release binary (this may take a while)..."
    if run_cmd "cargo build release" cargo build --release --workspace; then
        check_passed "Build: Release build successful"
        
        # 显示构建产物信息
        if [ -f "target/release/peng-blog" ]; then
            local size=$(du -h target/release/peng-blog | cut -f1)
            print_info "Binary size: $size (target/release/peng-blog)"
        fi
    else
        check_failed "Build: Release build failed"
        if [ "$VERBOSE" = false ]; then
            echo ""
            echo -e "${YELLOW}Showing last 30 lines of output:${NC}"
            tail -n 30 /tmp/ci-output.log
        fi
        return 1
    fi
}

##############################################################################
# 生成检查报告
##############################################################################

generate_summary() {
    local end_time=$(date +%s)
    local duration=$((end_time - START_TIME))
    local minutes=$((duration / 60))
    local seconds=$((duration % 60))
    
    echo ""
    print_header "CI Summary"
    echo ""
    
    # 检查结果表格
    echo -e "${BOLD}Check Results:${NC}"
    echo ""
    printf "  %-20s %s\n" "Check" "Status"
    printf "  %-20s %s\n" "──────────────────" "────────"
    
    # 根据执行情况显示结果
    printf "  %-20s " "1. Format"
    [ $TOTAL_CHECKS -ge 1 ] && [ $FAILED_CHECKS -eq 0 ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}"
    
    printf "  %-20s " "2. Clippy"
    [ $TOTAL_CHECKS -ge 2 ] && [ $FAILED_CHECKS -le 1 ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}"
    
    printf "  %-20s " "3. Check"
    [ $TOTAL_CHECKS -ge 3 ] && [ $FAILED_CHECKS -le 2 ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}"
    
    printf "  %-20s " "4. Tests"
    if [ "$SKIP_TESTS" = true ]; then
        echo -e "${YELLOW}⊘ SKIP${NC}"
    else
        [ $TOTAL_CHECKS -ge 4 ] && [ $FAILED_CHECKS -le 3 ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}"
    fi
    
    printf "  %-20s " "5. Doc Tests"
    if [ "$SKIP_TESTS" = true ]; then
        echo -e "${YELLOW}⊘ SKIP${NC}"
    else
        [ $TOTAL_CHECKS -ge 5 ] && [ $FAILED_CHECKS -le 4 ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}"
    fi
    
    printf "  %-20s " "6. Build"
    if [ "$FAST_MODE" = true ]; then
        echo -e "${YELLOW}⊘ SKIP${NC}"
    else
        [ $TOTAL_CHECKS -ge 6 ] && [ $FAILED_CHECKS -eq 0 ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}"
    fi
    
    echo ""
    echo -e "${BOLD}Statistics:${NC}"
    echo "  Total Checks:  $TOTAL_CHECKS"
    echo "  Passed:        $PASSED_CHECKS"
    echo "  Failed:        $FAILED_CHECKS"
    echo "  Duration:      ${minutes}m ${seconds}s"
    
    echo ""
    
    if [ $FAILED_CHECKS -eq 0 ]; then
        echo -e "${BOLD}${GREEN}✓ All CI checks passed!${NC}"
        echo -e "${GREEN}  Your code is ready to commit.${NC}"
        return 0
    else
        echo -e "${BOLD}${RED}✗ CI checks failed${NC}"
        echo -e "${RED}  Please fix the errors above before committing.${NC}"
        echo ""
        echo -e "${YELLOW}Tips:${NC}"
        echo "  • Run with --verbose to see full output"
        echo "  • Run with --fix to auto-fix format issues"
        echo "  • Check the error messages above for details"
        return 1
    fi
}

##############################################################################
# 主流程
##############################################################################

main() {
    # 清屏（可选）
    # clear
    
    print_header "Peng Blog - Local CI Check"
    echo ""
    echo -e "${BOLD}Running pre-commit CI checks (matching GitHub CI standards)${NC}"
    
    if [ "$FAST_MODE" = true ]; then
        print_info "Fast mode enabled: Skipping build and using quick tests"
    fi
    
    if [ "$SKIP_TESTS" = true ]; then
        print_warning "Tests will be skipped"
    fi
    
    if [ "$AUTO_FIX" = true ]; then
        print_info "Auto-fix mode enabled: Will format code automatically"
    fi
    
    # 检查依赖
    check_dependencies
    
    # 执行所有检查
    local overall_result=0
    
    check_format || overall_result=1
    check_clippy || overall_result=1
    check_code || overall_result=1
    check_tests || overall_result=1
    check_doc_tests || overall_result=1
    check_build || overall_result=1
    
    # 生成总结
    generate_summary || overall_result=1
    
    # 清理临时文件
    rm -f /tmp/ci-output.log
    
    exit $overall_result
}

##############################################################################
# 脚本入口
##############################################################################

# 解析参数
parse_args "$@"

# 执行主流程
main