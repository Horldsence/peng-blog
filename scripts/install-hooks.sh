#!/usr/bin/env bash

##############################################################################
# Git Hooks Installer
# 安装 Git pre-commit hook，在提交前自动运行 CI 检查
#
# Usage: ./scripts/install-hooks.sh
##############################################################################

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
BOLD='\033[1m'

print_header() {
    echo ""
    echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${BLUE}  $1${NC}"
    echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
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
    echo -e "${BLUE}ℹ${NC} $1"
}

# 检查是否在 Git 仓库中
if [ ! -d ".git" ]; then
    print_error "Not a git repository. Please run this script from the project root."
    exit 1
fi

print_header "Git Hooks Installer"

# 确保 .git/hooks 目录存在
if [ ! -d ".git/hooks" ]; then
    mkdir -p .git/hooks
    print_info "Created .git/hooks directory"
fi

# 创建 pre-commit hook
PRE_COMMIT_HOOK=".git/hooks/pre-commit"

echo ""
print_info "Installing pre-commit hook..."

# 检查是否已存在 hook
if [ -f "$PRE_COMMIT_HOOK" ]; then
    print_warning "pre-commit hook already exists"
    echo ""
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Installation cancelled"
        exit 0
    fi
    
    # 备份现有的 hook
    BACKUP="${PRE_COMMIT_HOOK}.backup.$(date +%s)"
    mv "$PRE_COMMIT_HOOK" "$BACKUP"
    print_info "Backed up existing hook to: $BACKUP"
fi

# 写入 pre-commit hook 内容
cat > "$PRE_COMMIT_HOOK" << 'EOF'
#!/usr/bin/env bash

##############################################################################
# Git Pre-commit Hook
# 在 git commit 前自动运行 CI 检查
#
# 此文件由 scripts/install-hooks.sh 自动生成
# 如需修改，请编辑 scripts/install-hooks.sh 后重新运行安装脚本
##############################################################################

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'
BOLD='\033[1m'

echo ""
echo -e "${BOLD}${BLUE}Running pre-commit CI checks...${NC}"
echo ""

# 检查是否有暂存的文件
if git diff --cached --quiet; then
    echo -e "${YELLOW}⚠ No staged changes detected${NC}"
    echo "  Nothing to commit. Skipping CI checks."
    exit 0
fi

# 检查 CI 脚本是否存在
CI_SCRIPT="./scripts/pre-commit-ci.sh"
if [ ! -f "$CI_SCRIPT" ]; then
    echo -e "${RED}✗ CI script not found: $CI_SCRIPT${NC}"
    echo ""
    echo "Please ensure the CI script exists or run:"
    echo "  git update-index --no-skip-worktree scripts/pre-commit-ci.sh"
    exit 1
fi

# 确保 CI 脚本可执行
if [ ! -x "$CI_SCRIPT" ]; then
    chmod +x "$CI_SCRIPT"
fi

# 运行 CI 检查（快速模式）
if "$CI_SCRIPT" --fast; then
    echo ""
    echo -e "${GREEN}✓ Pre-commit checks passed${NC}"
    echo -e "${GREEN}  Proceeding with commit...${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}✗ Pre-commit checks failed${NC}"
    echo ""
    echo -e "${YELLOW}Options:${NC}"
    echo "  1. Fix the errors and try again"
    echo "  2. Run full CI check: ./scripts/pre-commit-ci.sh"
    echo "  3. Skip hook (not recommended): git commit --no-verify"
    echo ""
    exit 1
fi
EOF

# 设置 hook 为可执行
chmod +x "$PRE_COMMIT_HOOK"

print_success "pre-commit hook installed successfully"

# 创建 pre-push hook（可选）
echo ""
read -p "Do you want to install pre-push hook (full CI check before push)? (Y/n): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    PRE_PUSH_HOOK=".git/hooks/pre-push"
    
    # 备份现有的 hook
    if [ -f "$PRE_PUSH_HOOK" ]; then
        BACKUP="${PRE_PUSH_HOOK}.backup.$(date +%s)"
        mv "$PRE_PUSH_HOOK" "$BACKUP"
        print_info "Backed up existing pre-push hook to: $BACKUP"
    fi
    
    # 写入 pre-push hook 内容
    cat > "$PRE_PUSH_HOOK" << 'EOF'
#!/usr/bin/env bash

##############################################################################
# Git Pre-push Hook
# 在 git push 前运行完整的 CI 检查
#
# 此文件由 scripts/install-hooks.sh 自动生成
##############################################################################

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'
BOLD='\033[1m'

echo ""
echo -e "${BOLD}${BLUE}Running pre-push CI checks (full)...${NC}"
echo ""

# 检查 CI 脚本是否存在
CI_SCRIPT="./scripts/pre-commit-ci.sh"
if [ ! -f "$CI_SCRIPT" ]; then
    echo -e "${RED}✗ CI script not found: $CI_SCRIPT${NC}"
    exit 1
fi

# 确保 CI 脚本可执行
if [ ! -x "$CI_SCRIPT" ]; then
    chmod +x "$CI_SCRIPT"
fi

# 运行完整的 CI 检查
if "$CI_SCRIPT"; then
    echo ""
    echo -e "${GREEN}✓ Pre-push checks passed${NC}"
    echo -e "${GREEN}  Proceeding with push...${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}✗ Pre-push checks failed${NC}"
    echo ""
    echo "Please fix the errors before pushing, or use:"
    echo "  git push --no-verify  (not recommended)"
    echo ""
    exit 1
fi
EOF
    
    chmod +x "$PRE_PUSH_HOOK"
    print_success "pre-push hook installed successfully"
fi

# 显示安装总结
echo ""
print_header "Installation Complete"
echo ""
print_info "Installed hooks:"
[ -f ".git/hooks/pre-commit" ] && echo "  • pre-commit (fast mode CI check)"
[ -f ".git/hooks/pre-push" ] && echo "  • pre-push (full CI check)"

echo ""
print_info "Hook behavior:"
echo "  • pre-commit: Runs fast CI checks (--fast) before each commit"
echo "  • pre-push:   Runs full CI checks before pushing to remote"

echo ""
print_info "Manual CI check:"
echo "  • Fast:  ./scripts/pre-commit-ci.sh --fast"
echo "  • Full:  ./scripts/pre-commit-ci.sh"
echo "  • Fix:   ./scripts/pre-commit-ci.sh --fix"
echo "  • Help:  ./scripts/pre-commit-ci.sh --help"

echo ""
print_info "Bypass hooks (not recommended):"
echo "  • git commit --no-verify"
echo "  • git push --no-verify"

echo ""
print_success "Git hooks installed! Your commits will now be checked automatically."
echo ""