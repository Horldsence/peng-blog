# Peng-Blog CLI 使用指南

## 概述

Peng-Blog 的 CLI 工具提供了一个统一的命令行接口，既可以用来运行博客服务器，也可以用于管理和维护博客系统。

## 启动博客服务器

直接运行 `cargo run` 或 `peng-blog` 命令而不带任何子命令，即可启动博客服务器：

```bash
# 开发模式启动
cargo run

# 生产模式启动
cargo run --release

# 直接运行编译好的二进制文件
./target/release/peng-blog
```

服务器默认监听 `0.0.0.0:3000`。

## 环境变量配置

博客服务器支持以下环境变量：

- `DATABASE_URL`: 数据库连接字符串（默认：`sqlite://blog.db`）
- `JWT_SECRET`: JWT 密钥（默认：`change-this-secret-in-production`）
- `UPLOAD_DIR`: 文件上传目录（默认：`./uploads`）
- `BASE_URL`: 应用基础 URL（默认：`http://localhost:3000`）
- `GITHUB_CLIENT_ID`: GitHub OAuth 客户端 ID（可选）
- `GITHUB_CLIENT_SECRET`: GitHub OAuth 客户端密钥（可选）

示例：

```bash
DATABASE_URL="sqlite://blog.db" JWT_SECRET="your-secret-key" ./target/release/peng-blog
```

## CLI 命令

### 查看帮助信息

```bash
peng-blog --help
peng-blog -h
```

### 用户管理命令

#### 列出所有用户

```bash
peng-blog user list
```

#### 显示用户详情

```bash
peng-blog user show <user-id>
```

示例：
```bash
peng-blog user show 06c43b68-8cd9-4773-bf97-a859cb70b4f6
```

#### 创建新用户

**交互式模式：**
```bash
peng-blog user create
```

**带参数创建：**
```bash
peng-blog user create --username myuser --password mypass
```

**创建管理员用户：**
```bash
peng-blog user create --username admin --password adminpass --admin
```

**非交互式模式：**
```bash
peng-blog user create --username myuser --password mypass --non-interactive
```

#### 删除用户

```bash
peng-blog user delete <user-id>
```

跳过确认提示：
```bash
peng-blog user delete <user-id> --force
peng-blog user delete <user-id> -f
```

#### 重置用户密码

**交互式模式：**
```bash
peng-blog user reset-password <user-id>
```

**带参数重置：**
```bash
peng-blog user reset-password <user-id> --password newpassword
```

**非交互式模式：**
```bash
peng-blog user reset-password <user-id> --password newpassword --non-interactive
```

#### 提升用户为管理员

```bash
peng-blog user promote <user-id>
```

跳过确认提示：
```bash
peng-blog user promote <user-id> --force
```

#### 降级管理员

```bash
peng-blog user demote <user-id>
```

跳过确认提示：
```bash
peng-blog user demote <user-id> --force
```

### 数据库管理命令

#### 运行数据库迁移

```bash
peng-blog db migrate
```

这会执行所有待执行的数据库迁移，确保数据库结构是最新的。

#### 重置数据库

⚠️ **警告**：此操作会删除所有数据！

```bash
peng-blog db reset
```

跳过确认提示（危险）：
```bash
peng-blog db reset --force
```

#### 显示数据库状态

```bash
peng-blog db status
```

这会显示：
- 数据库连接状态
- 总用户数
- 管理员数量

## 权限说明

用户权限使用位掩码表示：

| 权限位 | 权限名称       | 描述                   |
|--------|----------------|------------------------|
| 0      | POST_CREATE    | 创建文章               |
| 1      | POST_UPDATE    | 更新文章               |
| 2      | POST_DELETE    | 删除文章               |
| 3      | POST_PUBLISH   | 发布文章               |
| 4      | USER_MANAGE    | 用户管理               |

普通用户权限：`POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH` (值为 15)  
管理员权限：包含所有权限 (值为 31)

## 使用示例

### 初始化博客系统

1. 首先运行数据库迁移：
   ```bash
   peng-blog db migrate
   ```

2. 创建管理员用户：
   ```bash
   peng-blog user create --username admin --password securepassword --admin
   ```

3. 启动博客服务器：
   ```bash
   peng-blog
   ```

### 日常管理

- 查看所有用户：
  ```bash
  peng-blog user list
  ```

- 添加新用户：
  ```bash
  peng-blog user create --username john --password johnpass
  ```

- 升级用户为管理员：
  ```bash
  peng-blog user promote <user-id>
  ```

- 重置忘记的密码：
  ```bash
  peng-blog user reset-password <user-id> --password newpass
  ```

### 故障排除

检查数据库状态：
```bash
peng-blog db status
```

如果数据库出现问题，可以重置（注意备份数据！）：
```bash
peng-blog db reset --force
```

## 注意事项

1. **安全性**：在生产环境中，务必修改默认的 `JWT_SECRET`。
2. **数据备份**：在运行 `db reset` 之前，请务必备份数据库文件。
3. **密码安全**：使用强密码保护管理员账户。
4. **非交互式模式**：在脚本中使用非交互式模式时，请确保所有必要参数都已提供。

## 版本信息

查看 CLI 版本：
```bash
peng-blog --version
peng-blog -V
```
