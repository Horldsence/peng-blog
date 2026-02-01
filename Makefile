##############################################################################
# Peng Blog - Makefile
# 提供快捷命令用于开发、测试和部署
##############################################################################

.PHONY: help dev build test check ci ci-fast clean install-hooks fmt clippy doc run release

# 默认目标
.DEFAULT_GOAL := help

##############################################################################
# 帮助信息
##############################################################################

help: ## 显示帮助信息
	@echo ""
	@echo "Peng Blog - Makefile Commands"
	@echo "=============================="
	@echo ""
	@echo "Development:"
	@echo "  make dev          - 启动开发服务器"
	@echo "  make run          - 运行应用（等同于 cargo run）"
	@echo "  make watch        - 监视文件变化并自动重新编译"
	@echo ""
	@echo "Build:"
	@echo "  make build        - 构建 debug 版本"
	@echo "  make build-all    - 构建前端和后端（debug）"
	@echo "  make release      - 构建 release 版本"
	@echo "  make release-all  - 构建前端和后端（release）"
	@echo "  make clean        - 清理构建产物"
	@echo ""
	@echo "Testing:"
	@echo "  make test         - 运行所有测试"
	@echo "  make test-unit    - 仅运行单元测试"
	@echo "  make test-doc     - 运行文档测试"
	@echo "  make test-verbose - 运行测试（详细输出）"
	@echo "  make coverage     - 生成测试覆盖率报告"
	@echo ""
	@echo "Code Quality:"
	@echo "  make check        - 编译检查"
	@echo "  make fmt          - 格式化代码"
	@echo "  make fmt-check    - 检查代码格式"
	@echo "  make clippy       - 运行 clippy 检查"
	@echo "  make doc          - 生成文档"
	@echo ""
	@echo "CI Checks:"
	@echo "  make ci           - 运行完整 CI 检查（提交前推荐）"
	@echo "  make ci-fast      - 运行快速 CI 检查（开发时使用）"
	@echo "  make ci-fix       - 运行 CI 并自动修复格式问题"
	@echo "  make install-hooks - 安装 Git pre-commit hooks"
	@echo ""
	@echo "Database:"
	@echo "  make db-migrate   - 运行数据库迁移"
	@echo "  make db-reset     - 重置数据库（危险！）"
	@echo "  make db-status    - 查看数据库状态"
	@echo ""
	@echo "User Management:"
	@echo "  make user-list    - 列出所有用户"
	@echo "  make user-create  - 创建管理员用户"
	@echo ""
	@echo "Frontend:"
	@echo "  make frontend-dev   - 启动前端开发服务器"
	@echo "  make frontend-build - 构建前端生产版本"
	@echo "  make frontend-install - 安装前端依赖"
	@echo ""

##############################################################################
# 开发命令
##############################################################################

dev: ## 启动开发服务器
	@echo "Starting development server..."
	cargo run

run: ## 运行应用
	cargo run

watch: ## 监视文件变化并自动重新编译
	@if ! command -v cargo-watch >/dev/null 2>&1; then \
		echo "Installing cargo-watch..."; \
		cargo install cargo-watch; \
	fi
	cargo watch -x run

##############################################################################
# 构建命令
##############################################################################

build: ## 构建 debug 版本
	@echo "Building debug version..."
	cargo build --workspace

build-all: frontend-build build ## 构建前端和后端（debug）
	@echo ""
	@echo "Full build complete!"
	@echo "Frontend: ./dist"
	@echo "Backend: ./target/debug/peng-blog"
	@echo ""
	@echo "Run with: cargo run"
	@echo "Server will serve frontend at http://localhost:3000"

release: ## 构建 release 版本
	@echo "Building release version..."
	cargo build --release --workspace
	@echo ""
	@echo "Build complete! Binary location:"
	@ls -lh target/release/peng-blog 2>/dev/null || echo "Binary not found"

release-all: frontend-build release ## 构建前端和后端（release）
	@echo ""
	@echo "Full production build complete!"
	@echo "Frontend: ./dist"
	@echo "Backend: ./target/release/peng-blog"
	@echo ""
	@echo "Run with: ./target/release/peng-blog"
	@echo "Server will serve frontend at http://localhost:3000"

clean: ## 清理构建产物
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Cleaning temporary files..."
	rm -f /tmp/ci-output.log
	rm -f coverage/*.html coverage/*.json 2>/dev/null || true
	@echo "Cleaning frontend dist..."
	rm -rf dist
	@echo "Clean complete!"

##############################################################################
# 测试命令
##############################################################################

test: ## 运行所有测试
	@echo "Running all tests..."
	cargo test --all-features --workspace

test-unit: ## 仅运行单元测试
	@echo "Running unit tests..."
	cargo test --lib --all-features --workspace

test-doc: ## 运行文档测试
	@echo "Running documentation tests..."
	cargo test --doc --workspace

test-verbose: ## 运行测试（详细输出）
	@echo "Running tests with verbose output..."
	cargo test --all-features --workspace --verbose

test-service: ## 仅测试 service 层
	@echo "Running service layer tests..."
	cargo test -p service

test-api: ## 仅测试 API 层
	@echo "Running API layer tests..."
	cargo test -p api

coverage: ## 生成测试覆盖率报告
	@if ! command -v cargo-tarpaulin >/dev/null 2>&1; then \
		echo "Installing cargo-tarpaulin..."; \
		cargo install cargo-tarpaulin; \
	fi
	@echo "Generating coverage report..."
	@mkdir -p coverage
	cargo tarpaulin --out Html --output-dir ./coverage --all-features --workspace
	@echo ""
	@echo "Coverage report generated: coverage/index.html"

##############################################################################
# 代码质量命令
##############################################################################

check: ## 编译检查
	@echo "Running cargo check..."
	cargo check --all-features --workspace

fmt: ## 格式化代码
	@echo "Formatting code..."
	cargo fmt --all

fmt-check: ## 检查代码格式
	@echo "Checking code format..."
	cargo fmt --all -- --check

clippy: ## 运行 clippy 检查
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features --workspace -- -D warnings

clippy-fix: ## 自动修复 clippy 警告
	@echo "Auto-fixing clippy warnings..."
	cargo clippy --all-targets --all-features --workspace --fix --allow-dirty

doc: ## 生成文档
	@echo "Generating documentation..."
	cargo doc --no-deps --workspace --open

doc-private: ## 生成文档（包括私有项）
	@echo "Generating documentation (including private items)..."
	cargo doc --no-deps --workspace --document-private-items --open

##############################################################################
# CI 检查命令
##############################################################################

ci: ## 运行完整 CI 检查
	@echo "Running full CI checks..."
	@chmod +x scripts/pre-commit-ci.sh
	@./scripts/pre-commit-ci.sh

ci-fast: ## 运行快速 CI 检查
	@echo "Running fast CI checks..."
	@chmod +x scripts/pre-commit-ci.sh
	@./scripts/pre-commit-ci.sh --fast

ci-fix: ## 运行 CI 并自动修复格式问题
	@echo "Running CI with auto-fix..."
	@chmod +x scripts/pre-commit-ci.sh
	@./scripts/pre-commit-ci.sh --fix

ci-skip-tests: ## 运行 CI（跳过测试）
	@echo "Running CI without tests..."
	@chmod +x scripts/pre-commit-ci.sh
	@./scripts/pre-commit-ci.sh --fast --skip-tests

ci-verbose: ## 运行 CI（详细输出）
	@echo "Running CI with verbose output..."
	@chmod +x scripts/pre-commit-ci.sh
	@./scripts/pre-commit-ci.sh --verbose

install-hooks: ## 安装 Git pre-commit hooks
	@echo "Installing Git hooks..."
	@chmod +x scripts/install-hooks.sh
	@./scripts/install-hooks.sh

##############################################################################
# 数据库命令
##############################################################################

db-migrate: ## 运行数据库迁移
	@echo "Running database migrations..."
	cargo run -- db migrate

db-reset: ## 重置数据库（危险！）
	@echo "⚠️  WARNING: This will delete ALL data!"
	@read -p "Are you sure? (yes/no): " confirm; \
	if [ "$$confirm" = "yes" ]; then \
		cargo run -- db reset --force; \
	else \
		echo "Database reset cancelled."; \
	fi

db-status: ## 查看数据库状态
	@echo "Checking database status..."
	cargo run -- db status

##############################################################################
# 用户管理命令
##############################################################################

user-list: ## 列出所有用户
	@echo "Listing all users..."
	cargo run -- user list

user-create: ## 创建管理员用户
	@echo "Creating admin user..."
	@read -p "Username: " username; \
	read -sp "Password: " password; \
	echo ""; \
	cargo run -- user create --username "$$username" --password "$$password" --admin

user-show: ## 显示用户详情（需要提供 user-id）
	@if [ -z "$(USER_ID)" ]; then \
		echo "Error: USER_ID is required"; \
		echo "Usage: make user-show USER_ID=<uuid>"; \
		exit 1; \
	fi
	cargo run -- user show $(USER_ID)

##############################################################################
# 前端命令
##############################################################################

frontend-install: ## 安装前端依赖
	@echo "Installing frontend dependencies..."
	cd frontend && npm install

frontend-dev: ## 启动前端开发服务器
	@echo "Starting frontend development server..."
	cd frontend && npm run dev

frontend-build: ## 构建前端生产版本
	@echo "Building frontend for production..."
	cd frontend && npm run build

frontend-lint: ## 检查前端代码
	@echo "Linting frontend code..."
	cd frontend && npm run lint

frontend-clean: ## 清理前端构建产物
	@echo "Cleaning frontend build artifacts..."
	rm -rf dist
	cd frontend && rm -rf node_modules/.vite

##############################################################################
# 组合命令
##############################################################################

all: clean build test ## 清理、构建、测试

pre-commit: ci-fast ## 提交前检查（快速）

pre-push: ci ## 推送前检查（完整）

setup: install-hooks frontend-install ## 初始化开发环境
	@echo ""
	@echo "Development environment setup complete!"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Copy .env.example to .env and configure"
	@echo "  2. Run 'make db-migrate' to initialize database"
	@echo "  3. Run 'make user-create' to create admin user"
	@echo "  4. Run 'make build-all' to build frontend and backend"
	@echo "  5. Run 'make dev' or 'cargo run' to start the server"
	@echo ""

full-check: fmt clippy test ## 完整代码检查（格式化、clippy、测试）

quick: fmt-check check ## 快速检查（不运行测试）

##############################################################################
# 实用工具
##############################################################################

deps-update: ## 更新依赖
	@echo "Updating dependencies..."
	cargo update

deps-tree: ## 显示依赖树
	@if ! command -v cargo-tree >/dev/null 2>&1; then \
		echo "cargo-tree is built-in since Rust 1.44"; \
	fi
	cargo tree

deps-outdated: ## 检查过时的依赖
	@if ! command -v cargo-outdated >/dev/null 2>&1; then \
		echo "Installing cargo-outdated..."; \
		cargo install cargo-outdated; \
	fi
	cargo outdated

audit: ## 安全审计
	@if ! command -v cargo-audit >/dev/null 2>&1; then \
		echo "Installing cargo-audit..."; \
		cargo install cargo-audit; \
	fi
	cargo audit

bloat: ## 分析二进制文件大小
	@if ! command -v cargo-bloat >/dev/null 2>&1; then \
		echo "Installing cargo-bloat..."; \
		cargo install cargo-bloat; \
	fi
	cargo bloat --release

bench: ## 运行性能测试
	@echo "Running benchmarks..."
	cargo bench

##############################################################################
# 环境信息
##############################################################################

info: ## 显示环境信息
	@echo "System Information:"
	@echo "==================="
	@echo "OS:           $$(uname -s)"
	@echo "Architecture: $$(uname -m)"
	@echo ""
	@echo "Rust Toolchain:"
	@echo "==============="
	@rustc --version
	@cargo --version
	@echo ""
	@echo "Project Info:"
	@echo "============="
	@echo "Database:     $$(ls -lh blog.db 2>/dev/null | awk '{print $$5}' || echo 'Not found')"
	@echo "Binary:       $$(ls -lh target/release/peng-blog 2>/dev/null | awk '{print $$5}' || echo 'Not built')"
	@echo ""

status: ## 显示项目状态
	@echo "Project Status:"
	@echo "==============="
	@echo ""
	@echo "Git Status:"
	@git status -s || echo "Not a git repository"
	@echo ""
	@echo "Build Status:"
	@if [ -f "target/release/peng-blog" ]; then \
		echo "✓ Release binary exists"; \
		ls -lh target/release/peng-blog; \
	else \
		echo "✗ Release binary not found"; \
	fi
	@echo ""
	@echo "Database Status:"
	@if [ -f "blog.db" ]; then \
		echo "✓ Database exists"; \
		ls -lh blog.db; \
	else \
		echo "✗ Database not found"; \
	fi