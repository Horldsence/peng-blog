import { http } from '../utils/request';
import type {
  Session,
  SessionCreateRequest,
  ApiResponse
} from '../types';

/**
 * 会话 API 模块
 * 处理会话的创建、获取和删除操作
 */
export const sessionsApi = {
  /**
   * 创建会话
   * 使用用户名和密码创建会话
   * @param data 会话创建请求
   * @returns 创建的会话信息
   */
  createSession: (data: SessionCreateRequest) => {
    return http.post<Session>('/sessions', data);
  },

  /**
   * 删除会话
   * @param id 会话 ID
   * @returns 删除成功的消息
   */
  deleteSession: (id: string) => {
    return http.delete<ApiResponse<{ message: string }>>(`/sessions/${id}`);
  },

  /**
   * 获取当前会话
   * @returns 当前会话信息
   */
  getCurrentSession: () => {
    return http.get<Session>('/sessions/current');
  },

  /**
   * GitHub OAuth 回调
   * 使用授权码获取会话
   * @param code GitHub 授权码
   * @returns 创建的会话信息
   */
  githubCallback: (code: string) => {
    return http.get<Session>('/sessions/github/callback', { params: { code } } as any);
  },
};

// 默认导出
export default sessionsApi;