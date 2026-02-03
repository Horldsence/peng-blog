import { http } from '../utils/request';
import type {
  UserCreateRequest,
  UserLoginRequest,
  UserLoginResponse,
  User,
  ApiResponse,
} from '../types';

/**
 * 认证 API 模块
 * 处理用户注册、登录、登出等认证相关操作
 */
export const authApi = {
  /**
   * 用户注册
   * 创建新用户账户
   * @param data 用户注册信息
   * @returns 注册成功的用户信息
   */
  register: (data: UserCreateRequest) => {
    return http.post<User>('/auth/register', data);
  },

  /**
   * 用户登录
   * 使用用户名和密码登录，返回 JWT token 和用户信息
   * @param data 用户登录信息
   * @returns 登录响应，包含 token 和用户信息
   */
  login: async (data: UserLoginRequest) => {
    const response = await http.post<{ code: number; message: string; data: UserLoginResponse }>(
      '/auth/login',
      data
    );
    return response.data;
  },

  /**
   * 用户登出
   * 使当前 token 失效
   * @returns 登出成功消息
   */
  logout: () => {
    return http.post<ApiResponse<{ message: string }>>('/auth/logout');
  },

  /**
   * 保存登录信息到 localStorage
   * @param response 登录响应
   */
  saveAuth: (response: UserLoginResponse) => {
    localStorage.setItem('token', response.token);
    localStorage.setItem('user', JSON.stringify(response.user));
  },

  /**
   * 清除登录信息
   */
  clearAuth: () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
  },

  /**
   * 获取当前登录用户信息
   * @returns 用户信息，如果未登录则返回 null
   */
  getCurrentUser: (): User | null => {
    const userStr = localStorage.getItem('user');
    if (userStr) {
      try {
        const user = JSON.parse(userStr) as User;
        // 确保 permissions 是数字类型
        if (user && typeof user.permissions === 'string') {
          user.permissions = parseInt(user.permissions, 10);
        }
        return user;
      } catch (error) {
        console.error('解析用户信息失败:', error);
        return null;
      }
    }
    return null;
  },

  /**
   * 获取当前 token
   * @returns token 字符串，如果未登录则返回 null
   */
  getToken: (): string | null => {
    return localStorage.getItem('token');
  },

  /**
   * 检查是否已登录
   * @returns 是否已登录
   */
  isAuthenticated: (): boolean => {
    const token = localStorage.getItem('token');
    const user = localStorage.getItem('user');
    return !!(token && user);
  },
};

// 默认导出
export default authApi;
