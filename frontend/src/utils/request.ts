import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig, AxiosResponse } from 'axios';
import type { ApiErrorV2 } from '../types';

/**
 * 创建 axios 实例
 */
const createAxiosInstance = (): AxiosInstance => {
  const instance = axios.create({
    baseURL: '/api',
    timeout: 30000,
    // 不设置默认Content-Type，让axios根据data类型自动设置
    // FormData会自动设置为multipart/form-data并生成boundary
    // JSON对象会自动设置为application/json
  });

  // 请求拦截器 - 添加 token
  instance.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
      const token = localStorage.getItem('token');
      if (token && config.headers) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    },
    (error: AxiosError) => {
      return Promise.reject(error);
    }
  );

  // 响应拦截器 - 处理 API v2 统一响应格式
  instance.interceptors.response.use(
    (response: AxiosResponse) => {
      // API v2 响应格式: { code, message, data, pagination? }
      // 直接返回 data，让调用方处理 code 和 data 字段
      // eslint-disable-next-line @typescript-eslint/no-unsafe-return
      return response.data;
    },
    (error: AxiosError<ApiErrorV2>) => {
      if (error.response) {
        const { status, data } = error.response;

        // 处理 401 未授权 - 清除 token 并跳转到登录页
        if (status === 401) {
          localStorage.removeItem('token');
          localStorage.removeItem('user');
          window.location.href = '/login';
        }

        // 处理 403 禁止访问
        if (status === 403) {
          console.error('权限不足');
        }

        // 处理 404 未找到
        if (status === 404) {
          console.error('请求的资源不存在');
        }

        // 处理 500 服务器错误
        if (status >= 500) {
          console.error('服务器错误，请稍后重试');
        }

        // 返回后端提供的错误信息（API v2 格式）
        const apiError: ApiErrorV2 = data || {
          code: status,
          message: '未知错误',
        };
        return Promise.reject(apiError);
      } else if (error.request) {
        // 请求已发出但没有收到响应
        const apiError: ApiErrorV2 = {
          code: 0,
          message: '网络错误，请检查您的连接',
        };
        return Promise.reject(apiError);
      } else {
        // 请求配置出错
        const apiError: ApiErrorV2 = {
          code: -1,
          message: '请求配置错误',
        };
        return Promise.reject(apiError);
      }
    }
  );

  return instance;
};

// 导出 axios 实例
export const request = createAxiosInstance();

// 导出常用的请求方法封装
export const http = {
  get: <T = unknown>(url: string, config?: Partial<InternalAxiosRequestConfig>) => {
    return request.get<unknown, T>(url, config);
  },

  post: <T = unknown>(
    url: string,
    data?: unknown,
    config?: Partial<InternalAxiosRequestConfig>
  ) => {
    return request.post<unknown, T>(url, data, config);
  },

  put: <T = unknown>(url: string, data?: unknown, config?: Partial<InternalAxiosRequestConfig>) => {
    return request.put<unknown, T>(url, data, config);
  },

  patch: <T = unknown>(
    url: string,
    data?: unknown,
    config?: Partial<InternalAxiosRequestConfig>
  ) => {
    return request.patch<unknown, T>(url, data, config);
  },

  delete: <T = unknown>(url: string, config?: Partial<InternalAxiosRequestConfig>) => {
    return request.delete<unknown, T>(url, config);
  },
};

// 导出类型
export type { AxiosInstance, AxiosError, InternalAxiosRequestConfig, AxiosResponse };
