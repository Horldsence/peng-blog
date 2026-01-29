import { http } from '../utils/request';
import type {
  User,
  PaginatedResponse,
  ApiResponse,
  PaginationParams
} from '../types';

/**
 * 用户 API 模块
 * 处理用户信息的获取和管理操作
 */
export const usersApi = {
  /**
   * 获取当前登录用户信息
   * @returns 当前用户信息
   */
  getCurrentUser: () => {
    return http.get<User>('/users/me');
  },

  /**
   * 获取用户列表
   * 支持分页
   * @param params 分页参数
   * @returns 用户列表
   */
  getUsers: (params?: PaginationParams) => {
    return http.get<PaginatedResponse<User>>('/users', { params } as any);
  },

  /**
   * 获取指定用户信息
   * @param id 用户 ID
   * @returns 用户信息
   */
  getUser: (id: string) => {
    return http.get<User>(`/users/${id}`);
  },

  /**
   * 删除用户
   * @param id 用户 ID
   * @returns 删除成功的消息
   */
  deleteUser: (id: string) => {
    return http.delete<ApiResponse<{ message: string }>>(`/users/${id}`);
  },
};

// 默认导出
export default usersApi;