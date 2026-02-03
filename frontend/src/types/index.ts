// 通用 ID 类型
export type UUID = string;

// 基础时间戳类型
export type Timestamp = string;

// ===== API v2 统一响应格式 =====

/**
 * API v2 统一响应格式（单个资源）
 */
export interface ApiResponseV2<T = unknown> {
  code: number;
  message: string;
  data: T;
}

/**
 * API v2 统一响应格式（列表）
 */
export interface ApiListResponseV2<T = unknown> {
  code: number;
  message: string;
  data: T[];
  pagination: {
    page: number;
    per_page: number;
    total: number;
    total_pages: number;
  };
}

/**
 * API v2 错误响应格式
 */
export interface ApiErrorV2 {
  code: number;
  message: string;
  errors?: Record<string, string[]>;
}

// ===== 兼容旧版本的类型（待废弃）=====

// @deprecated 使用 ApiResponseV2 代替
export interface ApiResponse<T> {
  data?: T;
  message?: string;
}

// @deprecated 使用 ApiListResponseV2 代替
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  page_size: number;
}

// @deprecated 使用 ApiErrorV2 代替
export interface ApiError {
  error: string;
  message: string;
  details?: Array<{
    field: string;
    message?: string;
  }>;
}

// ===== 用户相关类型 =====

export interface User {
  id: UUID;
  username: string;
  permissions: number;
  created_at: Timestamp;
}

export interface UserCreateRequest {
  username: string;
  password: string;
}

export interface UserLoginRequest {
  username: string;
  password: string;
}

export interface UserLoginResponse {
  token: string;
  user: User;
}

export interface UserUpdateRequest {
  username?: string;
}

// ===== 文章相关类型 =====

export interface Post {
  id: UUID;
  user_id: UUID;
  title: string;
  content: string;
  category_id?: UUID | null;
  views: number;
  created_at: Timestamp;
  updated_at?: Timestamp;
  published_at?: Timestamp | null;
}

export interface PostCreateRequest {
  title: string;
  content: string;
}

export interface PostUpdateRequest {
  title?: string;
  content?: string;
}

/**
 * 文章部分更新请求（用于 PATCH）
 */
export interface PostPatchRequest {
  title?: string;
  content?: string;
  category_id?: UUID | null;
  status?: 'published' | 'draft';
}

/**
 * 文章列表查询参数（API v2）
 */
export interface PostListParams {
  page?: number;
  per_page?: number;
  author?: UUID; // 按 author 过滤（替代 user_id）
  category?: UUID; // 按 category 过滤（替代 category_id）
  tag?: UUID; // 按 tag 过滤
  status?: 'published' | 'draft' | 'all'; // 按 status 过滤
}

/**
 * 文章搜索查询参数
 */
export interface PostSearchParams {
  q: string; // 搜索关键词
  page?: number;
  per_page?: number;
}

// ===== 分类相关类型 =====

export interface Category {
  id: UUID;
  name: string;
  slug: string;
  description?: string;
  parent_id?: UUID | null;
  created_at: Timestamp;
}

export interface CategoryCreateRequest {
  name: string;
  slug: string;
  description?: string;
  parent_id?: UUID | null;
}

export interface CategoryUpdateRequest {
  name?: string;
  slug?: string;
  description?: string;
  parent_id?: UUID | null;
}

// ===== 标签相关类型 =====

export interface Tag {
  id: UUID;
  name: string;
  slug: string;
  created_at: Timestamp;
}

export interface TagCreateRequest {
  name: string;
  slug: string;
}

// ===== 评论相关类型 =====

export interface Comment {
  id: UUID;
  post_id: UUID;
  user_id?: UUID | null;
  github_username?: string | null;
  github_avatar_url?: string | null;
  content: string;
  created_at: Timestamp;
  updated_at: Timestamp;
}

export interface CommentCreateRequest {
  post_id: UUID;
  content: string;
  access_token?: string; // GitHub OAuth token
}

export interface CommentUpdateRequest {
  content: string;
}

/**
 * 评论部分更新请求
 */
export interface CommentPatchRequest {
  content?: string;
}

export interface CommentListParams {
  page?: number;
  per_page?: number;
}

export interface GitHubAuthResponse {
  auth_url: string;
}

// ===== 文件相关类型 =====

export interface FileInfo {
  id: UUID;
  user_id: UUID;
  filename: string;
  original_filename: string;
  content_type: string;
  size_bytes: number;
  url: string;
  created_at: Timestamp;
}

export interface FileUploadParams {
  file: globalThis.File;
}

// ===== 会话相关类型 =====

export interface Session {
  id: UUID;
  user_id: UUID;
  expires_at: Timestamp;
  created_at: Timestamp;
}

export interface SessionCreateRequest {
  username: string;
  password: string;
  remember_me?: boolean;
}

/**
 * GitHub OAuth 回调请求
 */
export interface GitHubCallbackRequest {
  code: string;
}

// ===== 统计相关类型 =====

export interface GlobalStats {
  total_visits: number;
  today_visits: number;
  last_updated: Timestamp;
}

export interface PostViews {
  post_id: UUID;
  views: number;
  last_viewed_at: Timestamp;
}

export interface AdminStats {
  total_posts: number;
  total_users: number;
  total_comments: number;
  total_files: number;
  total_visits: number;
  today_visits: number;
}

export interface RecordVisitRequest {
  post_id?: UUID;
}

// ===== 配置相关类型 =====

export interface DatabaseConfig {
  url: string;
  url_env_override?: boolean;
}

export interface ServerConfig {
  host: string;
  host_env_override?: boolean;
  port: number;
  port_env_override?: boolean;
}

export interface AuthConfig {
  jwt_secret: string;
  jwt_secret_env_override?: boolean;
}

export interface StorageConfig {
  upload_dir: string;
  upload_dir_env_override?: boolean;
  cache_dir: string;
  cache_dir_env_override?: boolean;
}

export interface GitHubConfig {
  client_id: string;
  client_id_env_override?: boolean;
  client_secret: string;
  client_secret_env_override?: boolean;
}

export interface SiteConfig {
  allow_registration: boolean;
  allow_registration_env_override?: boolean;
}

export interface Config {
  database: DatabaseConfig;
  server: ServerConfig;
  auth: AuthConfig;
  storage: StorageConfig;
  github: GitHubConfig;
  site: SiteConfig;
}

export interface UpdateConfigRequest {
  database?: Partial<DatabaseConfig>;
  server?: Partial<ServerConfig>;
  auth?: Partial<AuthConfig>;
  storage?: Partial<StorageConfig>;
  github?: Partial<GitHubConfig>;
  site?: Partial<SiteConfig>;
}

// ===== 权限位标志 =====

export enum Permission {
  POST_CREATE = 1 << 0, // 1
  POST_UPDATE = 1 << 1, // 2
  POST_DELETE = 1 << 2, // 4
  POST_PUBLISH = 1 << 3, // 8
  USER_MANAGE = 1 << 4, // 16
}

// 默认用户权限
export const DEFAULT_USER_PERMISSIONS =
  Permission.POST_CREATE | Permission.POST_UPDATE | Permission.POST_PUBLISH;

// 管理员权限
export const ADMIN_PERMISSIONS =
  Permission.POST_CREATE |
  Permission.POST_UPDATE |
  Permission.POST_DELETE |
  Permission.POST_PUBLISH |
  Permission.USER_MANAGE;

// ===== 查询参数通用接口 =====

export interface PaginationParams {
  page?: number;
  per_page?: number;
}
