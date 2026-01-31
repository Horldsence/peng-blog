import { http } from '../utils/request';
import type {
  GlobalStats,
  PostViews,
  AdminStats,
  RecordVisitRequest,
  ApiResponseV2
} from '../types';

export const statsApi = {
  /**
   * 获取全局访问统计
   */
  getGlobalStats: () => {
    return http.get<ApiResponseV2<GlobalStats>>('/stats/visits');
  },

  /**
   * 记录访问
   */
  recordVisit: (data?: RecordVisitRequest) => {
    return http.post<ApiResponseV2<{ message: string }>>('/stats/visits', data || {});
  },

  /**
   * 获取文章阅读量
   */
  getPostViews: (postId: string) => {
    return http.get<ApiResponseV2<PostViews>>(`/stats/posts/${postId}/views`);
  },

  /**
   * 记录文章阅读
   */
  recordPostView: (postId: string) => {
    return http.post<ApiResponseV2<{ message: string }>>(`/stats/posts/${postId}/views`);
  },

  /**
   * 获取总统计（管理员）
   */
  getAdminStats: () => {
    return http.get<ApiResponseV2<AdminStats>>('/stats/total');
  },
};

export default statsApi;
