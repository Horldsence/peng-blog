# 前端静态构建和部署指南

本文档说明如何构建前端静态资源并将其集成到后端服务中。

## 概述

Peng Blog 采用前后端分离架构，但支持将前端构建为静态资源并由后端直接服务。这样只需要运行一个服务器进程即可同时提供前端页面和 API 服务。

### 架构说明

```
┌─────────────────────────────────────────┐
│         Axum Server (Port 3000)         │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  /api/*  → API Routes             │  │
│  │  - POST /api/auth/login            │  │
│  │  - GET  /api/posts                 │  │
│  │  - ...                             │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  /*  → Static Files (./dist)      │  │
│  │  - /index.html                     │  │
│  │  - /assets/*.js                    │  │
│  │  - /assets/*.css                   │  │
│  │  - ...                             │  │
│  │                                    │  │
│  │  SPA Fallback:                     │  │
│  │  - /posts/123 → index.html         │  │
│  │  - /login → index.html             │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## 快速开始

### 1. 构建前端和后端

```bash
# 使用 Makefile（推荐）
make build-all          # Debug 模式
make release-all        # Release 模式（生产环境）

# 或者使用脚本
./scripts/build.sh               # Debug 模式
./scripts/build.sh --release     # Release 模式

# 或者手动构建
cd frontend && npm run build     # 构建前端
cd .. && cargo build --release   # 构建后端
```

### 2. 运行服务器

```bash
# Debug 模式
cargo run

# Release 模式
./target/release/peng-blog
```

### 3. 访问应用

打开浏览器访问：
- **前端页面**: http://localhost:3000
- **API 文档**: http://localhost:3000/api
- **健康检查**: http://localhost:3000/api/health

## 构建选项

### 使用 Makefile

```bash
# 仅构建前端
make frontend-build

# 仅构建后端
make build              # Debug
make release            # Release

# 完整构建
make build-all          # 前端 + 后端 (Debug)
make release-all        # 前端 + 后端 (Release)

# 清理构建产物
make clean              # 清理后端
make frontend-clean     # 清理前端
```

### 使用构建脚本

```bash
# 完整构建
./scripts/build.sh

# 仅构建前端
./scripts/build.sh --frontend-only

# 仅构建后端
./scripts/build.sh --backend-only

# Release 模式
./scripts/build.sh --release

# 查看帮助
./scripts/build.sh --help
```

### 手动构建

```bash
# 1. 构建前端
cd frontend
npm install              # 首次构建需要
npm run build            # 输出到 ../dist

# 2. 构建后端
cd ..
cargo build --release
```

## 目录结构

```
peng-blog/
├── frontend/            # 前端源代码
│   ├── src/
│   ├── index.html
│   ├── package.json
│   └── vite.config.ts   # 配置构建输出到 ../dist
│
├── dist/                # 前端构建产物（自动生成）
│   ├── index.html
│   ├── assets/
│   │   ├── index-*.js
│   │   ├── vendor-*.js
│   │   ├── markdown-*.js
│   │   └── index-*.css
│   └── vite.svg
│
└── target/              # 后端构建产物
    ├── debug/
    │   └── peng-blog
    └── release/
        └── peng-blog
```

## 配置说明

### Vite 配置 (frontend/vite.config.ts)

```typescript
export default defineConfig({
  plugins: [react()],
  base: '/',                    // 基础路径
  build: {
    outDir: '../dist',          // 输出到项目根目录的 dist
    emptyOutDir: true,          // 构建前清空目录
    sourcemap: false,           // 生产环境不生成 sourcemap
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom', 'react-router-dom'],
          markdown: ['react-markdown', 'rehype-highlight', 'remark-gfm'],
        },
      },
    },
  },
  // ...
})
```

### 后端配置 (crates/app/src/lib.rs)

后端使用 `tower-http::services::ServeDir` 提供静态文件服务，并实现了 SPA fallback：

```rust
// 静态文件服务配置
let serve_dir = ServeDir::new("dist");

let app = axum::Router::new()
    .nest("/api", routes())           // API 路由
    .fallback_service(/* ... */)      // 静态文件 + SPA fallback
    .with_state(state);
```

**SPA Fallback 逻辑:**
1. 如果请求的是 `/api/*`，则走 API 路由
2. 如果请求的文件存在（如 `/assets/index.js`），则返回文件内容
3. 如果文件不存在且路径没有扩展名（如 `/posts/123`），则返回 `index.html`，让前端路由处理
4. 否则返回 404

## 开发流程

### 开发模式（前后端分离）

在开发阶段，建议前后端分别运行：

```bash
# Terminal 1: 后端服务器（3000 端口）
cargo run

# Terminal 2: 前端开发服务器（5173 端口）
cd frontend
npm run dev
```

访问 http://localhost:5173，Vite 会自动代理 `/api/*` 请求到后端。

### 生产模式（前端集成）

构建完成后，只需运行后端：

```bash
# 1. 构建
make release-all

# 2. 运行
./target/release/peng-blog
```

访问 http://localhost:3000，后端同时提供前端页面和 API。

## 环境变量

创建 `.env` 文件（或使用环境变量）：

```env
# 数据库
DATABASE_URL=sqlite://blog.db

# 服务器
HOST=0.0.0.0
PORT=3000

# JWT
JWT_SECRET=your-secret-key-change-in-production

# 文件上传
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:3000

# GitHub OAuth（可选）
GITHUB_CLIENT_ID=your-client-id
GITHUB_CLIENT_SECRET=your-client-secret

# 日志
RUST_LOG=info
```

## 部署指南

### 本地部署

```bash
# 1. 克隆仓库
git clone <repository-url>
cd peng-blog

# 2. 配置环境变量
cp .env.example .env
# 编辑 .env 文件

# 3. 构建
make release-all

# 4. 初始化数据库
./target/release/peng-blog db migrate

# 5. 创建管理员用户
./target/release/peng-blog user create --username admin --password yourpass --admin

# 6. 运行
./target/release/peng-blog
```

### Docker 部署（推荐）

创建 `Dockerfile`:

```dockerfile
# 构建阶段
FROM rust:1.70 AS builder

# 安装 Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs

WORKDIR /app

# 复制源代码
COPY . .

# 构建前端
WORKDIR /app/frontend
RUN npm install && npm run build

# 构建后端
WORKDIR /app
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 复制二进制文件和静态资源
COPY --from=builder /app/target/release/peng-blog /app/
COPY --from=builder /app/dist /app/dist

# 创建数据目录
RUN mkdir -p /app/uploads /app/cache

# 暴露端口
EXPOSE 3000

# 运行
CMD ["/app/peng-blog"]
```

构建和运行：

```bash
docker build -t peng-blog .
docker run -d -p 3000:3000 \
  -v $(pwd)/blog.db:/app/blog.db \
  -v $(pwd)/uploads:/app/uploads \
  -e JWT_SECRET=your-secret \
  peng-blog
```

### systemd 服务

创建 `/etc/systemd/system/peng-blog.service`:

```ini
[Unit]
Description=Peng Blog Server
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/peng-blog
ExecStart=/opt/peng-blog/peng-blog
Restart=on-failure
RestartSec=5s

Environment="DATABASE_URL=sqlite:///opt/peng-blog/blog.db"
Environment="JWT_SECRET=your-secret-key"
Environment="UPLOAD_DIR=/opt/peng-blog/uploads"
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

启用和运行：

```bash
sudo systemctl daemon-reload
sudo systemctl enable peng-blog
sudo systemctl start peng-blog
sudo systemctl status peng-blog
```

## 性能优化

### 前端优化

1. **代码分割**: 已配置 vendor 和 markdown 分离
2. **资源压缩**: Vite 自动压缩 JS/CSS
3. **懒加载**: 考虑使用 React.lazy() 延迟加载组件

### 后端优化

1. **Release 构建**: 使用 `--release` 启用优化
2. **静态资源缓存**: 添加 HTTP 缓存头
3. **Gzip 压缩**: 使用 tower-http 的 CompressionLayer

示例（添加到 app/lib.rs）:

```rust
use tower_http::compression::CompressionLayer;

let app = axum::Router::new()
    .nest("/api", routes())
    .fallback_service(serve_dir)
    .layer(CompressionLayer::new())  // 添加 Gzip 压缩
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive())
    .with_state(state);
```

## 故障排查

### 前端构建失败

```bash
# 清理并重新安装依赖
cd frontend
rm -rf node_modules package-lock.json
npm install
npm run build
```

### 后端找不到静态文件

确保：
1. `dist/` 目录存在且包含 `index.html`
2. 工作目录正确（在项目根目录运行）
3. 检查日志输出：`Frontend served from ./dist directory`

```bash
# 检查 dist 目录
ls -la dist/

# 从正确的目录运行
cd /path/to/peng-blog
./target/release/peng-blog
```

### 前端路由 404

如果前端路由（如 `/posts/123`）返回 404：
1. 检查 SPA fallback 逻辑是否正确
2. 确认 `dist/index.html` 存在
3. 查看浏览器控制台错误

### API 请求失败

开发模式下：
- 前端：http://localhost:5173
- 后端：http://localhost:3000
- Vite 会代理 `/api` 到后端

生产模式下：
- 前端和后端都在 http://localhost:3000
- 确保 API 路径以 `/api` 开头

## 常见问题

**Q: 为什么要将前端构建到 `../dist` 而不是 `frontend/dist`？**

A: 这样后端可以直接使用 `ServeDir::new("dist")` 而不需要关心前端项目结构。

**Q: 可以更改静态文件目录吗？**

A: 可以，修改两处：
1. `frontend/vite.config.ts` 的 `build.outDir`
2. `crates/app/src/lib.rs` 的 `ServeDir::new("your-dir")`

**Q: 如何添加 HTTPS 支持？**

A: 使用 Nginx 或 Caddy 作为反向代理：

```nginx
server {
    listen 443 ssl http2;
    server_name example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

**Q: 开发时每次都要重新构建前端吗？**

A: 不需要。开发时使用 `npm run dev`（5173 端口），只在部署前构建一次。

## 相关文档

- [开发指南](./DEVELOPMENT.md)
- [API 文档](./api/INDEX.md)
- [部署指南](./DEPLOYMENT.md)
- [架构设计](./ARCHITECTURE.md)

---

*最后更新: 2024-01-31*