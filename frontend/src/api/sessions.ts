import { http } from '../utils/request';
import type { Session, SessionCreateRequest, ApiResponseV2 } from '../types';

export const sessionsApi = {
  /**
   * 创建会话（登录）
   */
  createSession: (data: SessionCreateRequest) => {
    return http.post<ApiResponseV2<Session>>('/sessions', data);
  },

  /**
   * 删除当前会话（登出）
   */
  deleteSession: () => {
    return http.delete<void>('/sessions');
  },

  /**
   * 获取当前会话信息
   * API v2: 端点从 /sessions/current 改为 /sessions/info
   */
  getCurrentSession: () => {
    return http.get<ApiResponseV2<Session>>('/sessions/info');
  },

  /**
   * GitHub OAuth 回调
   * API v2: 从 GET /sessions/github/callback 改为 POST /sessions/github
   */
  githubCallback: (code: string) => {
    return http.post<ApiResponseV2<Session>>('/sessions/github', { code });
  },
};

export default sessionsApi;
