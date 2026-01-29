# Peng Blog 快速开始指南

本指南将帮助你在 5 分钟内启动 Peng Blog 并开始使用。

## 📋 前置要求

在开始之前，请确保你的系统已安装以下软件：

### 必需软件

- **Rust** 1.70 或更高版本
  ```bash
  # 检查 Rust 版本
  rustc --version
  
  # 如果未安装，访问 https://rustup.rs/
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**（随 Rust 自动安装）
  ```bash
  # 检查 Cargo 版本
  cargo --version
  ```

- **SQLite 3**（用于数据库）
  ```bash
  # macOS
  brew install sqlite3
  
  # Linux (Ubuntu/Debian)
  sudo apt-get install sqlite3
  
  # Windows
  # 下载 https://www.sqlite.org/download.html
  ```

### 可选软件（用于开发）

- **Git** - 版本控制
  ```bash
  # 检查 Git 版本
  git --version
  ```

---

## 🚀 安装步骤

### 1. 克隆项目仓库

```bash
# 使用 Git 克隆（如果可用）
git clone https://github.com/yourusername/peng-blog.git
cd peng-blog

# 或者直接下载并解压
```

### 2. 安装项目依赖

```bash
# 编译所有依赖
cargo build

# 这将下载并编译所有 crate 的依赖
# 首次编译可能需要 2-5 分钟
```

### 3. 配置环境变量

创建 `.env` 文件在项目根目录：

```bash
# 使用模板创建
cp .env.example .env  # 如果有示例文件

# 或者直接创建
touch .env
```

编辑 `.env` 文件，添加以下内容：

```env
# 数据库配置
DATABASE_URL=sqlite://blog.db

# JWT 认证密钥（生产环境请使用强密钥）
JWT_SECRET=change-this-secret-in-production-min-32-chars

# 文件上传配置
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:3000

# GitHub OAuth（可选，用于 GitHub 评论功能）
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
```

**重要提示：**
- `JWT_SECRET` 至少需要 32 个字符
- 生产环境建议使用随机生成的密钥：
  ```bash
  # 生成安全的 JWT 密钥
  openssl rand -base64 32
  ```

### 4. 运行数据库迁移

数据库迁移会在首次启动时自动执行，但你可以手动运行：

```bash
# 运行迁移
cargo run

# 首次启动时会看到类似输出：
# [INFO] Running migrations...
# [INFO] Migration completed successfully
```

### 5. 启动应用

```bash
# 开发模式启动（带日志）
cargo run

# 或者直接运行编译后的二进制文件
./target/debug/app
```

启动成功后，你会看到：

```
[INFO] DATABASE_URL: sqlite://blog.db
[INFO] Listening on 0.0.0.0:3000
```

---

## ✅ 验证安装

### 1. 检查服务状态

```bash
# 健康检查
curl http://localhost:3000/api/stats/visits
```

预期响应：
```json
{
  "total_visits": 0,
  "today_visits": 0,
  "last_updated": "2026-01-29T10:00:00Z"
}
```

### 2. 创建测试用户

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "Admin123!"
  }'
```

预期响应：
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "admin",
  "permissions": 15,
  "created_at": "2026-01-29T10:00:00Z"
}
```

### 3. 登录并获取 Token

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "Admin123!"
  }'
```

预期响应：
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "admin",
    "permissions": 15
  }
}
```

**保存返回的 token，后续 API 调用需要使用。**

---

## 📝 基本使用示例

### 创建第一篇文章

```bash
# 设置环境变量（方便后续使用）
export TOKEN="你的-jwt-token"

# 创建文章
curl -X POST http://localhost:3000/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "我的第一篇博客",
    "content": "这是我的第一篇博客文章！\n\n欢迎来到 Peng Blog！",
    "published": true
  }'
```

### 查看文章列表

```bash
curl http://localhost:3000/api/posts
```

### 获取单篇文章

```bash
# 使用返回的文章 ID
curl http://localhost:3000/api/posts/550e8400-e29b-41d4-a716-446655440000
```

### 上传文件

```bash
# 上传图片
curl -X POST http://localhost:3000/api/files \
  -H "Authorization: Bearer $TOKEN" \
  -F "file=@/path/to/your/image.jpg"
```

### 添加评论

```bash
curl -X POST http://localhost:3000/api/comments \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "post_id": "550e8400-e29b-41d4-a716-446655440000",
    "content": "这是一条测试评论"
  }'
```

### 查看统计信息

```bash
# 查看访问统计
curl http://localhost:3000/api/stats/visits

# 记录文章阅读
curl -X POST http://localhost:3000/api/stats/posts/550e8400-e29b-41d4-a716-446655440000/views
```

---

## 🛠️ 常用开发命令

### 编译和运行

```bash
# 开发模式编译（带调试信息）
cargo build

# 发布模式编译（优化性能）
cargo build --release

# 直接运行
cargo run

# 运行发布版本
cargo run --release
```

### 测试

```bash
# 运行所有测试
cargo test

# 运行特定包的测试
cargo test -p domain
cargo test -p service
cargo test -p api

# 显示测试输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_create_post
```

### 代码质量检查

```bash
# 代码格式化
cargo fmt

# 代码检查（Clippy）
cargo clippy

# 修复可自动修复的警告
cargo clippy --fix

# 检查特定包
cargo clippy -p api
```

### 数据库操作

```bash
# 查看数据库内容
sqlite3 blog.db

# SQLite 常用命令
.tables          # 查看所有表
.schema post     # 查看 post 表结构
SELECT * FROM post LIMIT 10;  # 查询前 10 篇文章
.quit            # 退出
```

---

## 🐛 故障排除

### 问题 1: 端口已被占用

**错误信息：**
```
Error: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
```

**解决方案：**
```bash
# 查找占用端口 3000 的进程
lsof -i :3000

# macOS/Linux
kill -9 <PID>

# 或者修改 src/main.rs 中的端口
# 将 0.0.0.0:3000 改为 0.0.0.0:3001
```

### 问题 2: 数据库连接失败

**错误信息：**
```
Error: Database connection failed
```

**解决方案：**
```bash
# 检查 .env 文件中的 DATABASE_URL
# 确保格式正确：
# SQLite: sqlite://blog.db
# PostgreSQL: postgresql://user:pass@localhost/blog

# 检查文件权限
ls -la blog.db
chmod 644 blog.db
```

### 问题 3: JWT 认证失败

**错误信息：**
```
Error: Invalid authentication token
```

**解决方案：**
```bash
# 确保 JWT_SECRET 至少 32 个字符
# 在 .env 文件中更新：
JWT_SECRET=your-very-long-secret-key-at-least-32-characters

# 重新登录获取新 token
```

### 问题 4: 文件上传目录权限错误

**错误信息：**
```
Error: Permission denied (os error 13)
```

**解决方案：**
```bash
# 创建上传目录
mkdir -p uploads

# 设置正确的权限
chmod 755 uploads

# 确保 .env 中的 UPLOAD_DIR 指向正确的目录
UPLOAD_DIR=./uploads
```

### 问题 5: 编译错误或依赖问题

**解决方案：**
```bash
# 清理并重新编译
cargo clean
cargo build

# 更新依赖
cargo update

# 检查 Rust 版本
rustc --version  # 应该 >= 1.70

# 更新 Rust（如果需要）
rustup update
```

---

## 📚 下一步

现在你已经成功运行了 Peng Blog，接下来可以：

1. **阅读完整文档**
   - [API 文档](./api/README.md) - 详细的 API 参考手册
   - [架构文档](./ARCHITECTURE.md) - 了解系统设计
   - [新功能说明](./NEW_FEATURES.md) - 最新功能介绍

2. **开始开发**
   - 查看 [开发指南](../docs/README.md#开发指南)
   - 了解 [添加新功能](./ARCHITECTURE.md#adding-new-features)

3. **配置生产环境**
   - 使用强密钥和配置
   - 配置 HTTPS
   - 设置数据库备份
   - 启用日志监控

4. **部署应用**
   - [Docker 部署](../docs/README.md#docker-部署)
   - 云服务器部署
   - CI/CD 配置

---

## 💡 提示和最佳实践

### 性能优化

```bash
# 使用发布模式获得更好的性能
cargo build --release
./target/release/app
```

### 日志配置

```env
# 在 .env 中设置日志级别
RUST_LOG=debug          # 详细日志
RUST_LOG=info           # 标准日志
RUST_LOG=warn           # 仅警告和错误
RUST_LOG=error          # 仅错误
RUST_LOG=peng_blog=debug,tower_http=info,axum=trace  # 分模块配置
```

### 开发建议

1. **使用热重载**（可选工具：cargo-watch）
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

2. **使用数据库可视化工具**
   - [DB Browser for SQLite](https://sqlitebrowser.org/)
   - [DBeaver](https://dbeaver.io/)

3. **API 测试工具**
   - [Postman](https://www.postman.com/)
   - [Insomnia](https://insomnia.rest/)
   - 或使用 `curl` / `httpie`

---

## 🆘 获取帮助

遇到问题？这里有一些获取帮助的途径：

- **查看文档**：docs/ 目录下的所有文档
- **查看示例代码**：各个 crate 的示例和测试
- **搜索问题**：GitHub Issues
- **提交问题**：在 GitHub 创建新的 Issue

---

## 📊 项目结构快速参考

```
peng-blog/
├── crates/
│   ├── app/              # 应用入口
│   ├── api/              # API 路由和处理器
│   ├── domain/           # 领域类型
│   ├── service/          # 业务逻辑
│   └── infrastructure/   # 数据库实现
├── docs/                 # 文档
├── static/               # 静态文件
├── uploads/              # 上传文件
├── .env                  # 环境变量（需要创建）
├── Cargo.toml            # 项目配置
└── README.md             # 项目说明
```

---

## ✨ 恭喜！

🎉 你已经成功启动了 Peng Blog！

现在可以开始创建你的博客内容了。如果你有任何问题或建议，欢迎参与项目贡献。

**祝你使用愉快！**

---

*最后更新：2026-01-29*