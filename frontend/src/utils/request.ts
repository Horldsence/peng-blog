import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig, AxiosResponse } from 'axios';
import { ApiError } from '../types';

// 创建 axios 实例
const createAxiosInstance = (): AxiosInstance => {
  const instance = axios.create({
    baseURL: '/api', // 使用代理，开发环境指向 http://localhost:3000/api
    timeout: 30000,
    headers: {
      'Content-Type': 'application/json',
    },
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

  // 响应拦截器 - 统一处理错误
  instance.interceptors.response.use(
    (response: AxiosResponse) => {
      return response;
    },
    (error: AxiosError<ApiError>) => {
      if (error.response) {
        const { status, data } = error.response;
        
        // 处理 401 未授权 - 清除 token 并跳转到登录页
        if (status === 401) {
          localStorage.removeItem('token');
          localStorage.removeItem('user');
          // 可以在这里跳转到登录页
          window.location.href = '/login';
        }
        
        // 处理 403 禁止访问
        if (status === 403) {
          console.error('没有权限访问该资源');
        }
        
        // 处理 404 未找到
        if (status === 404) {
          console.error('请求的资源不存在');
        }
        
        // 处理 500 服务器错误
        if (status >= 500) {
          console.error('服务器错误，请稍后重试');
        }
        
        // 返回后端提供的错误信息
        const apiError: ApiError = data || {
          error: 'Unknown Error',
          message: '未知错误',
        };
        return Promise.reject(apiError);
      } else if (error.request) {
        // 请求已发出但没有收到响应
        const apiError: ApiError = {
          error: 'Network Error',
          message: '网络错误，请检查您的连接',
        };
        return Promise.reject(apiError);
      } else {
        // 请求配置出错
        const apiError: ApiError = {
          error: 'Request Error',
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
  get: <T = any>(url: string, config?: InternalAxiosRequestConfig) => {
    return request.get<any, T>(url, config);
  },

  post: <T = any>(url: string, data?: any, config?: any) => {
    return request.post<any, T>(url, data, config);
  },

  put: <T = any>(url: string, data?: any, config?: any) => {
    return request.put<any, T>(url, data, config);
  },

  patch: <T = any>(url: string, data?: any, config?: any) => {
    return request.patch<any, T>(url, data, config);
  },

  delete: <T = any>(url: string, config?: any) => {
    return request.delete<any, T>(url, config);
  },
};

// 导出类型
export type { AxiosInstance, AxiosError, InternalAxiosRequestConfig, AxiosResponse };