import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import {
  Card,
  Button,
  Title2,
  Title3,
  Body1,
  Caption1,
  Spinner,
  Badge,
  tokens,
  Tab,
  TabList,
  Divider,
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import {
  HomeRegular,
  DocumentRegular,
  PeopleRegular,
  SettingsRegular,
  ArrowLeftRegular,
  EditRegular,
  DeleteRegular,
  EyeRegular,
  EyeOffRegular,
  AddRegular,
} from '@fluentui/react-icons';
import { authApi, postsApi, usersApi, statsApi } from '../api';
import { useToast } from '../components/ui/Toast';
import type { Post, User, AdminStats } from '../types';
import { Permission } from '../types';

const useStyles = makeStyles({
  container: {
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
  },
  card: {
    borderRadius: '0',
    minHeight: '100%',
    height: '100%',
    border: 'none',
    boxShadow: 'none',
    backgroundColor: tokens.colorNeutralBackground3,
  },
  layout: {
    display: 'flex',
    height: '100%',
    minHeight: 'calc(100vh - 48px)', 
  },
  sidebar: {
    width: '260px',
    backgroundColor: tokens.colorNeutralBackground2,
    padding: '24px',
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    display: 'flex',
    flexDirection: 'column',
  },
  sidebarHeader: {
    marginBottom: '32px',
  },
  sidebarFooter: {
    marginTop: 'auto',
  },
  userInfo: {
    fontWeight: tokens.fontWeightSemibold,
    marginBottom: '8px',
  },
  mainContent: {
    flex: '1',
    padding: '32px',
    overflowY: 'auto',
    backgroundColor: tokens.colorNeutralBackground3,
  },
  errorBox: {
    padding: '12px 16px',
    marginBottom: '24px',
    backgroundColor: tokens.colorStatusDangerBackground1,
    border: `1px solid ${tokens.colorStatusDangerBorder1}`,
    borderRadius: tokens.borderRadiusMedium,
    color: tokens.colorStatusDangerForeground1,
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  loadingContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '48px',
  },
  dashboardGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
    gap: '16px',
  },
  statCard: {
    padding: '20px',
    borderRadius: tokens.borderRadiusLarge,
  },
  statIcon: {
    fontSize: '32px',
  },
  statValue: {
    fontSize: '24px',
    fontWeight: tokens.fontWeightBold,
  },
  statLabel: {
    color: tokens.colorNeutralForeground2,
  },
  headerRow: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '24px',
  },
  listCard: {
    borderRadius: tokens.borderRadiusLarge,
    padding: '0',
  },
  listItem: {
    display: 'flex',
    alignItems: 'center',
    padding: '16px 20px',
    gap: '16px',
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    ':last-child': {
      borderBottom: 'none',
    },
  },
  listItemContent: {
    flex: '1',
  },
  listItemTitle: {
    color: tokens.colorNeutralForeground1,
    textDecoration: 'none',
    fontWeight: tokens.fontWeightSemibold,
    ':hover': {
      textDecoration: 'underline',
    },
  },
  listItemMeta: {
    display: 'flex',
    gap: '12px',
    marginTop: '4px',
  },
  metaText: {
    color: tokens.colorNeutralForeground2,
  },
  actions: {
    display: 'flex',
    gap: '8px',
  },
  emptyState: {
    padding: '48px',
    textAlign: 'center',
  },
});

export function AdminPage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const toast = useToast();
  const [currentUser, setCurrentUser] = useState<any>(null);
  const [activeTab, setActiveTab] = useState<'dashboard' | 'posts' | 'users' | 'settings'>('dashboard');
  const [stats, setStats] = useState<AdminStats | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');

  const hasAdminPermission = (user: User | null) => {
    if (!user) return false;
    const permissions = typeof user.permissions === 'string'
      ? parseInt(user.permissions, 10)
      : user.permissions;
    return (permissions & Permission.USER_MANAGE) !== 0;
  };

  useEffect(() => {
    const checkAuth = () => {
      const authenticated = authApi.isAuthenticated();
      if (!authenticated) {
        navigate('/login');
        return;
      }

      const user = authApi.getCurrentUser();
      if (!user) {
        toast.showError('无法获取用户信息，请重新登录');
        navigate('/login');
        return;
      }

      setCurrentUser(user);

      if (!hasAdminPermission(user)) {
        toast.showError('需要管理员权限才能访问此页面');
        navigate('/');
        return;
      }
    };

    checkAuth();
  }, [navigate, toast]);

  useEffect(() => {
    if (hasAdminPermission(currentUser)) {
      fetchData();
    }
  }, [currentUser, activeTab]);

  const fetchData = async () => {
    setLoading(true);
    setError('');

    try {
      if (activeTab === 'dashboard') {
        const statsResponse = await statsApi.getAdminStats();
        setStats(statsResponse.data);
      } else if (activeTab === 'posts') {
        const postsResponse = await postsApi.getPosts({ page: 1, per_page: 50 });
        setPosts(postsResponse.data);
      } else if (activeTab === 'users') {
        const usersResponse = await usersApi.getUsers({ page: 1, per_page: 50 });
        setUsers(usersResponse.data);
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
      toast.showSuccess('文章删除成功');
    } catch (err: any) {
      toast.showError(err.message || '删除失败');
    }
  };

  const handleTogglePublish = async (post: Post) => {
    try {
      if (post.published_at) {
        await postsApi.unpublishPost(post.id);
        toast.showSuccess('文章已取消发布');
      } else {
        await postsApi.publishPost(post.id);
        toast.showSuccess('文章发布成功');
      }
      fetchData();
    } catch (err: any) {
      toast.showError(err.message || '操作失败');
    }
  };

  const handleDeleteUser = async (userId: string) => {
    if (!confirm('确定要删除这个用户吗?')) return;

    try {
      await usersApi.deleteUser(userId);
      setUsers(users.filter(u => u.id !== userId));
      toast.showSuccess('用户删除成功');
    } catch (err: any) {
      toast.showError(err.message || '删除失败');
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

  if (!hasAdminPermission(currentUser)) {
    return (
      <div className={styles.loadingContainer}>
        <Spinner size="large" />
      </div>
    );
  }

  const statCards = [
    { icon: '📝', label: '文章总数', value: stats?.total_posts || 0, color: 'brand' },
    { icon: '👥', label: '用户总数', value: stats?.total_users || 0, color: 'success' },
    { icon: '💬', label: '评论总数', value: stats?.total_comments || 0, color: 'warning' },
    { icon: '📁', label: '文件总数', value: stats?.total_files || 0, color: 'important' },
    { icon: '👁', label: '总访问量', value: stats?.total_visits || 0, color: 'severe' },
    { icon: '📅', label: '今日访问', value: stats?.today_visits || 0, color: 'success' },
  ];

  return (
    <div className={styles.container}>
      <Card className={styles.card}>
        <div className={styles.layout}>
          {/* 侧边栏 */}
          <div className={styles.sidebar}>
            <div className={styles.sidebarHeader}>
              <Title3>管理后台</Title3>
            </div>

            <TabList
              vertical
              selectedValue={activeTab}
              onTabSelect={(_, data) => setActiveTab(data.value as any)}
            >
              <Tab icon={<HomeRegular />} value="dashboard">
                仪表板
              </Tab>
              <Tab icon={<DocumentRegular />} value="posts">
                文章管理
              </Tab>
              <Tab icon={<PeopleRegular />} value="users">
                用户管理
              </Tab>
              <Tab icon={<SettingsRegular />} value="settings">
                设置
              </Tab>
            </TabList>

            <div className={styles.sidebarFooter}>
              <Divider style={{ margin: '24px 0' }} />
              <div className={styles.userInfo}>
                {currentUser?.username}
              </div>
              <Button
                appearance="transparent"
                icon={<ArrowLeftRegular />}
                onClick={() => navigate('/')}
                size="small"
              >
                返回网站
              </Button>
            </div>
          </div>

          {/* 主内容区 */}
          <div className={styles.mainContent}>
            {/* 错误提示 */}
            {error && (
              <div className={styles.errorBox}>
                <Body1>{error}</Body1>
                <Button
                  appearance="transparent"
                  size="small"
                  onClick={() => setError('')}
                >
                  ×
                </Button>
              </div>
            )}

            {/* 加载状态 */}
            {loading && (
              <div className={styles.loadingContainer}>
                <Spinner size="large" />
              </div>
            )}

            {/* 仪表板 */}
            {!loading && activeTab === 'dashboard' && stats && (
              <div>
                <Title2 style={{ marginBottom: '24px' }}>仪表板</Title2>
                <div className={styles.dashboardGrid}>
                  {statCards.map((stat, index) => (
                    <Card
                      key={index}
                      className={styles.statCard}
                    >
                      <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                        <span className={styles.statIcon}>{stat.icon}</span>
                        <div>
                          <Body1 className={styles.statValue}>
                            {stat.value}
                          </Body1>
                          <Caption1 className={styles.statLabel}>
                            {stat.label}
                          </Caption1>
                        </div>
                      </div>
                    </Card>
                  ))}
                </div>
              </div>
            )}

            {/* 文章管理 */}
            {!loading && activeTab === 'posts' && (
              <div>
                <div className={styles.headerRow}>
                  <Title2>文章管理</Title2>
                  <Button
                    appearance="primary"
                    icon={<AddRegular />}
                    onClick={() => navigate('/admin/posts/new')}
                  >
                    新建文章
                  </Button>
                </div>

                <Card className={styles.listCard}>
                  {posts.length === 0 ? (
                    <div className={styles.emptyState}>
                      <Body1 className={styles.metaText}>
                        暂无文章
                      </Body1>
                    </div>
                  ) : (
                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                      {posts.map((post) => (
                        <div key={post.id} className={styles.listItem}>
                          <div className={styles.listItemContent}>
                            <Link
                              to={`/post/${post.id}`}
                              className={styles.listItemTitle}
                            >
                              {post.title}
                            </Link>
                            <div className={styles.listItemMeta}>
                              <Caption1 className={styles.metaText}>
                                {formatDate(post.created_at)}
                              </Caption1>
                              <Caption1 className={styles.metaText}>
                                {post.views} 次阅读
                              </Caption1>
                            </div>
                          </div>

                          <Badge
                            appearance={post.published_at ? 'filled' : 'outline'}
                            color={post.published_at ? 'success' : 'warning'}
                          >
                            {post.published_at ? '已发布' : '草稿'}
                          </Badge>

                          <div className={styles.actions}>
                            <Button
                              appearance="transparent"
                              icon={<EditRegular />}
                              size="small"
                              onClick={() => navigate(`/admin/posts/edit/${post.id}`)}
                            >
                              编辑
                            </Button>
                            <Button
                              appearance="transparent"
                              icon={post.published_at ? <EyeOffRegular /> : <EyeRegular />}
                              size="small"
                              onClick={() => handleTogglePublish(post)}
                            >
                              {post.published_at ? '取消发布' : '发布'}
                            </Button>
                            <Button
                              appearance="transparent"
                              icon={<DeleteRegular />}
                              size="small"
                              onClick={() => handleDeletePost(post.id)}
                            >
                              删除
                            </Button>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </Card>
              </div>
            )}

            {/* 用户管理 */}
            {!loading && activeTab === 'users' && (
              <div>
                <Title2 style={{ marginBottom: '24px' }}>用户管理</Title2>

                <Card className={styles.listCard}>
                  {users.length === 0 ? (
                    <div className={styles.emptyState}>
                      <Body1 className={styles.metaText}>
                        暂无用户
                      </Body1>
                    </div>
                  ) : (
                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                      {users.map((user) => (
                        <div key={user.id} className={styles.listItem}>
                          <div className={styles.listItemContent}>
                            <Body1 style={{ fontWeight: '600' }}>{user.username}</Body1>
                            <Caption1 className={styles.metaText}>
                              {formatDate(user.created_at)}
                            </Caption1>
                          </div>

                          <Badge
                            appearance="filled"
                            color={(user.permissions & Permission.USER_MANAGE) !== 0 ? 'brand' : 'success'}
                          >
                            {(user.permissions & Permission.USER_MANAGE) !== 0 ? '管理员' : '普通用户'}
                          </Badge>

                          <Button
                            appearance="transparent"
                            icon={<DeleteRegular />}
                            size="small"
                            disabled={user.id === currentUser.id}
                            onClick={() => handleDeleteUser(user.id)}
                          >
                            删除
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                </Card>
              </div>
            )}

            {/* 设置 */}
            {!loading && activeTab === 'settings' && (
              <div>
                <Title2 style={{ marginBottom: '24px' }}>设置</Title2>
                <Card style={{ borderRadius: tokens.borderRadiusLarge, padding: '32px' }}>
                  <Body1 className={styles.metaText}>
                    设置功能开发中...
                  </Body1>
                </Card>
              </div>
            )}
          </div>
        </div>
      </Card>
    </div>
  );
};
