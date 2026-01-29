// 通用 ID 类型
export type UUID = string;

// 基础时间戳类型
export type Timestamp = string;

// 通用响应类型
export interface ApiResponse<T> {
  data?: T;
  message?: string;
}

// 分页响应类型
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  page_size: number;
}

// 错误响应类型
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

// ===== 文章相关类型 =====
export interface Post {
  id: UUID;
  user_id: UUID;
  title: string;
  content: string;
  published: boolean;
  views: number;
  created_at: Timestamp;
  updated_at: Timestamp;
  published_at?: Timestamp;
}

export interface PostCreateRequest {
  title: string;
  content: string;
  published: boolean;
}

export interface PostUpdateRequest {
  title?: string;
  content?: string;
  published?: boolean;
}

export interface PostListParams {
  page?: number;
  page_size?: number;
  user_id?: UUID;
}

// ===== 评论相关类型 =====
export interface Comment {
  id: UUID;
  post_id: UUID;
  user_id: UUID;
  github_username?: string;
  github_avatar_url?: string;
  content: string;
  created_at: Timestamp;
  updated_at: Timestamp;
}

export interface CommentCreateRequest {
  post_id: UUID;
  content: string;
  access_token?: string;
}

export interface CommentUpdateRequest {
  content: string;
}

export interface CommentListParams {
  page?: number;
  page_size?: number;
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

// ===== 权限位标志 =====
export enum Permission {
  READ = 1 << 0,
  WRITE = 1 << 1,
  DELETE = 1 << 2,
  ADMIN = 1 << 3,
}

// ===== 查询参数通用接口 =====
export interface PaginationParams {
  page?: number;
  page_size?: number;
}