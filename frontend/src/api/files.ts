import { http } from '../utils/request';
import type {
  FileInfo,
  FileUploadParams,
  PaginatedResponse,
  ApiResponse,
  PaginationParams
} from '../types';

/**
 * 文件 API 模块
 * 处理文件的上传、下载和管理操作
 */
export const filesApi = {
  /**
   * 上传文件
   * @param params 包含文件对象的参数
   * @returns 上传成功的文件信息
   */
  uploadFile: (params: FileUploadParams) => {
    const formData = new FormData();
    formData.append('file', params.file);
    
    return http.post<FileInfo>('/files', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      } as any,
    });
  },

  /**
   * 获取文件信息
   * @param id 文件 ID
   * @returns 文件信息
   */
  getFile: (id: string) => {
    return http.get<FileInfo>(`/files/${id}`);
  },

  /**
   * 下载文件
   * @param id 文件 ID
   * @returns 文件的 Blob 数据
   */
  downloadFile: (id: string) => {
    return http.get<Blob>(`/files/${id}/download`, {
      responseType: 'blob',
    } as any);
  },

  /**
   * 获取用户文件列表
   * 支持分页
   * @param params 分页参数
   * @returns 文件列表
   */
  getFiles: (params?: PaginationParams) => {
    return http.get<PaginatedResponse<FileInfo>>('/files', { params } as any);
  },

  /**
   * 删除文件
   * @param id 文件 ID
   * @returns 删除成功的消息
   */
  deleteFile: (id: string) => {
    return http.delete<ApiResponse<{ message: string }>>(`/files/${id}`);
  },
};

// 默认导出
export default filesApi;