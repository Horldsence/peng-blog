/**
 * API 统一导出文件
 *
 * 使用方式：
 * import { authApi, postsApi } from '@/api';
 *
 * 或者：
 * import api from '@/api';
 * api.auth.login({...});
 */

// 导入所有 API 模块
import { authApi } from './auth';
import { postsApi } from './posts';
import { usersApi } from './users';
import { sessionsApi } from './sessions';
import { filesApi } from './files';
import { commentsApi } from './comments';
import { statsApi } from './stats';
import { categoriesApi } from './categories';
import { tagsApi } from './tags';
import { bingApi } from './bing';
import { configApi } from './config';

// 重新导出所有 API 模块
export { authApi } from './auth';
export { postsApi } from './posts';
export { usersApi } from './users';
export { sessionsApi } from './sessions';
export { filesApi } from './files';
export { commentsApi } from './comments';
export { statsApi } from './stats';
export { categoriesApi } from './categories';
export { tagsApi } from './tags';
export { bingApi } from './bing';
export { configApi } from './config';

// 创建统一的 API 对象
export const api = {
  auth: authApi,
  posts: postsApi,
  users: usersApi,
  sessions: sessionsApi,
  files: filesApi,
  comments: commentsApi,
  stats: statsApi,
  categories: categoriesApi,
  tags: tagsApi,
  bing: bingApi,
  config: configApi,
};

// 默认导出统一 API 对象
export default api;
