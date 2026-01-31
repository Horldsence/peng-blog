import React, { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { authApi, postsApi, usersApi, statsApi } from '../api';
import type { Post, User, AdminStats } from '../types';

const Admin: React.FC = () => {
  const navigate = useNavigate();
  const [currentUser, setCurrentUser] = useState<any>(null);
  const [activeTab, setActiveTab] = useState<'dashboard' | 'posts' | 'users' | 'settings'>('dashboard');
  const [stats, setStats] = useState<AdminStats | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    const checkAuth = () => {
      const authenticated = authApi.isAuthenticated();
      if (!authenticated) {
        navigate('/login');
        return;
      }

      const user = authApi.getCurrentUser();
      setCurrentUser(user);

      if (!user || user.permissions !== 31) {
        alert('需要管理员权限才能访问此页面');
        navigate('/');
        return;
      }
    };

    checkAuth();
  }, [navigate]);

  useEffect(() => {
    if (currentUser && currentUser.permissions === 31) {
      fetchData();
    }
  }, [currentUser, activeTab]);

  const fetchData = async () => {
    setLoading(true);
    setError('');

    try {
      if (activeTab === 'dashboard') {
        const adminStats = await statsApi.getAdminStats();
        setStats(adminStats);
      } else if (activeTab === 'posts') {
        const postsData = await postsApi.getPosts({ page: 1, page_size: 50 });
        // 后端返回的是纯数组格式，适配为前端需要的格式
        setPosts(Array.isArray(postsData) ? postsData : (postsData.data || []));
      } else if (activeTab === 'users') {
        const usersData = await usersApi.getUsers({ page: 1, page_size: 50 });
        // 后端返回的是纯数组格式，适配为前端需要的格式
        setUsers(Array.isArray(usersData) ? usersData : (usersData.data || []));
      }
    } catch (err: any) {
      const errorMessage = err.message || '获取数据失败';
      setError(errorMessage);
      console.error('获取数据失败:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleDeletePost = async (postId: string) => {
    if (!confirm('确定要删除这篇文章吗?')) return;

    try {
      await postsApi.deletePost(postId);
      setPosts(posts.filter(p => p.id !== postId));
      alert('删除成功');
    } catch (err: any) {
      alert(err.message || '删除失败');
    }
  };

  const handleTogglePublish = async (post: Post) => {
    try {
      if (post.published_at) {
        await postsApi.unpublishPost(post.id);
      } else {
        await postsApi.publishPost(post.id);
      }
      fetchData();
    } catch (err: any) {
      alert(err.message || '操作失败');
    }
  };

  const handleDeleteUser = async (userId: string) => {
    if (!confirm('确定要删除这个用户吗?')) return;

    try {
      await usersApi.deleteUser(userId);
      setUsers(users.filter(u => u.id !== userId));
      alert('删除成功');
    } catch (err: any) {
      alert(err.message || '删除失败');
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    });
  };

  if (!currentUser || currentUser.permissions !== 31) {
    return (
      <div className="admin-page">
        <div className="loading-state">
          <p>验证权限中...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="admin-page">
      <div className="admin-container">
        <aside className="admin-sidebar">
          <h2 className="admin-logo">管理后台</h2>
          <nav className="admin-nav">
            <button
              className={`nav-item ${activeTab === 'dashboard' ? 'active' : ''}`}
              onClick={() => setActiveTab('dashboard')}
            >
              📊 仪表板
            </button>
            <button
              className={`nav-item ${activeTab === 'posts' ? 'active' : ''}`}
              onClick={() => setActiveTab('posts')}
            >
              📝 文章管理
            </button>
            <button
              className={`nav-item ${activeTab === 'users' ? 'active' : ''}`}
              onClick={() => setActiveTab('users')}
            >
              👥 用户管理
            </button>
            <button
              className={`nav-item ${activeTab === 'settings' ? 'active' : ''}`}
              onClick={() => setActiveTab('settings')}
            >
              ⚙️ 设置
            </button>
          </nav>
          <div className="admin-user-info">
            <p>{currentUser.username}</p>
            <Link to="/" className="back-site-link">返回网站</Link>
          </div>
        </aside>

        <main className="admin-content">
          {error && (
            <div className="error-message">
              {error}
              <button onClick={() => setError('')}>×</button>
            </div>
          )}

          {loading && (
            <div className="loading-state">
              <p>加载中...</p>
            </div>
          )}

          {!loading && activeTab === 'dashboard' && stats && (
            <div className="dashboard-view">
              <h1>仪表板</h1>
              <div className="stats-grid">
                <div className="stat-card">
                  <div className="stat-icon">📝</div>
                  <div className="stat-info">
                    <div className="stat-number">{stats.total_posts}</div>
                    <div className="stat-label">文章总数</div>
                  </div>
                </div>
                <div className="stat-card">
                  <div className="stat-icon">👥</div>
                  <div className="stat-info">
                    <div className="stat-number">{stats.total_users}</div>
                    <div className="stat-label">用户总数</div>
                  </div>
                </div>
                <div className="stat-card">
                  <div className="stat-icon">💬</div>
                  <div className="stat-info">
                    <div className="stat-number">{stats.total_comments}</div>
                    <div className="stat-label">评论总数</div>
                  </div>
                </div>
                <div className="stat-card">
                  <div className="stat-icon">📁</div>
                  <div className="stat-info">
                    <div className="stat-number">{stats.total_files}</div>
                    <div className="stat-label">文件总数</div>
                  </div>
                </div>
                <div className="stat-card">
                  <div className="stat-icon">👁</div>
                  <div className="stat-info">
                    <div className="stat-number">{stats.total_visits}</div>
                    <div className="stat-label">总访问量</div>
                  </div>
                </div>
                <div className="stat-card">
                  <div className="stat-icon">📅</div>
                  <div className="stat-info">
                    <div className="stat-number">{stats.today_visits}</div>
                    <div className="stat-label">今日访问</div>
                  </div>
                </div>
              </div>
            </div>
          )}

          {!loading && activeTab === 'posts' && (
            <div className="posts-view">
              <div className="view-header">
                <h1>文章管理</h1>
                <Link to="/admin/posts/new" className="create-button">
                  + 新建文章
                </Link>
              </div>
              <div className="data-table">
                <table>
                  <thead>
                    <tr>
                      <th>标题</th>
                      <th>状态</th>
                      <th>阅读量</th>
                      <th>创建时间</th>
                      <th>操作</th>
                    </tr>
                  </thead>
                  <tbody>
                    {posts.map(post => (
                      <tr key={post.id}>
                        <td>
                          <Link to={`/post/${post.id}`} className="post-link">
                            {post.title}
                          </Link>
                        </td>
                        <td>
                          <span className={`status-badge ${post.published_at ? 'published' : 'draft'}`}>
                            {post.published_at ? '已发布' : '草稿'}
                          </span>
                        </td>
                        <td>{post.views}</td>
                        <td>{formatDate(post.created_at)}</td>
                        <td className="actions">
                          <Link to={`/admin/posts/edit/${post.id}`} className="action-button edit">
                            编辑
                          </Link>
                          <button
                            onClick={() => handleTogglePublish(post)}
                            className="action-button"
                          >
                            {post.published_at ? '取消发布' : '发布'}
                          </button>
                          <button
                            onClick={() => handleDeletePost(post.id)}
                            className="action-button delete"
                          >
                            删除
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
                {posts.length === 0 && (
                  <div className="empty-state">
                    <p>暂无文章</p>
                  </div>
                )}
              </div>
            </div>
          )}

          {!loading && activeTab === 'users' && (
            <div className="users-view">
              <h1>用户管理</h1>
              <div className="data-table">
                <table>
                  <thead>
                    <tr>
                      <th>用户名</th>
                      <th>权限</th>
                      <th>创建时间</th>
                      <th>操作</th>
                    </tr>
                  </thead>
                  <tbody>
                    {users.map(user => (
                      <tr key={user.id}>
                        <td>{user.username}</td>
                        <td>
                          <span className={`permission-badge ${user.permissions === 31 ? 'admin' : 'user'}`}>
                            {user.permissions === 31 ? '管理员' : '普通用户'}
                          </span>
                        </td>
                        <td>{formatDate(user.created_at)}</td>
                        <td className="actions">
                          <button
                            onClick={() => handleDeleteUser(user.id)}
                            className="action-button delete"
                            disabled={user.id === currentUser.id}
                          >
                            删除
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
                {users.length === 0 && (
                  <div className="empty-state">
                    <p>暂无用户</p>
                  </div>
                )}
              </div>
            </div>
          )}

          {!loading && activeTab === 'settings' && (
            <div className="settings-view">
              <h1>设置</h1>
              <div className="settings-content">
                <p>设置功能开发中...</p>
              </div>
            </div>
          )}
        </main>
      </div>
    </div>
  );
};

export default Admin;
