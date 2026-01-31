import { http } from '../utils/request';
import type {
  Comment,
  CommentCreateRequest,
  CommentUpdateRequest,
  CommentPatchRequest,
  ApiResponseV2,
  GitHubAuthResponse
} from '../types';

export const commentsApi = {
  /**
   * 获取单条评论
   */
  getComment: (id: string) => {
    return http.get<ApiResponseV2<Comment>>(`/comments/${id}`);
  },

  /**
   * 更新评论（全量更新）
   */
  updateComment: (id: string, data: CommentUpdateRequest) => {
    return http.put<ApiResponseV2<Comment>>(`/comments/${id}`, data);
  },

  /**
   * 部分更新评论
   * API v2 新增方法
   */
  patchComment: (id: string, data: CommentPatchRequest) => {
    return http.patch<ApiResponseV2<Comment>>(`/comments/${id}`, data);
  },

  /**
   * 删除评论
   */
  deleteComment: (id: string) => {
    return http.delete<void>(`/comments/${id}`);
  },

  /**
   * 创建评论（注册用户）
   * 注意：此方法保留用于直接调用 /comments 端点
   * 推荐使用 postsApi.createPostComment() 通过文章创建评论
   */
  createComment: (data: CommentCreateRequest) => {
    return http.post<ApiResponseV2<Comment>>('/comments', data);
  },

  /**
   * 创建评论（GitHub 用户）
   */
  createGitHubComment: (data: CommentCreateRequest & { access_token: string }) => {
    return http.post<ApiResponseV2<Comment>>('/comments/github', data);
  },

  /**
   * 获取 GitHub 授权 URL
   * API v2: 端点从 /github/auth-url 改为 /github/auth
   */
  getGitHubAuthUrl: () => {
    return http.get<ApiResponseV2<GitHubAuthResponse>>('/comments/github/auth');
  },
};

export default commentsApi;
