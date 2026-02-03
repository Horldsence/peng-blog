import { http } from '../utils/request';
import type {
  Post,
  PostCreateRequest,
  PostUpdateRequest,
  PostPatchRequest,
  PostListParams,
  PostSearchParams,
  ApiResponseV2,
  ApiListResponseV2,
  Tag,
  Comment,
} from '../types';

export const postsApi = {
  /**
   * 获取文章列表
   * API v2: 支持按 author, category, tag, status 过滤
   */
  getPosts: (params?: PostListParams) => {
    return http.get<ApiListResponseV2<Post>>('/posts', { params });
  },

  /**
   * 搜索文章
   * API v2 新增端点
   */
  searchPosts: (params: PostSearchParams) => {
    return http.get<ApiListResponseV2<Post>>('/posts/search', { params });
  },

  /**
   * 获取单篇文章
   */
  getPost: (id: string) => {
    return http.get<ApiResponseV2<Post>>(`/posts/${id}`);
  },

  /**
   * 创建文章
   * API v2: 创建时不需要传 published 字段，默认为 draft
   */
  createPost: (data: PostCreateRequest) => {
    return http.post<ApiResponseV2<Post>>('/posts', data);
  },

  /**
   * 全量更新文章
   * API v2: 使用 PUT 方法
   */
  updatePost: (id: string, data: PostUpdateRequest) => {
    return http.put<ApiResponseV2<Post>>(`/posts/${id}`, data);
  },

  /**
   * 部分更新文章
   * API v2 新增方法：用于发布/取消发布、更改分类等
   */
  patchPost: (id: string, data: PostPatchRequest) => {
    return http.patch<ApiResponseV2<Post>>(`/posts/${id}`, data);
  },

  /**
   * 删除文章
   * API v2: 返回 204 No Content
   */
  deletePost: (id: string) => {
    return http.delete<void>(`/posts/${id}`);
  },

  /**
   * 发布文章
   * API v2: 使用 PATCH 方法
   */
  publishPost: (id: string) => {
    return http.patch<ApiResponseV2<Post>>(`/posts/${id}`, { status: 'published' });
  },

  /**
   * 取消发布文章
   * API v2: 使用 PATCH 方法
   */
  unpublishPost: (id: string) => {
    return http.patch<ApiResponseV2<Post>>(`/posts/${id}`, { status: 'draft' });
  },

  /**
   * 获取文章评论列表
   * API v2: 端点从 /comments 改为 /posts/{id}/comments
   */
  getPostComments: (postId: string, params?: { page?: number; per_page?: number }) => {
    return http.get<ApiListResponseV2<Comment>>(`/posts/${postId}/comments`, { params });
  },

  /**
   * 为文章添加评论
   * API v2: 端点从 /comments 改为 /posts/{id}/comments
   */
  createPostComment: (postId: string, data: { content: string }) => {
    return http.post<ApiResponseV2<Comment>>(`/posts/${postId}/comments`, data);
  },

  /**
   * 获取文章的所有标签
   * API v2 新增方法
   */
  getPostTags: (postId: string) => {
    return http.get<ApiResponseV2<Tag[]>>(`/posts/${postId}/tags`);
  },

  /**
   * 为文章添加标签
   * API v2: 使用 POST /posts/{id}/tags + { tag_id }
   */
  addPostTag: (postId: string, tagId: string) => {
    return http.post<ApiResponseV2<Tag[]>>(`/posts/${postId}/tags`, { tag_id: tagId });
  },

  /**
   * 移除文章的标签
   * API v2: 使用 DELETE /posts/{id}/tags/{tag_id}
   */
  removePostTag: (postId: string, tagId: string) => {
    return http.delete<ApiResponseV2<Tag[]>>(`/posts/${postId}/tags/${tagId}`);
  },
};

export default postsApi;
