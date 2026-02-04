/**
 * 文章详情页 - Markdown 渲染和代码高亮
 */

import { useEffect, useState, useMemo, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Card,
  CardHeader,
  Body1,
  Title1,
  Caption1,
  Button,
  Spinner,
  Divider,
  Textarea,
  makeStyles,
  tokens,
  shorthands,
} from '@fluentui/react-components';
import {
  ArrowLeftRegular,
  EditRegular,
  CalendarRegular,
  EyeRegular,
  TimerRegular,
  BookRegular,
} from '@fluentui/react-icons';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import mermaid from 'mermaid';
import { postsApi, authApi, statsApi, bingApi } from '../api';
import { useToast } from '../components/ui/Toast';
import { getDominantColor } from '../utils/color';
import type { Post, Comment } from '../types';
import 'highlight.js/styles/github-dark.css';

interface GithubUserData {
  username: string;
  avatar_url: string;
}

interface StoredGithubUser {
  user: GithubUserData;
  timestamp: number;
  token: string;
}

interface JwtPayload {
  username: string;
  avatar_url: string;
  exp: number;
}

// Initialize mermaid
mermaid.initialize({
  startOnLoad: false,
  theme: 'dark',
  securityLevel: 'loose',
});

const useStyles = makeStyles({
  wrapper: {
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '32px 16px',
    display: 'flex',
    gap: '32px',
    alignItems: 'flex-start',
  },
  mainContent: {
    flex: 1,
    minWidth: 0, // Fix for flex child overflow
    maxWidth: '800px', // Maintain readable line length
    margin: '0 auto', // Center if sidebar is hidden
  },
  sidebar: {
    width: '260px',
    position: 'sticky',
    top: '24px',
    display: 'none',
    '@media (min-width: 1100px)': {
      display: 'block',
    },
  },
  tocCard: {
    maxHeight: 'calc(100vh - 48px)',
    overflowY: 'auto',
  },
  tocTitle: {
    fontSize: '20px',
    fontWeight: '700',
    color: tokens.colorNeutralForeground1,
    marginBottom: '16px',
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
  },
  tocList: {
    listStyleType: 'none',
    padding: 0,
    margin: 0,
  },
  tocItem: {
    marginBottom: '10px',
    fontSize: '15px',
    fontWeight: '600',
    cursor: 'pointer',
    color: tokens.colorNeutralForeground2,
    ':hover': {
      color: tokens.colorBrandForeground1,
    },
  },
  tocLink: {
    textDecoration: 'none',
    color: 'inherit',
    display: 'block',
    ...shorthands.overflow('hidden'),
    whiteSpace: 'nowrap',
    textOverflow: 'ellipsis',
  },
  backButton: {
    marginBottom: '16px',
  },
  postCard: {
    marginBottom: '32px',
    padding: '24px',
  },
  title: {
    fontSize: '36px',
    fontWeight: '600',
  },
  metaContainer: {
    display: 'flex',
    flexWrap: 'wrap',
    gap: '16px',
    marginTop: '16px',
  },
  metaItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
  },
  editContainer: {
    marginLeft: 'auto',
    display: 'flex',
    gap: '8px',
  },
  license: {
    marginTop: '8px',
    padding: '8px 12px',
    backgroundColor: tokens.colorNeutralBackground2,
    borderRadius: '4px',
    fontSize: '12px',
    color: tokens.colorNeutralForeground2,
    display: 'inline-block',
  },
  content: {
    padding: '24px 0',
    lineHeight: '1.8',
    fontSize: '16px',
  },
  commentInputContainer: {
    marginBottom: '24px',
  },
  commentTextarea: {
    marginBottom: '12px',
    minHeight: '100px',
    width: '100%',
  },
  loginPrompt: {
    marginBottom: '24px',
    padding: '16px',
    backgroundColor: tokens.colorNeutralBackground1,
  },
  commentList: {
    marginTop: '16px',
  },
  emptyComments: {
    textAlign: 'center',
    padding: '32px',
    color: tokens.colorNeutralForeground3,
  },
  commentItem: {
    padding: '16px 0',
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
  },
  commentHeader: {
    marginBottom: '8px',
  },
  commentUser: {
    fontWeight: '600',
  },
  commentDate: {
    marginLeft: '8px',
    color: tokens.colorNeutralForeground3,
  },
  commentContent: {
    lineHeight: '1.6',
  },
  loadingContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '48px',
  },
  errorContainer: {
    textAlign: 'center',
    padding: '48px',
  },
  // Markdown Styles
  mdH1: {
    fontSize: '32px',
    fontWeight: '600',
    marginTop: '32px',
    marginBottom: '16px',
    scrollMarginTop: '80px',
  },
  mdH2: {
    fontSize: '28px',
    fontWeight: '600',
    marginTop: '28px',
    marginBottom: '14px',
    scrollMarginTop: '80px',
  },
  mdH3: {
    fontSize: '24px',
    fontWeight: '600',
    marginTop: '24px',
    marginBottom: '12px',
    scrollMarginTop: '80px',
  },
  mdP: { marginBottom: '16px' },
  mdInlineCode: {
    backgroundColor: tokens.colorNeutralBackground1Hover,
    padding: '2px 6px',
    borderRadius: '4px',
    fontSize: '14px',
    fontFamily: 'monospace',
  },
  mdBlockCode: {
    // Legacy style, now handled by mdPre & > code
  },
  mdPre: {
    backgroundColor: 'transparent',
    padding: 0,
    borderRadius: 0,
    overflow: 'visible',
    marginBottom: '16px',
    '& > code': {
      display: 'block',
      backgroundColor: '#0d1117',
      color: '#c9d1d9',
      padding: '16px',
      borderRadius: '8px',
      overflow: 'auto',
      fontSize: '14px',
      fontFamily: 'monospace',
    },
  },
  mdLink: {
    color: tokens.colorBrandForeground1,
    textDecoration: 'underline',
  },
  mdList: {
    marginBottom: '16px',
    paddingLeft: '24px',
  },
  mdBlockquote: {
    borderLeft: `4px solid ${tokens.colorBrandStroke1}`,
    paddingLeft: '16px',
    marginLeft: '0',
    fontStyle: 'italic',
    color: tokens.colorNeutralForeground2,
    marginBottom: '16px',
  },
  mermaidContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '16px',
    backgroundColor: tokens.colorNeutralBackground1,
    borderRadius: '8px',
    marginBottom: '16px',
    overflowX: 'auto',
    border: `1px solid ${tokens.colorNeutralStroke1}`,
  },
});

const MermaidDiagram = ({ chart, accentColor }: { chart: string; accentColor?: string }) => {
  const styles = useStyles();
  const [svg, setSvg] = useState<string>('');

  useEffect(() => {
    const renderChart = async () => {
      if (!chart) return;
      try {
        const id = `mermaid-${Math.random().toString(36).substr(2, 9)}`;

        let chartCode = chart;
        if (accentColor) {
          const config = {
            theme: 'base',
            themeVariables: {
              primaryColor: accentColor,
              lineColor: accentColor,
              primaryBorderColor: accentColor,
            },
          };
          chartCode = `%%{init: ${JSON.stringify(config)} }%%\n${chart}`;
        }

        const { svg } = await mermaid.render(id, chartCode);
        setSvg(svg);
      } catch (error) {
        console.error('Mermaid render error:', error);
        setSvg('<div style="color:red">Diagram render failed</div>');
      }
    };
    void renderChart();
  }, [chart, accentColor]);

  return <div className={styles.mermaidContainer} dangerouslySetInnerHTML={{ __html: svg }} />;
};

export function PostDetailPage() {
  const styles = useStyles();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const toast = useToast();
  const [post, setPost] = useState<Post | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState(true);
  const [commentContent, setCommentContent] = useState('');
  const [accentColor, setAccentColor] = useState<string>('');
  const [githubUser, setGithubUser] = useState<{ username: string; avatar_url: string } | null>(
    null
  );

  useEffect(() => {
    const fetchBingColor = async () => {
      try {
        const response = await bingApi.getDailyImage();
        if (response.data?.url) {
          const color = await getDominantColor(response.data.url);
          setAccentColor(color);
        }
      } catch (error) {
        console.error('Failed to fetch Bing color:', error);
      }
    };
    void fetchBingColor();
  }, []);

  useEffect(() => {
    setIsAuthenticated(authApi.isAuthenticated());
  }, []);

  // Restore GitHub user from localStorage on mount
  useEffect(() => {
    const savedGithubUser = localStorage.getItem('github_user');
    if (savedGithubUser) {
      const data = JSON.parse(savedGithubUser) as StoredGithubUser;
      if (Date.now() - data.timestamp < 6 * 60 * 60 * 1000) {
        setGithubUser(data.user);
      } else {
        localStorage.removeItem('github_user');
      }
    }
  }, []);

  // Handle GitHub OAuth callback
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const token = params.get('token');

    if (token) {
      try {
        const payload = JSON.parse(atob(token.split('.')[1])) as JwtPayload;
        const user: GithubUserData = {
          username: payload.username,
          avatar_url: payload.avatar_url,
        };

        setGithubUser(user);
        localStorage.setItem(
          'github_user',
          JSON.stringify({
            user,
            timestamp: Date.now(),
            token,
          })
        );

        window.history.replaceState({}, '', window.location.pathname);
      } catch (error) {
        console.error('Failed to parse GitHub token:', error);
      }
    }
  }, []);

  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const fetchPost = useCallback(async () => {
    if (!id) return;

    try {
      setLoading(true);
      const response = await postsApi.getPost(id);
      setPost(response.data);

      // 记录阅读量
      await statsApi.recordPostView(id);

      // 重新获取文章数据以显示最新阅读量
      const updatedResponse = await postsApi.getPost(id);
      setPost(updatedResponse.data);
    } catch (error) {
      console.error('Failed to fetch post:', error);
    } finally {
      setLoading(false);
    }
  }, [id]);

  const fetchComments = useCallback(async () => {
    if (!id) return;

    try {
      const response = await postsApi.getPostComments(id);
      setComments(response.data);
    } catch (error) {
      console.error('Failed to fetch comments:', error);
    }
  }, [id]);

  useEffect(() => {
    setIsAuthenticated(authApi.isAuthenticated());
    if (id) {
      void fetchPost();
      void fetchComments();
    }
  }, [id, fetchPost, fetchComments]);

  const handleCommentSubmit = async () => {
    if (!id || !commentContent.trim()) return;

    if (isAuthenticated) {
      try {
        await postsApi.createPostComment(id, {
          content: commentContent,
        });
        setCommentContent('');
        void fetchComments();
      } catch (error) {
        console.error('Failed to create comment:', error);
        toast.showError('评论失败，请重试');
      }
    } else {
      toast.showWarning('请先登录');
    }
  };

  const handleGitHubLogin = async () => {
    try {
      const response = await postsApi.getGitHubAuthUrl();
      sessionStorage.setItem('github_oauth_return', window.location.pathname);
      window.location.href = response.auth_url;
    } catch (error) {
      console.error('Failed to get GitHub auth URL:', error);
      toast.showError('GitHub登录失败，请重试');
    }
  };

  const handleGitHubLogout = () => {
    setGithubUser(null);
    localStorage.removeItem('github_user');
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  // Helper to slugify text for IDs
  const slugify = (text: string) => {
    return text
      .toLowerCase()
      .replace(/[^\w\u4e00-\u9fa5]+/g, '-') // Support Chinese characters
      .replace(/^-+|-+$/g, '');
  };

  // Extract headings for TOC
  const headings = useMemo(() => {
    if (!post?.content) return [];
    const lines = post.content.split('\n');
    const extracted = [];
    for (const line of lines) {
      const match = line.match(/^(#{1,3})\s+(.+)$/);
      if (match) {
        extracted.push({
          level: match[1].length,
          text: match[2].trim(),
          id: slugify(match[2].trim()),
        });
      }
    }
    return extracted;
  }, [post?.content]);

  // Calculate reading time
  const readingTime = useMemo(() => {
    if (!post?.content) return 0;
    const words = post.content.length; // Simple char count for Chinese context
    return Math.ceil(words / 400); // Assume 400 chars per minute
  }, [post?.content]);

  const scrollToHeading = (id: string) => {
    const element = document.getElementById(id);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' });
    }
  };

  if (loading) {
    return (
      <div className={styles.loadingContainer}>
        <Spinner size="large" />
      </div>
    );
  }

  if (!post) {
    return (
      <div className={styles.errorContainer}>
        <Body1>文章不存在</Body1>
        <Button appearance="primary" onClick={() => navigate('/')}>
          返回首页
        </Button>
      </div>
    );
  }

  return (
    <div className={styles.wrapper}>
      {/* Main Content Area */}
      <div className={styles.mainContent}>
        {/* 返回按钮 */}
        <Button
          appearance="transparent"
          icon={<ArrowLeftRegular />}
          onClick={() => navigate(-1)}
          className={styles.backButton}
        >
          返回
        </Button>

        {/* 文章卡片 */}
        <Card className={styles.postCard}>
          <CardHeader
            header={<Title1 className={styles.title}>{post.title}</Title1>}
            description={
              <div>
                <div className={styles.metaContainer}>
                  <div className={styles.metaItem}>
                    <CalendarRegular fontSize={14} />
                    <Caption1>{formatDate(post.created_at)}</Caption1>
                  </div>
                  <div className={styles.metaItem}>
                    <EyeRegular fontSize={14} />
                    <Caption1>{post.views} 次阅读</Caption1>
                  </div>
                  <div className={styles.metaItem}>
                    <TimerRegular fontSize={14} />
                    <Caption1>阅读时长 {readingTime} 分钟</Caption1>
                  </div>
                  {isAuthenticated && (
                    <div className={styles.editContainer}>
                      <Button
                        size="small"
                        appearance="transparent"
                        icon={<EditRegular />}
                        onClick={() => navigate(`/admin/posts/edit/${post.id}`)}
                      >
                        编辑
                      </Button>
                    </div>
                  )}
                </div>
                <div className={styles.license}>许可证：GPL v3.0 (GNU General Public License)</div>
              </div>
            }
          />

          <Divider />

          {/* Markdown 内容 */}
          <div className={styles.content}>
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              rehypePlugins={[rehypeHighlight]}
              components={{
                h1: ({ ...props }) => {
                  const id = slugify(String(props.children));
                  return <h1 id={id} className={styles.mdH1} {...props} />;
                },
                h2: ({ ...props }) => {
                  const id = slugify(String(props.children));
                  return <h2 id={id} className={styles.mdH2} {...props} />;
                },
                h3: ({ ...props }) => {
                  const id = slugify(String(props.children));
                  return <h3 id={id} className={styles.mdH3} {...props} />;
                },
                p: ({ ...props }) => <p className={styles.mdP} {...props} />,
                code: ({
                  className,
                  children,
                  ...props
                }: React.HTMLAttributes<HTMLElement> & { inline?: boolean }) => {
                  const match = /language-(\w+)/.exec(className ?? '');
                  const isMermaid = match && match[1] === 'mermaid';

                  if (isMermaid) {
                    return (
                      <MermaidDiagram
                        chart={String(children).replace(/\n$/, '')}
                        accentColor={accentColor}
                      />
                    );
                  }

                  return (
                    <code className={`${styles.mdInlineCode} ${className ?? ''}`} {...props}>
                      {children}
                    </code>
                  );
                },
                pre: ({ ...props }) => <pre className={styles.mdPre} {...props} />,
                a: ({ ...props }) => (
                  <a
                    className={styles.mdLink}
                    target="_blank"
                    rel="noopener noreferrer"
                    {...props}
                  />
                ),
                ul: ({ ...props }) => <ul className={styles.mdList} {...props} />,
                ol: ({ ...props }) => <ol className={styles.mdList} {...props} />,
                blockquote: ({ ...props }) => (
                  <blockquote className={styles.mdBlockquote} {...props} />
                ),
              }}
            >
              {post.content}
            </ReactMarkdown>
          </div>
        </Card>

        {/* 评论区 */}
        <Card className={styles.postCard}>
          <CardHeader
            header={<Body1 style={{ fontSize: '20px', fontWeight: '600' }}>评论</Body1>}
            description={<Caption1>{comments.length} 条评论</Caption1>}
          />

          {/* 评论输入 */}
          {isAuthenticated || githubUser ? (
            <div className={styles.commentInputContainer}>
              {githubUser && (
                <div
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    marginBottom: '12px',
                  }}
                >
                  <img
                    src={githubUser.avatar_url}
                    alt={githubUser.username}
                    style={{ width: '32px', height: '32px', borderRadius: '50%' }}
                  />
                  <Body1>
                    <strong>{githubUser.username}</strong>
                  </Body1>
                  <Button appearance="transparent" size="small" onClick={handleGitHubLogout}>
                    退出
                  </Button>
                </div>
              )}
              <Textarea
                placeholder="写下你的评论..."
                value={commentContent}
                onChange={(_, data) => setCommentContent(data.value)}
                className={styles.commentTextarea}
              />
              <Button
                appearance="primary"
                onClick={() => {
                  void handleCommentSubmit();
                }}
                disabled={!commentContent.trim()}
              >
                发表评论
              </Button>
            </div>
          ) : (
            <div className={styles.loginPrompt}>
              <Body1 style={{ marginBottom: '8px' }}>登录后发表评论</Body1>
              <Button
                appearance="primary"
                onClick={() => {
                  void handleGitHubLogin();
                }}
                style={{ marginRight: '8px' }}
              >
                使用 GitHub 登录
              </Button>
              <Button appearance="transparent" onClick={() => navigate('/login')}>
                账号密码登录
              </Button>
            </div>
          )}

          <Divider />

          {/* 评论列表 */}
          <div className={styles.commentList}>
            {comments.length === 0 ? (
              <div className={styles.emptyComments}>
                <Caption1>暂无评论，快来发表第一条评论吧！</Caption1>
              </div>
            ) : (
              comments.map((comment) => (
                <div key={comment.id} className={styles.commentItem}>
                  <div className={styles.commentHeader}>
                    <strong className={styles.commentUser}>{comment.username || '用户'}</strong>
                    <Caption1 className={styles.commentDate}>
                      {formatDate(comment.created_at)}
                    </Caption1>
                  </div>
                  <Body1 className={styles.commentContent}>{comment.content}</Body1>
                </div>
              ))
            )}
          </div>
        </Card>
      </div>

      {/* Sidebar Table of Contents - Only visible on desktop */}
      <aside className={styles.sidebar}>
        <div className={styles.tocCard}>
          <div className={styles.tocTitle}>
            <BookRegular />
            <span>目录</span>
          </div>
          {headings.length > 0 ? (
            <ul className={styles.tocList}>
              {headings.map((heading, index) => (
                <li
                  key={`${heading.id}-${index}`}
                  className={styles.tocItem}
                  style={{ paddingLeft: `${(heading.level - 1) * 12}px` }}
                >
                  <div
                    className={styles.tocLink}
                    onClick={() => scrollToHeading(heading.id)}
                    title={heading.text}
                  >
                    {heading.text}
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            <Caption1>本文无目录</Caption1>
          )}
        </div>
      </aside>
    </div>
  );
}
