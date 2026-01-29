import { http } from '../utils/request';
import type {
  Post,
  PostCreateRequest,
  PostUpdateRequest,
  PostListParams,
  PaginatedResponse,
  ApiResponse
} from '../types';

/**
 * 文章 API 模块
 * 处理文章的增删改查操作
 */
export const postsApi = {
  /**
   * 获取文章列表
   * 支持分页和按用户过滤
   * @param params 查询参数
   * @returns 文章列表
   */
  getPosts: (params?: PostListParams) => {
    return http.get<PaginatedResponse<Post>>('/posts', { params } as any);
  },

  /**
   * 获取单篇文章
   * @param id 文章 ID
   * @returns 文章详情
   */
  getPost: (id: string) => {
    return http.get<Post>(`/posts/${id}`);
  },

  /**
   * 创建文章
   * @param data 文章数据
   * @returns 创建成功的文章
   */
  createPost: (data: PostCreateRequest) => {
    return http.post<Post>('/posts', data);
  },

  /**
   * 更新文章
   * @param id 文章 ID
   * @param data 更新的文章数据
   * @returns 更新后的文章
   */
  updatePost: (id: string, data: PostUpdateRequest) => {
    return http.put<Post>(`/posts/${id}`, data);
  },

  /**
   * 删除文章
   * @param id 文章 ID
   * @returns 删除成功的消息
   */
  deletePost: (id: string) => {
    return http.delete<ApiResponse<{ message: string }>>(`/posts/${id}`);
  },
};

// 默认导出
export default postsApi;