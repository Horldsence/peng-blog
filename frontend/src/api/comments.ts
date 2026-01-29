import { http } from '../utils/request';
import type {
  Comment,
  CommentCreateRequest,
  CommentUpdateRequest,
  CommentListParams,
  PaginatedResponse,
  ApiResponse,
  GitHubAuthResponse
} from '../types';

/**
 * 评论 API 模块
 * 处理评论的创建、获取、更新和删除操作
 */
export const commentsApi = {
  /**
   * 获取文章评论列表
   * 支持分页
   * @param postId 文章 ID
   * @param params 分页参数
   * @returns 评论列表
   */
  getCommentsByPost: (postId: string, params?: CommentListParams) => {
    return http.get<PaginatedResponse<Comment>>(`/comments`, {
      params: { post_id: postId, ...params }
    } as any);
  },

  /**
   * 创建评论（注册用户）
   * @param data 评论数据
   * @returns 创建成功的评论
   */
  createComment: (data: CommentCreateRequest) => {
    return http.post<Comment>('/comments', data);
  },

  /**
   * 创建评论（GitHub 用户）
   * 使用 GitHub access token 创建评论
   * @param data 评论数据，包含 access_token
   * @returns 创建成功的评论
   */
  createGitHubComment: (data: CommentCreateRequest) => {
    return http.post<Comment>('/comments/github', data);
  },

  /**
   * 获取 GitHub 授权 URL
   * 用于 GitHub 用户登录并创建评论
   * @returns GitHub 授权 URL
   */
  getGitHubAuthUrl: () => {
    return http.get<GitHubAuthResponse>('/comments/github/auth-url');
  },

  /**
   * 获取单条评论
   * @param id 评论 ID
   * @returns 评论详情
   */
  getComment: (id: string) => {
    return http.get<Comment>(`/comments/${id}`);
  },

  /**
   * 更新评论
   * @param id 评论 ID
   * @param data 更新的评论数据
   * @returns 更新后的评论
   */
  updateComment: (id: string, data: CommentUpdateRequest) => {
    return http.put<Comment>(`/comments/${id}`, data);
  },

  /**
   * 删除评论
   * @param id 评论 ID
   * @returns 删除成功的消息
   */
  deleteComment: (id: string) => {
    return http.delete<ApiResponse<{ message: string }>>(`/comments/${id}`);
  },
};

// 默认导出
export default commentsApi;