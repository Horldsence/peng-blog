# RSS Feed功能说明

## 概述

博客系统已集成RSS feed功能，每次文章创建、更新、发布或删除时，RSS feed会自动刷新。

## 功能特性

- **自动刷新**: 文章内容变更时自动更新RSS feed
- **缓存机制**: 提升性能，避免每次请求都重新生成
- **标准RSS 2.0格式**: 兼容所有主流RSS阅读器
- **最新文章**: 默认包含最新20篇已发布文章

## API端点

### 获取RSS Feed

```
GET /api/rss
```

**响应格式**: `application/rss+xml; charset=utf-8`

**示例**:

```bash
curl http://localhost:3000/api/rss
```

## 文章内容包含

RSS feed包含以下信息：

- 文章标题
- 文章链接（格式: `{BASE_URL}/posts/{id}`）
- 文章内容
- 发布日期（如果文章已发布）或创建日期

## 自动触发场景

以下操作会自动刷新RSS feed缓存：

1. **创建文章**: `POST /api/posts`
2. **更新文章**: `PUT /api/posts/{id}`
3. **部分更新**: `PATCH /api/posts/{id}` (标题、内容或状态变更)
4. **删除文章**: `DELETE /api/posts/{id}`

## 配置

RSS服务在 `crates/app/src/lib.rs` 中初始化：

```rust
let rss_service = Arc::new(RssServiceImpl::new(
    post_repo.clone(),
    base_url.clone(),
    Some("Peng Blog".to_string()),
    Some("Latest posts from Peng Blog".to_string()),
)) as Arc<dyn RssService>;
```

可以通过环境变量 `BASE_URL` 配置博客基础URL。

## 使用示例

### 通过RSS阅读器订阅

将以下URL添加到RSS阅读器：

```
http://localhost:3000/api/rss
```

### 通过浏览器访问

直接在浏览器中访问 `/api/rss` 端点查看RSS feed内容。

## 技术实现

### 架构层次

```
Service层 (RssService trait)
    ↓
Service层 (RssServiceImpl 实现)
    ↓
API层 (get_rss_feed handler)
    ↓
HTTP响应 (application/rss+xml)
```

### 关键组件

- **RssService trait**: 定义RSS生成接口
- **RssServiceImpl**: 实现RSS生成逻辑，包含缓存机制
- **API Handler**: 处理HTTP请求，返回RSS feed
- **Post Handler集成**: 文章操作后自动刷新RSS缓存

## 依赖

- `rss` crate v2.0: RSS 2.0格式生成
- `tokio`: 异步运行时支持
- `domain::PostRepository`: 获取已发布文章

## 注意事项

1. **仅包含已发布文章**: 草稿文章不会出现在RSS feed中
2. **性能优化**: 使用RwLock缓存RSS feed，避免频繁数据库查询
3. **自动刷新**: 文章变更后自动刷新，无需手动触发
