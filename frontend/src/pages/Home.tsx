import React, { useState, useEffect } from 'react';
import PostList from '../components/PostList';
import { authApi, statsApi } from '../api';
import type { Post, GlobalStats } from '../types';

const Home: React.FC = () => {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [user, setUser] = useState<any>(null);
  const [stats, setStats] = useState<GlobalStats | null>(null);
  const [selectedPost, setSelectedPost] = useState<Post | null>(null);
  const [showPostDetail, setShowPostDetail] = useState<boolean>(false);

  // 检查登录状态并获取用户信息
  useEffect(() => {
    const checkAuth = () => {
      const authenticated = authApi.isAuthenticated();
      setIsAuthenticated(authenticated);
      if (authenticated) {
        const currentUser = authApi.getCurrentUser();
        setUser(currentUser);
      }
    };

    checkAuth();
  }, []);

  // 获取全局统计信息
  useEffect(() => {
    const fetchStats = async () => {
      try {
        const globalStats = await statsApi.getGlobalStats();
        setStats(globalStats);
        
        // 记录访问
        await statsApi.recordVisit();
      } catch (error) {
        console.error('获取统计信息失败:', error);
      }
    };

    fetchStats();
  }, []);

  // 处理登出
  const handleLogout = async () => {
    try {
      await authApi.logout();
      authApi.clearAuth();
      setIsAuthenticated(false);
      setUser(null);
      console.log('登出成功');
    } catch (error) {
      console.error('登出失败:', error);
      // 即使后端登出失败，也清除本地登录状态
      authApi.clearAuth();
      setIsAuthenticated(false);
      setUser(null);
    }
  };

  // 处理文章点击
  const handlePostClick = (post: Post) => {
    setSelectedPost(post);
    setShowPostDetail(true);
    
    // 记录文章阅读
    statsApi.recordPostView(post.id).catch(error => {
      console.error('记录文章阅读失败:', error);
    });
  };

  // 返回文章列表
  const handleBackToList = () => {
    setShowPostDetail(false);
    setSelectedPost(null);
  };

  // 格式化日期
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

  return (
    <div className="home-page">
      {/* 顶部导航栏 */}
      <header className="header">
        <div className="header-content">
          <h1 className="blog-title">Peng Blog</h1>
          <nav className="nav">
            {isAuthenticated ? (
              <div className="user-nav">
                <span className="welcome-message">
                  欢迎回来，{user?.username || '用户'}
                </span>
                <button onClick={handleLogout} className="logout-button">
                  登出
                </button>
              </div>
            ) : (
              <div className="auth-nav">
                <a href="/login" className="login-link">登录</a>
                <a href="/register" className="register-link">注册</a>
              </div>
            )}
          </nav>
        </div>
      </header>

      {/* 主要内容区域 */}
      <main className="main-content">
        {/* 统计信息栏 */}
        {stats && (
          <section className="stats-section">
            <div className="stats-container">
              <div className="stat-item">
                <span className="stat-label">总访问量</span>
                <span className="stat-value">{stats.total_visits.toLocaleString()}</span>
              </div>
              <div className="stat-item">
                <span className="stat-label">今日访问</span>
                <span className="stat-value">{stats.today_visits.toLocaleString()}</span>
              </div>
              <div className="stat-item">
                <span className="stat-label">最后更新</span>
                <span className="stat-value">{formatDate(stats.last_updated)}</span>
              </div>
            </div>
          </section>
        )}

        {/* 文章内容区域 */}
        <section className="posts-section">
          {!showPostDetail ? (
            <>
              <div className="section-header">
                <h2>最新文章</h2>
                {isAuthenticated && (
                  <button className="create-post-button">
                    + 写文章
                  </button>
                )}
              </div>
              <PostList onPostClick={handlePostClick} />
            </>
          ) : selectedPost ? (
            <div className="post-detail">
              <div className="post-detail-header">
                <button onClick={handleBackToList} className="back-button">
                  ← 返回列表
                </button>
                <div className="post-actions">
                  <span className="post-views">
                    👁 {selectedPost.views} 次阅读
                  </span>
                  {isAuthenticated && (
                    <button className="edit-button">编辑</button>
                  )}
                </div>
              </div>
              
              <article className="post-article">
                <h1 className="post-detail-title">{selectedPost.title}</h1>
                
                <div className="post-detail-meta">
                  <span className="meta-item">
                    📅 创建于 {formatDate(selectedPost.created_at)}
                  </span>
                  {selectedPost.updated_at !== selectedPost.created_at && (
                    <span className="meta-item">
                      🔄 更新于 {formatDate(selectedPost.updated_at)}
                    </span>
                  )}
                  {selectedPost.published_at && (
                    <span className="meta-item">
                      ✓ 发布于 {formatDate(selectedPost.published_at)}
                    </span>
                  )}
                </div>
                
                <div className="post-detail-content">
                  {selectedPost.content}
                </div>
              </article>

              {/* 评论区 */}
              <div className="comments-section">
                <h3>评论</h3>
                <div className="comments-placeholder">
                  <p>评论功能开发中...</p>
                </div>
              </div>
            </div>
          ) : null}
        </section>
      </main>

      {/* 页脚 */}
      <footer className="footer">
        <div className="footer-content">
          <p>&copy; 2026 Peng Blog. All rights reserved.</p>
          <div className="footer-links">
            <a href="/about">关于</a>
            <a href="/privacy">隐私政策</a>
            <a href="/terms">使用条款</a>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default Home;