# Peng Blog Frontend

这是 Peng Blog 博客平台的前端应用，使用 React + Vite 构建，提供了现代化的博客浏览和管理功能。

## 🚀 技术栈

- **React 18** - UI 框架
- **TypeScript** - 类型安全的 JavaScript 超集
- **Vite** - 快速的前端构建工具
- **React Router** - 路由管理
- **Axios** - HTTP 请求库
- **CSS3** - 样式设计

## 📁 项目结构

```
frontend/
├── public/                 # 静态资源目录
│   └── vite.svg           # Vite 图标
├── src/                   # 源代码目录
│   ├── api/               # API 接口层
│   │   ├── index.ts       # API 统一导出
│   │   ├── auth.ts        # 认证相关 API
│   │   ├── posts.ts       # 文章相关 API
│   │   ├── users.ts       # 用户相关 API
│   │   ├── sessions.ts    # 会话相关 API
│   │   ├── files.ts       # 文件相关 API
│   │   ├── comments.ts    # 评论相关 API
│   │   └── stats.ts       # 统计相关 API
│   ├── components/        # React 组件
│   │   ├── LoginForm.tsx  # 登录表单组件
│   │   └── PostList.tsx   # 文章列表组件
│   ├── pages/             # 页面组件
│   │   └── Home.tsx       # 主页面
│   ├── types/             # TypeScript 类型定义
│   │   └── index.ts       # 全局类型定义
│   ├── utils/             # 工具函数
│   │   └── request.ts     # Axios 请求配置
│   ├── App.tsx            # 根组件
│   ├── main.tsx           # 应用入口
│   ├── App.css            # App 组件样式
│   └── index.css          # 全局样式
├── index.html             # HTML 入口文件
├── package.json           # 项目配置
├── tsconfig.json          # TypeScript 配置
├── tsconfig.node.json     # Node TypeScript 配置
├── vite.config.ts         # Vite 配置
└── README.md              # 项目说明文档
```

## 🎯 功能特性

### 已实现的功能

1. **用户认证**
   - 用户注册
   - 用户登录（JWT Token 认证）
   - 用户登出
   - 自动 token 管理

2. **文章管理**
   - 获取文章列表（支持分页）
   - 获取单篇文章详情
   - 创建新文章
   - 更新文章
   - 删除文章
   - 文章阅读量统计

3. **评论系统**
   - 获取文章评论列表
   - 创建评论（注册用户）
   - 创建评论（GitHub 用户）
   - 更新评论
   - 删除评论
   - GitHub OAuth 集成

4. **用户管理**
   - 获取当前用户信息
   - 获取用户列表
   - 获取指定用户信息
   - 删除用户

5. **文件管理**
   - 文件上传
   - 获取文件信息
   - 文件下载
   - 获取用户文件列表
   - 删除文件

6. **统计分析**
   - 全局访问统计
   - 记录访问
   - 文章阅读量统计
   - 管理员统计信息

### 计划中的功能

- 富文本编辑器
- 图片上传和预览
- 标签系统
- 搜索功能
- 用户个人主页
- 文章分类管理
- 评论回复功能
- 文章点赞功能

## 📦 安装与运行

### 前置要求

- Node.js >= 18.0.0
- npm >= 9.0.0 或 yarn >= 1.22.0

### 安装依赖

```bash
cd frontend
npm install
```

或使用 yarn：

```bash
cd frontend
yarn install
```

### 开发模式

启动开发服务器（默认运行在 http://localhost:5173）：

```bash
npm run dev
```

或使用 yarn：

```bash
yarn dev
```

### 构建生产版本

```bash
npm run build
```

或使用 yarn：

```bash
yarn build
```

构建后的文件将输出到 `dist` 目录。

### 预览生产版本

```bash
npm run preview
```

或使用 yarn：

```bash
yarn preview
```

## 🔧 配置说明

### 后端 API 连接

前端通过 Vite 的代理功能连接到后端 API。配置位于 `vite.config.ts`：

```typescript
server: {
  port: 5173,
  proxy: {
    '/api': {
      target: 'http://localhost:3000',
      changeOrigin: true,
    }
  }
}
```

确保后端服务运行在 `http://localhost:3000`。

### 环境变量

如需配置不同的 API 地址，可以创建 `.env` 文件：

```env
VITE_API_BASE_URL=http://localhost:3000/api
```

## 📚 API 使用示例

### 认证 API

```typescript
import { authApi } from './api';

// 用户登录
const login = async () => {
  try {
    const response = await authApi.login({
      username: 'testuser',
      password: 'password123'
    });
    
    // 保存登录信息
    authApi.saveAuth(response);
    console.log('登录成功:', response.user);
  } catch (error) {
    console.error('登录失败:', error);
  }
};

// 检查登录状态
const isAuth = authApi.isAuthenticated();
const currentUser = authApi.getCurrentUser();

// 用户登出
const logout = async () => {
  await authApi.logout();
  authApi.clearAuth();
};
```

### 文章 API

```typescript
import { postsApi } from './api';

// 获取文章列表
const fetchPosts = async () => {
  const response = await postsApi.getPosts({
    page: 1,
    page_size: 10,
    user_id: 'optional-user-id'
  });
  
  console.log('文章列表:', response.data);
  console.log('总数:', response.total);
};

// 创建文章
const createPost = async () => {
  const response = await postsApi.createPost({
    title: '新文章标题',
    content: '文章内容...',
    published: true
  });
  
  console.log('创建成功:', response);
};

// 更新文章
const updatePost = async (postId: string) => {
  const response = await postsApi.updatePost(postId, {
    title: '更新后的标题',
    published: false
  });
  
  console.log('更新成功:', response);
};

// 删除文章
const deletePost = async (postId: string) => {
  const response = await postsApi.deletePost(postId);
  console.log('删除成功:', response.message);
};
```

### 评论 API

```typescript
import { commentsApi } from './api';

// 获取文章评论
const fetchComments = async (postId: string) => {
  const response = await commentsApi.getCommentsByPost(postId, {
    page: 1,
    page_size: 20
  });
  
  console.log('评论列表:', response.data);
};

// 创建评论
const createComment = async (postId: string) => {
  const response = await commentsApi.createComment({
    post_id: postId,
    content: '这是一条评论'
  });
  
  console.log('评论创建成功:', response);
};

// GitHub 用户创建评论
const createGitHubComment = async (postId: string, accessToken: string) => {
  const response = await commentsApi.createGitHubComment({
    post_id: postId,
    content: 'GitHub 用户评论',
    access_token: accessToken
  });
  
  console.log('评论创建成功:', response);
};
```

### 统计 API

```typescript
import { statsApi } from './api';

// 获取全局统计
const getStats = async () => {
  const stats = await statsApi.getGlobalStats();
  console.log('总访问量:', stats.total_visits);
  console.log('今日访问:', stats.today_visits);
};

// 记录文章阅读
const recordView = async (postId: string) => {
  await statsApi.recordPostView(postId);
  console.log('阅读记录成功');
};

// 获取管理员统计
const getAdminStats = async () => {
  const stats = await statsApi.getAdminStats();
  console.log('文章总数:', stats.total_posts);
  console.log('用户总数:', stats.total_users);
  console.log('评论总数:', stats.total_comments);
};
```

## 🎨 组件使用示例

### 登录表单

```typescript
import React from 'react';
import LoginForm from './components/LoginForm';

const LoginPage: React.FC = () => {
  const handleLoginSuccess = () => {
    // 登录成功后的处理
    window.location.href = '/';
  };

  const handleLoginError = (error: any) => {
    console.error('登录失败:', error);
  };

  return (
    <div className="login-page">
      <LoginForm 
        onLoginSuccess={handleLoginSuccess}
        onLoginError={handleLoginError}
      />
    </div>
  );
};

export default LoginPage;
```

### 文章列表

```typescript
import React from 'react';
import PostList from './components/PostList';

const HomePage: React.FC = () => {
  const handlePostClick = (post: any) => {
    // 处理文章点击事件
    console.log('点击文章:', post.title);
    // 可以跳转到文章详情页
  };

  return (
    <div>
      <PostList onPostClick={handlePostClick} />
    </div>
  );
};

export default HomePage;
```

## 🔐 认证机制

### JWT Token 认证

前端使用 JWT Token 进行身份认证：

1. 用户登录时，后端返回 JWT Token 和用户信息
2. Token 被存储在 localStorage 中
3. 每个请求的 Authorization 头都会自动包含 Token
4. Token 过期时，自动跳转到登录页

### 请求拦截器

```typescript
// 自动添加 Token 到请求头
instance.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  }
);
```

### 响应拦截器

```typescript
// 统一处理错误响应
instance.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // 清除认证信息并跳转到登录页
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);
```

## 📝 类型定义

所有 API 相关的类型定义都位于 `src/types/index.ts`：

- `User` - 用户类型
- `Post` - 文章类型
- `Comment` - 评论类型
- `File` - 文件类型
- `Session` - 会话类型
- `GlobalStats` - 全局统计类型
- `AdminStats` - 管理员统计类型
- `ApiResponse` - API 响应基础类型
- `PaginatedResponse` - 分页响应类型
- `ApiError` - 错误响应类型

## 🌐 路由配置

应用使用 React Router 进行路由管理：

```typescript
<Routes>
  <Route path="/" element={<Home />} />
  <Route path="/login" element={<LoginForm />} />
  <Route path="/register" element={<RegisterForm />} />
  <Route path="/about" element={<AboutPage />} />
  <Route path="/admin" element={<ProtectedRoute><AdminPage /></ProtectedRoute>} />
  <Route path="*" element={<Navigate to="/" replace />} />
</Routes>
```

## 🎯 开发建议

1. **代码风格**
   - 使用 TypeScript 严格模式
   - 遵循 ESLint 规则
   - 组件使用函数式组件和 Hooks

2. **状态管理**
   - 对于简单状态，使用 React Hooks（useState, useEffect）
   - 对于复杂状态，可以考虑集成 Redux 或 Zustand

3. **性能优化**
   - 使用 React.memo 避免不必要的重渲染
   - 使用 useMemo 和 useCallback 优化计算和回调
   - 实现代码分割（React.lazy）

4. **错误处理**
   - 使用 Error Boundary 捕获组件错误
   - 统一处理 API 错误
   - 提供友好的错误提示

## 🐛 调试

### 开发工具

1. **React DevTools** - 调试 React 组件
2. **Redux DevTools** - 如果使用 Redux
3. **Network Tab** - 查看 API 请求
4. **Console** - 查看日志和错误

### 常见问题

1. **CORS 错误**
   - 确保 Vite 代理配置正确
   - 检查后端 CORS 设置

2. **认证失败**
   - 检查 Token 是否正确存储
   - 验证 Token 是否过期
   - 查看网络请求头

3. **构建错误**
   - 清除 node_modules 和重新安装
   - 检查 TypeScript 类型错误

## 📄 许可证

本项目采用 MIT 许可证。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📧 联系方式

如有问题，请通过以下方式联系：

- Email: peng@example.com
- GitHub Issues: [项目 Issues 页面]

---

**Happy Coding! 🚀**