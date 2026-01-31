import React, { useState, useEffect } from 'react';
import { postsApi } from '../api';
import type { Post, PostListParams } from '../types';

interface PostListProps {
  userId?: string;
  onPostClick?: (post: Post) => void;
}

const PostList: React.FC<PostListProps> = ({ userId, onPostClick }) => {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [pageSize] = useState<number>(10);
  const [totalPosts, setTotalPosts] = useState<number>(0);

  const fetchPosts = async (page: number = 1) => {
    setLoading(true);
    setError('');

    try {
      const params: PostListParams = {
        page,
        per_page: pageSize,
      };

      if (userId) {
        params.author = userId;
      }

      const response = await postsApi.getPosts(params);
      setPosts(response.data);
      setTotalPosts(response.pagination?.total || 0);
      setCurrentPage(response.pagination?.page || 1);
    } catch (err: any) {
      const errorMessage = err.message || '获取文章列表失败';
      setError(errorMessage);
      console.error('获取文章列表失败:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosts(currentPage);
  }, [currentPage, userId]);

  const handlePageChange = (newPage: number) => {
    if (newPage >= 1 && newPage <= Math.ceil(totalPosts / pageSize)) {
      setCurrentPage(newPage);
    }
  };

  const handlePostClick = (post: Post) => {
    if (onPostClick) {
      onPostClick(post);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const totalPages = Math.ceil(totalPosts / pageSize);

  return (
    <div className="post-list">
      <h2>文章列表</h2>

      {loading && (
        <div className="loading-state">
          <p>加载中...</p>
        </div>
      )}

      {error && (
        <div className="error-state">
          <p>{error}</p>
          <button onClick={() => fetchPosts(currentPage)} className="retry-button">
            重试
          </button>
        </div>
      )}

      {!loading && !error && posts.length === 0 && (
        <div className="empty-state">
          <p>暂无文章</p>
        </div>
      )}

      {!loading && !error && posts.length > 0 && (
        <>
          <div className="posts-container">
            {posts.map((post) => (
              <div
                key={post.id}
                className={`post-item ${!post.published_at ? 'unpublished' : ''}`}
                onClick={() => handlePostClick(post)}
              >
                <div className="post-header">
                  <h3 className="post-title">{post.title}</h3>
                  <div className="post-status">
                    {post.published_at ? (
                      <span className="status-badge published">已发布</span>
                    ) : (
                      <span className="status-badge unpublished">未发布</span>
                    )}
                  </div>
                </div>
                
                <div className="post-excerpt">
                  {post.content.length > 200 
                    ? `${post.content.substring(0, 200)}...` 
                    : post.content}
                </div>
                
                <div className="post-meta">
                  <span className="meta-item">
                    <i className="icon-views">👁</i>
                    {post.views}
                  </span>
                  <span className="meta-item">
                    <i className="icon-date">📅</i>
                    {formatDate(post.created_at)}
                  </span>
                  {post.published_at && (
                    <span className="meta-item">
                      <i className="icon-published">✓</i>
                      发布于 {formatDate(post.published_at)}
                    </span>
                  )}
                  {post.updated_at && (
                    <span className="meta-item">
                      <i className="icon-updated">🔄</i>
                      更新于 {formatDate(post.updated_at)}
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>

          {totalPages > 1 && (
            <div className="pagination">
              <button
                onClick={() => handlePageChange(currentPage - 1)}
                disabled={currentPage === 1}
                className="pagination-button"
              >
                上一页
              </button>
              
              <div className="page-info">
                第 {currentPage} / {totalPages} 页
              </div>
              
              <button
                onClick={() => handlePageChange(currentPage + 1)}
                disabled={currentPage === totalPages}
                className="pagination-button"
              >
                下一页
              </button>
            </div>
          )}

          <div className="total-info">
            共 {totalPosts} 篇文章
          </div>
        </>
      )}
    </div>
  );
};

export default PostList;
