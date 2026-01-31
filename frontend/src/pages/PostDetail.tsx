import React, { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { postsApi, commentsApi, authApi, statsApi } from '../api';
import type { Post, Comment } from '../types';

const PostDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [post, setPost] = useState<Post | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [currentUser, setCurrentUser] = useState<any>(null);
  const [commentContent, setCommentContent] = useState<string>('');
  const [submittingComment, setSubmittingComment] = useState<boolean>(false);

  useEffect(() => {
    const authenticated = authApi.isAuthenticated();
    setIsAuthenticated(authenticated);
    if (authenticated) {
      setCurrentUser(authApi.getCurrentUser());
    }
  }, []);

  useEffect(() => {
    const fetchPostDetail = async () => {
      if (!id) return;

      setLoading(true);
      setError('');

      try {
        const postData = await postsApi.getPost(id);
        setPost(postData);

        await statsApi.recordPostView(id);

        const commentsData = await commentsApi.getCommentsByPost(id, {
          page: 1,
          page_size: 50,
        });
        // 后端返回的是纯数组格式，适配为前端需要的格式
        setComments(Array.isArray(commentsData) ? commentsData : (commentsData.data || []));
      } catch (err: any) {
        const errorMessage = err.message || '获取文章详情失败';
        setError(errorMessage);
        console.error('获取文章详情失败:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchPostDetail();
  }, [id]);

  const handleSubmitComment = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!commentContent.trim()) {
      return;
    }

    if (!isAuthenticated) {
      alert('请先登录后再评论');
      navigate('/login');
      return;
    }

    if (!id) return;

    setSubmittingComment(true);

    try {
      const newComment = await commentsApi.createComment({
        post_id: id,
        content: commentContent,
      });

      setComments([...comments, newComment]);
      setCommentContent('');
    } catch (err: any) {
      alert(err.message || '发表评论失败');
      console.error('发表评论失败:', err);
    } finally {
      setSubmittingComment(false);
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

  if (loading) {
    return (
      <div className="post-detail-page">
        <div className="loading-state">
          <p>加载中...</p>
        </div>
      </div>
    );
  }

  const canEdit = currentUser && post && (currentUser.id === post.user_id || currentUser.permissions >= 16);

  if (!post) {
    return (
      <div className="post-detail-page">
        <div className="loading-state">
          <p>文章不存在</p>
        </div>
      </div>
    );
  }

  return (
    <div className="post-detail-page">
      {error && (
        <div className="error-message">
          {error}
          <button onClick={() => setError('')}>×</button>
        </div>
      )}
      <div className="back-nav">
        <button onClick={() => navigate('/')} className="back-button">
          ← 返回首页
        </button>
        {canEdit && (
          <Link to={`/admin/posts/edit/${post.id}`} className="edit-button">
            编辑文章
          </Link>
        )}
      </div>

      <article className="post-article">
        <header className="post-article-header">
          <h1 className="post-title">{post.title}</h1>

          <div className="post-meta">
            <span className="meta-item">
              👁 {post.views} 次阅读
            </span>
            <span className="meta-item">
              📅 创建于 {formatDate(post.created_at)}
            </span>
            {post.updated_at !== post.created_at && (
              <span className="meta-item">
                🔄 更新于 {formatDate(post.updated_at)}
              </span>
            )}
            {post.published_at && (
              <span className="meta-item published-badge">
                ✓ 发布于 {formatDate(post.published_at)}
              </span>
            )}
          </div>
        </header>

        <div className="post-content">
          {post.content.split('\n').map((paragraph, index) => (
            <p key={index}>{paragraph}</p>
          ))}
        </div>
      </article>

      <section className="comments-section">
        <h2 className="comments-title">评论 ({comments.length})</h2>

        {isAuthenticated ? (
          <form onSubmit={handleSubmitComment} className="comment-form">
            <textarea
              value={commentContent}
              onChange={(e) => setCommentContent(e.target.value)}
              placeholder="写下你的评论..."
              rows={4}
              className="comment-textarea"
            />
            <button
              type="submit"
              disabled={submittingComment || !commentContent.trim()}
              className="submit-comment-button"
            >
              {submittingComment ? '发表中...' : '发表评论'}
            </button>
          </form>
        ) : (
          <div className="comment-login-prompt">
            <p>请 <Link to="/login">登录</Link> 后发表评论</p>
          </div>
        )}

        <div className="comments-list">
          {comments.length === 0 ? (
            <div className="empty-comments">
              <p>还没有评论，快来抢沙发吧！</p>
            </div>
          ) : (
            comments.map((comment) => (
              <div key={comment.id} className="comment-item">
                <div className="comment-header">
                  <span className="comment-author">
                    {comment.github_username || '用户'}
                  </span>
                  <span className="comment-date">
                    {formatDate(comment.created_at)}
                  </span>
                </div>
                <div className="comment-content">
                  {comment.content}
                </div>
              </div>
            ))
          )}
        </div>
      </section>
    </div>
  );
};

export default PostDetail;
