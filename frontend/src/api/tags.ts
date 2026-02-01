import { http } from '../utils/request';
import type { Tag, TagCreateRequest, ApiResponseV2, ApiListResponseV2, Post } from '../types';

export const tagsApi = {
  /**
   * 获取标签列表
   */
  getTags: (params?: { page?: number; per_page?: number }) => {
    return http.get<ApiListResponseV2<Tag>>('/tags', { params } as any);
  },

  /**
   * 获取单个标签
   */
  getTag: (id: string) => {
    return http.get<ApiResponseV2<Tag>>(`/tags/${id}`);
  },

  /**
   * 创建标签
   * 需要管理员权限 (USER_MANAGE)
   */
  createTag: (data: TagCreateRequest) => {
    return http.post<ApiResponseV2<Tag>>('/tags', data);
  },

  /**
   * 删除标签
   * 需要管理员权限 (USER_MANAGE)
   */
  deleteTag: (id: string) => {
    return http.delete<void>(`/tags/${id}`);
  },

  /**
   * 获取标签下的文章列表
   */
  getTagPosts: (id: string, params?: { page?: number; per_page?: number }) => {
    return http.get<ApiListResponseV2<Post>>(`/tags/${id}/posts`, { params } as any);
  },
};

export default tagsApi;
