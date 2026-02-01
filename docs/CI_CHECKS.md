# CI Checks - 本地 CI 检查指南

> 本文档介绍如何在本地运行与 GitHub CI 完全一致的代码检查，确保提交前代码质量。

## 目录

- [快速开始](#快速开始)
- [CI 检查工具](#ci-检查工具)
- [Git Hooks](#git-hooks)
- [Makefile 命令](#makefile-命令)
- [检查项说明](#检查项说明)
- [常见问题](#常见问题)
- [最佳实践](#最佳实践)

---

## 快速开始

### 1. 安装 Git Hooks（推荐）

```bash
# 方式一：使用安装脚本
./scripts/install-hooks.sh

# 方式二：使用 Makefile
make install-hooks
```

安装后，每次 `git commit` 和 `git push` 都会自动运行 CI 检查。

### 2. 手动运行 CI 检查

```bash
# 快速检查（开发时使用）
./scripts/pre-commit-ci.sh --fast
make ci-fast

# 完整检查（提交前推荐）
./scripts/pre-commit-ci.sh
make ci

# 自动修复格式问题
./scripts/pre-commit-ci.sh --fix
make ci-fix
```

### 3. 查看帮助信息

```bash
# CI 脚本帮助
./scripts/pre-commit-ci.sh --help

# Makefile 帮助
make help
```

---

## CI 检查工具

### 主脚本：`scripts/pre-commit-ci.sh`

这是核心 CI 检查脚本，严格遵照 GitHub CI 配置 (`.github/workflows/ci.yml`)。

**特点：**
- ✅ 与 GitHub CI 完全一致的检查标准
- ✅ 彩色输出，易于阅读
- ✅ 详细的错误报告
- ✅ 支持多种运行模式

**命令选项：**

| 选项 | 说明 | 适用场景 |
|------|------|----------|
| 无参数 | 完整 CI 检查 | 提交前、推送前 |
| `--fast` | 快速模式（跳过 build） | 开发过程中 |
| `--skip-tests` | 跳过所有测试 | 仅检查代码质量 |
| `--fix` | 自动修复格式问题 | 格式化代码 |
| `--verbose` | 显示详细输出 | 调试 CI 失败 |
| `--help` | 显示帮助信息 | 查看用法 |

**使用示例：**

```bash
# 1. 完整检查（提交前推荐）
./scripts/pre-commit-ci.sh
# 检查项：Format → Clippy → Check → Tests → Doc Tests → Build
# 耗时：~2-5 分钟

# 2. 快速检查（开发时使用）
./scripts/pre-commit-ci.sh --fast
# 检查项：Format → Clippy → Check → Tests → Doc Tests（跳过 Build）
# 耗时：~30 秒 - 1 分钟

# 3. 自动修复格式问题
./scripts/pre-commit-ci.sh --fix
# 运行 cargo fmt，然后执行完整检查

# 4. 仅检查代码质量（不运行测试）
./scripts/pre-commit-ci.sh --fast --skip-tests
# 检查项：Format → Clippy → Check
# 耗时：~10-20 秒

# 5. 详细输出（调试用）
./scripts/pre-commit-ci.sh --verbose
# 显示所有命令的完整输出
```

---

## Git Hooks

### 安装 Hooks

运行安装脚本：

```bash
./scripts/install-hooks.sh
```

或使用 Makefile：

```bash
make install-hooks
```

### Hook 类型

#### 1. Pre-commit Hook

**触发时机：** 执行 `git commit` 时

**检查内容：** 快速 CI 检查（`--fast` 模式）

**行为：**
- 检查是否有暂存的文件
- 运行快速 CI 检查（跳过 build）
- 失败时阻止提交

**绕过方式：**

```bash
# 不推荐：跳过 pre-commit 检查
git commit --no-verify
git commit -n
```

#### 2. Pre-push Hook

**触发时机：** 执行 `git push` 时

**检查内容：** 完整 CI 检查

**行为：**
- 运行完整的 CI 检查（包括 build）
- 失败时阻止推送

**绕过方式：**

```bash
# 不推荐：跳过 pre-push 检查
git push --no-verify
```

### Hook 文件位置

```
.git/
└── hooks/
    ├── pre-commit      # 提交前检查
    ├── pre-push        # 推送前检查
    ├── pre-commit.backup.xxxxx  # 备份文件（如果有）
    └── pre-push.backup.xxxxx    # 备份文件（如果有）
```

### 卸载 Hooks

```bash
# 删除 pre-commit hook
rm .git/hooks/pre-commit

# 删除 pre-push hook
rm .git/hooks/pre-push

# 恢复备份（如果需要）
mv .git/hooks/pre-commit.backup.xxxxx .git/hooks/pre-commit
```

---

## Makefile 命令

项目根目录的 `Makefile` 提供了丰富的快捷命令。

### CI 相关命令

```bash
# 完整 CI 检查
make ci

# 快速 CI 检查
make ci-fast

# 自动修复格式问题
make ci-fix

# 跳过测试的检查
make ci-skip-tests

# 详细输出
make ci-verbose

# 安装 Git hooks
make install-hooks
```

### 代码质量命令

```bash
# 格式化代码
make fmt

# 检查格式
make fmt-check

# 运行 clippy
make clippy

# 自动修复 clippy 警告
make clippy-fix

# 编译检查
make check

# 生成文档
make doc
```

### 测试命令

```bash
# 运行所有测试
make test

# 仅单元测试
make test-unit

# 文档测试
make test-doc

# 详细输出
make test-verbose

# 测试特定层
make test-service    # Service 层
make test-api        # API 层

# 生成覆盖率报告
make coverage
```

### 组合命令

```bash
# 完整检查（格式化、clippy、测试）
make full-check

# 快速检查（格式、编译）
make quick

# 提交前检查
make pre-commit

# 推送前检查
make pre-push

# 清理、构建、测试
make all

# 初始化开发环境
make setup
```

### 查看所有命令

```bash
make help
```

---

## 检查项说明

本地 CI 检查严格遵照 GitHub CI (`.github/workflows/ci.yml`) 的标准。

### 1. Format Check

**检查内容：** 代码格式是否符合 Rust 标准

**命令：** `cargo fmt --all -- --check`

**标准：**
- 使用 `rustfmt` 默认配置
- 所有 `.rs` 文件必须正确格式化

**修复方式：**

```bash
# 自动格式化
cargo fmt --all
make fmt

# 或使用 CI 脚本
./scripts/pre-commit-ci.sh --fix
make ci-fix
```

**常见错误：**
- 缩进不正确
- 行尾空格
- 导入顺序错误

### 2. Clippy

**检查内容：** 代码质量和潜在问题

**命令：** `cargo clippy --all-targets --all-features --workspace -- -D warnings`

**标准：**
- 所有 clippy 警告视为错误（`-D warnings`）
- 检查所有目标（lib、bin、tests、examples）
- 启用所有特性

**常见警告：**
- 未使用的变量/函数
- 不必要的克隆
- 可以简化的代码
- 潜在的性能问题

**修复方式：**

```bash
# 查看警告
cargo clippy --all-targets --all-features --workspace

# 自动修复（部分）
cargo clippy --all-targets --all-features --workspace --fix --allow-dirty
make clippy-fix

# 手动修复后重新检查
make clippy
```

### 3. Code Check

**检查内容：** 代码是否能够编译

**命令：** `cargo check --all-features --workspace`

**标准：**
- 所有 workspace 成员必须编译通过
- 启用所有特性

**常见错误：**
- 语法错误
- 类型错误
- 缺少依赖
- 模块路径错误

### 4. Test Suite

**检查内容：** 单元测试和集成测试

**命令：** `cargo test --all-features --workspace --verbose`

**标准：**
- 所有测试必须通过
- 包括单元测试和集成测试
- 详细输出（与 CI 一致）

**运行测试：**

```bash
# 所有测试
make test

# 详细输出
make test-verbose

# 特定包
cargo test -p service
cargo test -p api

# 特定测试
cargo test test_create_post
```

### 5. Doc Tests

**检查内容：** 文档中的示例代码

**命令：** `cargo test --doc --workspace`

**标准：**
- 文档注释中的示例代码必须能够编译和运行
- 所有 `/// # Examples` 中的代码都会被测试

**示例：**

```rust
/// 创建新文章
///
/// # Examples
///
/// ```no_run
/// use service::PostService;
/// 
/// let service = PostService::new(repo);
/// let post = service.create_post(request).await?;
/// ```
pub async fn create_post(&self, request: CreatePost) -> Result<Post> {
    // 实现
}
```

### 6. Build Release

**检查内容：** Release 版本是否能够构建

**命令：** `cargo build --release --workspace`

**标准：**
- Release 优化必须成功
- 所有 workspace 成员都能构建

**注意：**
- 此步骤在 `--fast` 模式下跳过
- 构建时间较长（~1-3 分钟）

---

## 检查流程图

```
┌─────────────────────────────────────────────────────────────┐
│                     CI Check Process                        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │  1. Format Check │
                    └──────────────────┘
                              │
                         ✓ Pass │ ✗ Fail → Exit
                              ▼
                    ┌──────────────────┐
                    │    2. Clippy     │
                    └──────────────────┘
                              │
                         ✓ Pass │ ✗ Fail → Exit
                              ▼
                    ┌──────────────────┐
                    │   3. Code Check  │
                    └──────────────────┘
                              │
                         ✓ Pass │ ✗ Fail → Exit
                              ▼
                    ┌──────────────────┐
                    │   4. Test Suite  │
                    └──────────────────┘
                              │
                         ✓ Pass │ ✗ Fail → Exit
                              ▼
                    ┌──────────────────┐
                    │   5. Doc Tests   │
                    └──────────────────┘
                              │
                         ✓ Pass │ ✗ Fail → Exit
                              ▼
                    ┌──────────────────┐
                    │  6. Build Release│ (skipped in --fast)
                    └──────────────────┘
                              │
                         ✓ Pass │ ✗ Fail → Exit
                              ▼
                    ┌──────────────────┐
                    │   All Passed! ✓  │
                    └──────────────────┘
```

---

## 常见问题

### Q1: CI 检查失败，如何查看详细错误？

```bash
# 使用 --verbose 标志
./scripts/pre-commit-ci.sh --verbose

# 或直接运行失败的命令
cargo clippy --all-targets --all-features --workspace
cargo test --all-features --workspace --verbose
```

### Q2: 如何跳过某个特定的检查？

CI 脚本不支持跳过单个检查，但可以使用以下组合：

```bash
# 跳过所有测试
./scripts/pre-commit-ci.sh --skip-tests

# 跳过 build（快速模式）
./scripts/pre-commit-ci.sh --fast

# 仅检查格式和编译
./scripts/pre-commit-ci.sh --fast --skip-tests
```

如需更细粒度的控制，直接运行 cargo 命令：

```bash
make fmt-check   # 仅格式检查
make clippy      # 仅 clippy
make check       # 仅编译检查
make test        # 仅测试
```

### Q3: 格式检查失败，如何修复？

```bash
# 自动修复所有格式问题
cargo fmt --all
make fmt

# 或使用 CI 脚本的 --fix 选项
./scripts/pre-commit-ci.sh --fix
make ci-fix
```

### Q4: Clippy 警告太多，如何临时忽略？

**不推荐：** 在代码中添加 `#[allow(clippy::...)]`

**推荐做法：** 逐个修复警告

```bash
# 查看所有警告
cargo clippy --all-targets --all-features --workspace

# 尝试自动修复
cargo clippy --all-targets --all-features --workspace --fix --allow-dirty

# 手动修复后重新检查
make clippy
```

### Q5: 测试失败，但我确定代码是对的？

```bash
# 1. 查看详细测试输出
cargo test --all-features --workspace -- --nocapture

# 2. 运行特定测试
cargo test test_function_name -- --nocapture

# 3. 运行特定包的测试
cargo test -p service -- --nocapture

# 4. 检查测试环境
# - 数据库文件是否正确
# - 环境变量是否设置
# - 测试数据是否冲突
```

### Q6: CI 检查太慢，如何加速？

```bash
# 1. 使用快速模式（跳过 build）
./scripts/pre-commit-ci.sh --fast
make ci-fast

# 2. 跳过测试（开发时）
./scripts/pre-commit-ci.sh --fast --skip-tests

# 3. 仅检查修改的代码
make fmt-check && make clippy

# 4. 使用增量编译（默认开启）
# 确保 target/ 目录不被频繁删除

# 5. 使用 cargo-watch 自动检查
cargo install cargo-watch
cargo watch -x check -x test
make watch
```

### Q7: 如何临时禁用 Git hooks？

```bash
# 提交时跳过 pre-commit
git commit --no-verify
git commit -n

# 推送时跳过 pre-push
git push --no-verify

# 永久禁用（不推荐）
rm .git/hooks/pre-commit
rm .git/hooks/pre-push
```

### Q8: CI 通过了，但 GitHub CI 失败？

这通常不应该发生，因为本地 CI 严格遵照 GitHub CI。可能原因：

1. **环境差异：** 本地和 CI 的 Rust 版本不同
   ```bash
   # 检查版本
   rustc --version
   cargo --version
   
   # 更新到最新稳定版
   rustup update stable
   ```

2. **缓存问题：** 本地缓存了旧的构建产物
   ```bash
   # 清理并重新检查
   make clean
   ./scripts/pre-commit-ci.sh
   ```

3. **平台差异：** 某些测试在不同操作系统上有不同行为
   - 检查测试是否有平台特定的逻辑
   - 使用 `#[cfg(target_os = "...")]` 条件编译

4. **并发问题：** 测试在并发运行时失败
   ```bash
   # 单线程运行测试
   cargo test -- --test-threads=1
   ```

---

## 最佳实践

### 开发工作流

```
┌─────────────────────────────────────────────────────────────┐
│                   Recommended Workflow                      │
└─────────────────────────────────────────────────────────────┘

1. 修改代码
   ↓
2. 快速检查（开发时）
   ./scripts/pre-commit-ci.sh --fast --skip-tests
   或 make quick
   ↓
3. 运行测试（功能完成后）
   make test
   ↓
4. 完整检查（提交前）
   ./scripts/pre-commit-ci.sh
   或 make ci
   ↓
5. 提交代码
   git commit -m "..."
   （自动运行 pre-commit hook）
   ↓
6. 推送代码
   git push
   （自动运行 pre-push hook）
```

### 提交前检查清单

在提交代码前，确保：

- [ ] 代码已格式化（`make fmt`）
- [ ] 没有 clippy 警告（`make clippy`）
- [ ] 所有测试通过（`make test`）
- [ ] 完整 CI 检查通过（`make ci`）
- [ ] 提交信息清晰明确

### 推荐的 CI 使用方式

**开发过程中（频繁运行）：**

```bash
# 快速模式 - 检查格式、clippy 和编译
./scripts/pre-commit-ci.sh --fast --skip-tests
make quick
```

**功能完成后：**

```bash
# 运行测试
make test

# 快速 CI（包括测试）
./scripts/pre-commit-ci.sh --fast
make ci-fast
```

**提交前（必须）：**

```bash
# 完整 CI 检查
./scripts/pre-commit-ci.sh
make ci
make pre-commit
```

**推送前（自动）：**

```bash
# pre-push hook 会自动运行完整检查
git push
```

### 团队协作建议

1. **统一使用 Git Hooks：** 所有团队成员都应该安装 hooks

   ```bash
   make install-hooks
   ```

2. **CI 失败不推送：** 确保本地 CI 通过后再推送

3. **定期更新工具链：**

   ```bash
   rustup update stable
   make deps-update
   ```

4. **使用 Makefile 命令：** 统一命令接口，降低学习成本

   ```bash
   make ci        # 而不是记忆复杂的 cargo 命令
   make test      # 简单易记
   ```

5. **遇到问题先查看文档：**

   ```bash
   ./scripts/pre-commit-ci.sh --help
   make help
   ```

### 性能优化建议

1. **保留 `target/` 目录：** 避免频繁清理，利用增量编译

2. **使用 `sccache` 加速编译：**

   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

3. **并行运行测试：**

   ```bash
   cargo test -- --test-threads=4
   ```

4. **开发时使用 `cargo check`：**

   ```bash
   cargo check  # 比 cargo build 快
   ```

---

## 环境变量

CI 脚本使用的环境变量（与 GitHub CI 一致）：

```bash
export CARGO_TERM_COLOR=always  # 彩色输出
export RUST_BACKTRACE=1         # 错误时显示回溯
```

可以在 `~/.bashrc` 或 `~/.zshrc` 中设置。

---

## 相关文件

```
peng-blog/
├── .github/workflows/ci.yml         # GitHub CI 配置（标准来源）
├── scripts/
│   ├── pre-commit-ci.sh            # 本地 CI 检查脚本
│   └── install-hooks.sh            # Git hooks 安装脚本
├── Makefile                         # 快捷命令
├── docs/
│   └── CI_CHECKS.md                # 本文档
└── .git/hooks/
    ├── pre-commit                   # 提交前 hook
    └── pre-push                     # 推送前 hook
```

---

## 附录：CI 输出示例

### 成功的 CI 检查

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Peng Blog - Local CI Check
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Running pre-commit CI checks (matching GitHub CI standards)

▶ Checking dependencies...
✓ All dependencies available

▶ 1/6 Format Check (cargo fmt --check)
✓ Format: All files are properly formatted

▶ 2/6 Clippy (cargo clippy -D warnings)
✓ Clippy: No warnings or errors

▶ 3/6 Check (cargo check)
✓ Check: Code compiles successfully

▶ 4/6 Test Suite (cargo test)
✓ Tests: All tests passed

▶ 5/6 Doc Tests (cargo test --doc)
✓ Doc Tests: All documentation tests passed

▶ 6/6 Build (cargo build --release)
ℹ Building release binary (this may take a while)...
✓ Build: Release build successful
ℹ Binary size: 12M (target/release/peng-blog)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  CI Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Check Results:

  Check                Status
  ──────────────────  ────────
  1. Format           ✓ PASS
  2. Clippy           ✓ PASS
  3. Check            ✓ PASS
  4. Tests            ✓ PASS
  5. Doc Tests        ✓ PASS
  6. Build            ✓ PASS

Statistics:
  Total Checks:  6
  Passed:        6
  Failed:        0
  Duration:      2m 34s

✓ All CI checks passed!
  Your code is ready to commit.
```

### 失败的 CI 检查

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Peng Blog - Local CI Check
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

▶ 2/6 Clippy (cargo clippy -D warnings)
✗ Clippy: Found warnings or errors

Showing last 30 lines of output:
warning: unused variable `user_id`
  --> crates/service/src/post.rs:42:9
   |
42 |     let user_id = uuid::Uuid::new_v4();
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_user_id`
   |
   = note: `#[warn(unused_variables)]` on by default

error: could not compile `service` due to previous error

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  CI Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Statistics:
  Total Checks:  2
  Passed:        1
  Failed:        1
  Duration:      0m 15s

✗ CI checks failed
  Please fix the errors above before committing.

Tips:
  • Run with --verbose to see full output
  • Run with --fix to auto-fix format issues
  • Check the error messages above for details
```

---

## 总结

本地 CI 检查工具提供了与 GitHub CI 完全一致的代码质量保障：

1. **自动化：** 通过 Git hooks 自动运行检查
2. **快速：** 支持快速模式，适合开发过程中频繁运行
3. **灵活：** 提供多种选项，适应不同场景
4. **友好：** 彩色输出和详细报告，易于理解和修复

**建议所有开发者：**
- 安装 Git hooks（`make install-hooks`）
- 提交前运行完整检查（`make ci`）
- 开发时使用快速模式（`make ci-fast`）

**保持代码质量，从本地 CI 开始！** 🚀

---

*Last updated: 2026-01-31*