# AGENTS.md - Frontend API Layer

> 前端API客户端层 - HTTP请求封装和错误处理

**Generated:** 2026-02-04 10:00:33 PM **Parent:** `../../AGENTS.md`

---

## OVERVIEW

**Frontend API Layer** - React应用与后端HTTP API的交互层（12个模块文件）

---

## STRUCTURE

```
frontend/src/api/
├── client.ts        # Axios实例 + interceptors
├── index.ts         # Barrel导出
├── types.ts         # ApiResponseV2, ApiErrorV2
├── auth.ts          # login, register, logout
├── posts.ts         # CRUD, publish
├── comments.ts      # CRUD, GitHub OAuth
├── users.ts         # profile, update
├── files.ts         # upload, delete
└── stats.ts         # visits, views
```

---

## WHERE TO LOOK

| Task       | File        | Purpose                                 |
| ---------- | ----------- | --------------------------------------- |
| Axios配置  | `client.ts` | baseURL, token interceptor, 401 handler |
| 类型定义   | `types.ts`  | ApiResponseV2<T>, ApiListResponseV2<T>  |
| Barrel导出 | `index.ts`  | 统一导出所有API模块                     |

---

## CONVENTIONS

**API模块模式:**

```tsx
export const postsApi = {
  list: (params?) => api.get<ApiListResponseV2<Post>>('/posts', { params }),
  get: (id) => api.get<ApiResponseV2<Post>>(`/posts/${id}`),
  create: (data) => api.post<ApiResponseV2<Post>>('/posts', data),
};
```

**错误处理（FluentUI Toast）:**

```tsx
try {
  const result = await postsApi.list();
  setPosts(result.data);
} catch (error) {
  dispatchToast({ intent: 'error', title: 'Error', content: 'Failed to load' });
}
```

---

## DEPRECATED TYPES

**Location:** `../types/index.ts:44-58`

迁移到V2版本:

- `ApiResponse<T>` → `ApiResponseV2<T>`
- `PaginatedResponse<T>` → `ApiListResponseV2<T>`
- `ApiError` → `ApiErrorV2`

---

## UNIQUE STYLES

- **Barrel Export** - 从`../api`统一导入所有模块
- **Token Interceptor** - 自动注入`Authorization: Bearer ${token}`
- **401 Redirect** - 拦截器自动重定向到`/login`
- **Type Safety** - 所有API函数有完整泛型类型

---

_Last updated: 2026-02-04_
