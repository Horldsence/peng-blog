import { http } from '../utils/request';
import type {
  Category,
  CategoryCreateRequest,
  CategoryUpdateRequest,
  ApiResponseV2,
  ApiListResponseV2,
  Post,
} from '../types';

export const categoriesApi = {
  /**
   * 获取分类列表
   */
  getCategories: (params?: { page?: number; per_page?: number }) => {
    return http.get<ApiListResponseV2<Category>>('/categories', { params } as any);
  },

  /**
   * 获取单个分类
   */
  getCategory: (id: string) => {
    return http.get<ApiResponseV2<Category>>(`/categories/${id}`);
  },

  /**
   * 创建分类
   * 需要管理员权限 (USER_MANAGE)
   */
  createCategory: (data: CategoryCreateRequest) => {
    return http.post<ApiResponseV2<Category>>('/categories', data);
  },

  /**
   * 更新分类（部分更新）
   * 需要管理员权限 (USER_MANAGE)
   */
  patchCategory: (id: string, data: CategoryUpdateRequest) => {
    return http.patch<ApiResponseV2<Category>>(`/categories/${id}`, data);
  },

  /**
   * 删除分类
   * 需要管理员权限 (USER_MANAGE)
   */
  deleteCategory: (id: string) => {
    return http.delete<void>(`/categories/${id}`);
  },

  /**
   * 获取分类下的文章列表
   */
  getCategoryPosts: (id: string, params?: { page?: number; per_page?: number }) => {
    return http.get<ApiListResponseV2<Post>>(`/categories/${id}/posts`, { params } as any);
  },
};

export default categoriesApi;
