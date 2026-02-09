import { useState, useEffect, useCallback } from 'react';
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
  Tooltip,
  tokens,
  Tab,
  TabList,
  Divider,
  makeStyles,
  Input,
  Switch,
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
  SendRegular,
} from '@fluentui/react-icons';
import { authApi, postsApi, usersApi, statsApi, configApi } from '../api';
import { useToast } from '../components/ui/Toast';
import type { Post, User, AdminStats, Config, UpdateConfigRequest } from '../types';
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
    padding: '48px 0',
    textAlign: 'center',
  },
  settingsSection: {
    marginBottom: '32px',
  },
  settingsGroup: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    marginBottom: '24px',
  },
  settingsRow: {
    display: 'grid',
    gridTemplateColumns: '200px 1fr',
    gap: '24px',
    alignItems: 'start',
    '@media (max-width: 768px)': {
      gridTemplateColumns: '1fr',
      gap: '8px',
    },
  },
  settingsActions: {
    display: 'flex',
    justifyContent: 'flex-end',
    marginTop: '24px',
    paddingTop: '24px',
    borderTop: `1px solid ${tokens.colorNeutralStroke2}`,
  },
});

export function AdminPage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const toast = useToast();
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  const [activeTab, setActiveTab] = useState<'dashboard' | 'posts' | 'users' | 'settings'>(
    'dashboard'
  );
  const [stats, setStats] = useState<AdminStats | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [config, setConfig] = useState<Config | null>(null);
  const [pendingConfig, setPendingConfig] = useState<UpdateConfigRequest>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  const [submittingIndexNow, setSubmittingIndexNow] = useState<Set<string>>(new Set());

  const EnvOverrideBadge = ({ message }: { message?: string }) => (
    <Tooltip
      content={
        <div style={{ fontSize: '12px' }}>
          此值由环境变量设置{message ? `: ${message}` : ''}
          <br />
          修改配置文件不会生效
        </div>
      }
      relationship="label"
    >
      <Badge size="small" color="warning" style={{ marginLeft: '8px' }}>
        ENV
      </Badge>
    </Tooltip>
  );

  const hasAdminPermission = (user: User | null) => {
    if (!user) return false;
    const permissions =
      typeof user.permissions === 'string' ? parseInt(user.permissions, 10) : user.permissions;
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

  const fetchData = useCallback(async () => {
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
      } else if (activeTab === 'settings') {
        const configResponse = await configApi.getConfig();
        setConfig(configResponse.data);
        setPendingConfig({});
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取数据失败';
      setError(errorMessage);
      console.error('获取数据失败:', err);
    } finally {
      setLoading(false);
    }
  }, [activeTab]);

  useEffect(() => {
    if (hasAdminPermission(currentUser)) {
      void fetchData();
    }
  }, [currentUser, activeTab, fetchData]);

  const handleDeletePost = async (postId: string) => {
    // eslint-disable-next-line no-alert
    if (!confirm('确定要删除这篇文章吗?')) return;

    try {
      await postsApi.deletePost(postId);
      setPosts(posts.filter((p) => p.id !== postId));
      toast.showSuccess('文章删除成功');
    } catch (err) {
      toast.showError(err instanceof Error ? err.message : '删除失败');
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
      void fetchData();
    } catch (err) {
      toast.showError(err instanceof Error ? err.message : '操作失败');
    }
  };

  const handleSubmitToIndexNow = async (postId: string) => {
    setSubmittingIndexNow((prev) => new Set(prev).add(postId));

    try {
      toast.showInfo('正在通知搜索引擎...');
      await postsApi.submitToIndexNow(postId);
      toast.showSuccess('已成功通知搜索引擎');
      void fetchData();
    } catch (err) {
      toast.showError(err instanceof Error ? err.message : '通知搜索引擎失败');
    } finally {
      setSubmittingIndexNow((prev) => {
        const next = new Set(prev);
        next.delete(postId);
        return next;
      });
    }
  };

  const handleDeleteUser = async (userId: string) => {
    // eslint-disable-next-line no-alert
    if (!confirm('确定要删除这个用户吗?')) return;

    try {
      await usersApi.deleteUser(userId);
      setUsers(users.filter((u) => u.id !== userId));
      toast.showSuccess('用户删除成功');
    } catch (err) {
      toast.showError(err instanceof Error ? err.message : '删除失败');
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

  const handleSaveConfig = async () => {
    if (Object.keys(pendingConfig).length === 0) {
      toast.showSuccess('没有更改需要保存');
      return;
    }

    try {
      await configApi.updateConfig(pendingConfig);
      toast.showSuccess('配置更新成功');
      setPendingConfig({});
      void fetchData();
    } catch (err) {
      toast.showError(err instanceof Error ? err.message : '配置更新失败');
    }
  };

  const handleConfigChange = (
    section: keyof UpdateConfigRequest,
    field: string,
    value: string | number | boolean
  ) => {
    setPendingConfig((prev) => ({
      ...prev,
      [section]: {
        ...prev[section],
        [field]: value,
      },
    }));

    // Optimistically update local config for display
    if (config) {
      setConfig({
        ...config,
        [section]: {
          ...config[section as keyof Config],
          [field]: value,
        },
      } as Config);
    }
  };

  if (!hasAdminPermission(currentUser)) {
    return (
      <div className={styles.loadingContainer}>
        <Spinner size="large" />
      </div>
    );
  }

  const statCards = [
    { icon: '📝', label: '文章总数', value: stats?.total_posts ?? 0, color: 'brand' },
    { icon: '👥', label: '用户总数', value: stats?.total_users ?? 0, color: 'success' },
    { icon: '💬', label: '评论总数', value: stats?.total_comments ?? 0, color: 'warning' },
    { icon: '📁', label: '文件总数', value: stats?.total_files ?? 0, color: 'important' },
    { icon: '👁', label: '总访问量', value: stats?.total_visits ?? 0, color: 'severe' },
    { icon: '📅', label: '今日访问', value: stats?.today_visits ?? 0, color: 'success' },
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
              onTabSelect={(_, data) =>
                setActiveTab(data.value as 'dashboard' | 'posts' | 'users' | 'settings')
              }
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
              <div className={styles.userInfo}>{currentUser?.username}</div>
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
                <Button appearance="transparent" size="small" onClick={() => setError('')}>
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
                    <Card key={index} className={styles.statCard}>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                        <span className={styles.statIcon}>{stat.icon}</span>
                        <div>
                          <Body1 className={styles.statValue}>{stat.value}</Body1>
                          <Caption1 className={styles.statLabel}>{stat.label}</Caption1>
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
                      <Body1 className={styles.metaText}>暂无文章</Body1>
                    </div>
                  ) : (
                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                      {posts.map((post) => (
                        <div key={post.id} className={styles.listItem}>
                          <div className={styles.listItemContent}>
                            <Link to={`/post/${post.id}`} className={styles.listItemTitle}>
                              {post.title}
                            </Link>
                            <div className={styles.listItemMeta}>
                              <Caption1 className={styles.metaText}>
                                {formatDate(post.created_at)}
                              </Caption1>
                              <Caption1 className={styles.metaText}>{post.views} 次阅读</Caption1>

                              {post.indexnow_submitted &&
                                post.indexnow_last_status === 'success' && (
                                  <Tooltip
                                    content={`提交于: ${post.indexnow_submitted_at ? formatDate(post.indexnow_submitted_at) : '未知'}`}
                                    relationship="label"
                                  >
                                    <Badge size="small" color="success" appearance="outline">
                                      ✓ 已通知搜索引擎
                                    </Badge>
                                  </Tooltip>
                                )}

                              {post.indexnow_last_status === 'pending' && (
                                <Badge size="small" color="brand" appearance="outline">
                                  正在提交...
                                </Badge>
                              )}

                              {post.indexnow_last_status === 'failed' && (
                                <Tooltip
                                  content={post.indexnow_last_error ?? '未知错误'}
                                  relationship="label"
                                >
                                  <Badge size="small" color="danger" appearance="outline">
                                    ✗ 提交失败
                                  </Badge>
                                </Tooltip>
                              )}

                              {!post.indexnow_submitted && post.published_at && (
                                <Badge size="small" color="warning" appearance="outline">
                                  未通知搜索引擎
                                </Badge>
                              )}
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
                              onClick={() => {
                                void handleTogglePublish(post);
                              }}
                            >
                              {post.published_at ? '取消发布' : '发布'}
                            </Button>

                            {post.published_at && (
                              <Button
                                appearance="transparent"
                                icon={<SendRegular />}
                                size="small"
                                onClick={() => {
                                  void handleSubmitToIndexNow(post.id);
                                }}
                                disabled={submittingIndexNow.has(post.id)}
                              >
                                {submittingIndexNow.has(post.id) ? '提交中...' : '通知搜索引擎'}
                              </Button>
                            )}

                            <Button
                              appearance="transparent"
                              icon={<DeleteRegular />}
                              size="small"
                              onClick={() => {
                                void handleDeletePost(post.id);
                              }}
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
                      <Body1 className={styles.metaText}>暂无用户</Body1>
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
                            color={
                              (user.permissions & Permission.USER_MANAGE) !== 0
                                ? 'brand'
                                : 'success'
                            }
                          >
                            {(user.permissions & Permission.USER_MANAGE) !== 0
                              ? '管理员'
                              : '普通用户'}
                          </Badge>

                          <Button
                            appearance="transparent"
                            icon={<DeleteRegular />}
                            size="small"
                            disabled={user.id === currentUser?.id}
                            onClick={() => {
                              void handleDeleteUser(user.id);
                            }}
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
            {!loading && activeTab === 'settings' && config && (
              <div>
                <Title2 style={{ marginBottom: '24px' }}>系统设置</Title2>
                <Card style={{ borderRadius: tokens.borderRadiusLarge, padding: '32px' }}>
                  {/* 站点设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>站点设置</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          允许注册
                          {config.site.allow_registration_env_override && (
                            <EnvOverrideBadge message="ALLOW_REGISTRATION" />
                          )}
                        </Body1>
                        <Switch
                          checked={config.site.allow_registration}
                          onChange={(_, data) =>
                            handleConfigChange('site', 'allow_registration', data.checked)
                          }
                          label={config.site.allow_registration ? '开启' : '关闭'}
                          disabled={!!config.site.allow_registration_env_override}
                        />
                      </div>
                    </div>
                  </div>

                  <Divider style={{ margin: '24px 0' }} />

                  {/* 服务器设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>服务器设置</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          主机地址
                          {config.server.host_env_override && <EnvOverrideBadge message="HOST" />}
                        </Body1>
                        <Input
                          value={config.server.host}
                          onChange={(e) => handleConfigChange('server', 'host', e.target.value)}
                          disabled={!!config.server.host_env_override}
                        />
                      </div>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          端口
                          {config.server.port_env_override && <EnvOverrideBadge message="PORT" />}
                        </Body1>
                        <Input
                          type="number"
                          value={config.server.port.toString()}
                          onChange={(e) =>
                            handleConfigChange('server', 'port', parseInt(e.target.value, 10))
                          }
                          disabled={!!config.server.port_env_override}
                        />
                      </div>
                    </div>
                  </div>

                  <Divider style={{ margin: '24px 0' }} />

                  {/* 数据库设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>数据库设置</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          连接 URL
                          {config.database.url_env_override && (
                            <EnvOverrideBadge message="DATABASE_URL" />
                          )}
                        </Body1>
                        <Input
                          value={config.database.url}
                          onChange={(e) => handleConfigChange('database', 'url', e.target.value)}
                          type="password"
                          disabled={!!config.database.url_env_override}
                        />
                      </div>
                    </div>
                  </div>

                  <Divider style={{ margin: '24px 0' }} />

                  {/* 认证设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>认证设置</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          JWT 密钥
                          {config.auth.jwt_secret_env_override && (
                            <EnvOverrideBadge message="JWT_SECRET" />
                          )}
                        </Body1>
                        <Input
                          value={config.auth.jwt_secret}
                          onChange={(e) => handleConfigChange('auth', 'jwt_secret', e.target.value)}
                          type="password"
                          disabled={!!config.auth.jwt_secret_env_override}
                        />
                      </div>
                    </div>
                  </div>

                  <Divider style={{ margin: '24px 0' }} />

                  {/* 存储设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>存储设置</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          上传目录
                          {config.storage.upload_dir_env_override && (
                            <EnvOverrideBadge message="UPLOAD_DIR" />
                          )}
                        </Body1>
                        <Input
                          value={config.storage.upload_dir}
                          onChange={(e) =>
                            handleConfigChange('storage', 'upload_dir', e.target.value)
                          }
                          disabled={!!config.storage.upload_dir_env_override}
                        />
                      </div>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          缓存目录
                          {config.storage.cache_dir_env_override && (
                            <EnvOverrideBadge message="CACHE_DIR" />
                          )}
                        </Body1>
                        <Input
                          value={config.storage.cache_dir}
                          onChange={(e) =>
                            handleConfigChange('storage', 'cache_dir', e.target.value)
                          }
                          disabled={!!config.storage.cache_dir_env_override}
                        />
                      </div>
                    </div>
                  </div>

                  <Divider style={{ margin: '24px 0' }} />

                  {/* GitHub 设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>GitHub 集成</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          Client ID
                          {config.github.client_id_env_override && (
                            <EnvOverrideBadge message="GITHUB_CLIENT_ID" />
                          )}
                        </Body1>
                        <Input
                          value={config.github.client_id}
                          onChange={(e) =>
                            handleConfigChange('github', 'client_id', e.target.value)
                          }
                          disabled={!!config.github.client_id_env_override}
                        />
                      </div>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          Client Secret
                          {config.github.client_secret_env_override && (
                            <EnvOverrideBadge message="GITHUB_CLIENT_SECRET" />
                          )}
                        </Body1>
                        <Input
                          value={config.github.client_secret}
                          onChange={(e) =>
                            handleConfigChange('github', 'client_secret', e.target.value)
                          }
                          type="password"
                          disabled={!!config.github.client_secret_env_override}
                        />
                      </div>
                    </div>
                  </div>

                  <Divider style={{ margin: '24px 0' }} />

                  {/* IndexNow 设置 */}
                  <div className={styles.settingsSection}>
                    <Title3 style={{ marginBottom: '16px' }}>IndexNow 搜索引擎通知</Title3>
                    <div className={styles.settingsGroup}>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>启用 IndexNow</Body1>
                        <Switch
                          checked={config.indexnow.enabled}
                          onChange={(_, data) =>
                            handleConfigChange('indexnow', 'enabled', data.checked)
                          }
                          label={config.indexnow.enabled ? '开启' : '关闭'}
                        />
                      </div>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>
                          API Key
                          {config.indexnow.api_key_env_override && (
                            <EnvOverrideBadge message="INDEXNOW_API_KEY" />
                          )}
                        </Body1>
                        <Input
                          value={config.indexnow.api_key}
                          onChange={(e) =>
                            handleConfigChange('indexnow', 'api_key', e.target.value)
                          }
                          type="password"
                          disabled={!!config.indexnow.api_key_env_override}
                          placeholder="输入你的 IndexNow API Key"
                        />
                      </div>
                      <div className={styles.settingsRow}>
                        <Body1 style={{ fontWeight: '600' }}>API 端点</Body1>
                        <Input
                          value={config.indexnow.endpoint}
                          onChange={(e) =>
                            handleConfigChange('indexnow', 'endpoint', e.target.value)
                          }
                          placeholder="https://www.indexnow.org/indexnow"
                        />
                      </div>
                    </div>
                  </div>

                  <div className={styles.settingsActions}>
                    <Button appearance="primary" onClick={() => void handleSaveConfig()}>
                      保存更改
                    </Button>
                  </div>
                </Card>
              </div>
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
