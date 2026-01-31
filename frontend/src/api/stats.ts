import { http } from '../utils/request';
import type {
  GlobalStats,
  PostViews,
  AdminStats,
  RecordVisitRequest,
  ApiResponse
} from '../types';

/**
 * 统计 API 模块
 * 处理访问统计、文章阅读量统计等操作
 */
export const statsApi = {
  /**
   * 获取全局访问统计
   * @returns 全局访问统计信息
   */
  getGlobalStats: () => {
    return http.get<GlobalStats>('/stats/visits');
  },

  /**
   * 记录访问
   * 可以记录对特定页面的访问
   * @param data 访问记录请求，可包含 post_id
   * @returns 操作成功消息
   */
  recordVisit: (data?: RecordVisitRequest) => {
    return http.post<ApiResponse<{ message: string }>>('/stats/visits', data || {});
  },

  /**
   * 获取文章阅读量
   * @param postId 文章 ID
   * @returns 文章阅读量信息
   */
  getPostViews: (postId: string) => {
    return http.get<PostViews>(`/stats/posts/${postId}/views`);
  },

  /**
   * 记录文章阅读
   * 增加文章的阅读量
   * @param postId 文章 ID
   * @returns 操作成功消息
   */
  recordPostView: (postId: string) => {
    return http.post<ApiResponse<{ message: string }>>(`/stats/posts/${postId}/views`);
  },

  /**
   * 获取总统计（管理员）
   * 获取系统的全面统计信息，包括文章、用户、评论、文件、访问量等
   * @returns 管理员统计信息
   */
  getAdminStats: () => {
    return http.get<AdminStats>('/stats/total');
  },
};

// 默认导出
export default statsApi;