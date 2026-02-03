import { http } from '../utils/request';
import type {
  User,
  UserUpdateRequest,
  ApiResponseV2,
  ApiListResponseV2,
  PaginationParams,
  Post,
} from '../types';

export const usersApi = {
  /**
   * 获取当前用户信息
   */
  getCurrentUser: () => {
    return http.get<ApiResponseV2<User>>('/auth/me');
  },

  /**
   * 获取用户列表
   * 需要管理员权限 (USER_MANAGE)
   */
  getUsers: (params?: PaginationParams) => {
    return http.get<ApiListResponseV2<User>>('/users', { params });
  },

  /**
   * 获取指定用户信息
   */
  getUser: (id: string) => {
    return http.get<ApiResponseV2<User>>(`/users/${id}`);
  },

  /**
   * 更新用户（部分更新）
   * API v2 新增方法
   */
  patchUser: (id: string, data: UserUpdateRequest) => {
    return http.patch<ApiResponseV2<User>>(`/users/${id}`, data);
  },

  /**
   * 删除用户
   */
  deleteUser: (id: string) => {
    return http.delete<void>(`/users/${id}`);
  },

  /**
   * 获取用户的文章列表
   * API v2 新增端点
   */
  getUserPosts: (id: string, params?: PaginationParams) => {
    return http.get<ApiListResponseV2<Post>>(`/users/${id}/posts`, { params });
  },
};

export default usersApi;
