/**
 * 文章详情页 - Markdown 渲染和代码高亮
 */

import { useEffect, useState } from 'react';
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
} from '@fluentui/react-components';
import {
  ArrowLeftRegular,
  EditRegular,
  CalendarRegular,
  EyeRegular,
  ClockRegular,
  CertificateRegular,
  TextBulletListLtrRegular,
} from '@fluentui/react-icons';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import { postsApi, authApi, statsApi } from '../api';
import type { Post, Comment } from '../types';
import { extractTOC, calculateReadingTime, slugify, type TOCItem } from '../utils/markdown';
import 'highlight.js/styles/github-dark.css';

const useStyles = makeStyles({
  container: {
    maxWidth: '800px',
    margin: '0 auto',
    padding: '32px 0',
  },
  progressBar: {
    position: 'fixed',
    top: 0,
    left: 0,
    height: '4px',
    backgroundColor: tokens.colorBrandBackground,
    zIndex: 1000,
    transition: 'width 0.2s',
  },
  tocContainer: {
    position: 'fixed',
    top: '120px',
    left: 'calc(50% + 440px)', // 800px / 2 + 40px gap
    width: '240px',
    maxHeight: 'calc(100vh - 160px)',
    overflowY: 'auto',
    '@media (max-width: 1400px)': {
      display: 'none',
    },
  },
  tocHeader: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    fontWeight: '600',
    marginBottom: '12px',
    color: tokens.colorNeutralForeground2,
  },
  tocList: {
    listStyle: 'none',
    padding: 0,
    margin: 0,
  },
  tocItem: {
    marginBottom: '8px',
    fontSize: '13px',
    lineHeight: '1.4',
    color: tokens.colorNeutralForeground3,
    textDecoration: 'none',
    display: 'block',
    transition: 'color 0.2s',
    ':hover': {
      color: tokens.colorBrandForeground1,
    },
  },
  activeTocItem: {
    color: tokens.colorBrandForeground1,
    fontWeight: '600',
  },
  licenseBlock: {
    marginTop: '48px',
    padding: '16px',
    backgroundColor: tokens.colorNeutralBackground2,
    borderRadius: '8px',
    fontSize: '14px',
    color: tokens.colorNeutralForeground2,
    border: `1px solid ${tokens.colorNeutralStroke2}`,
  },
  licenseTitle: {
    fontWeight: '600',
    marginBottom: '4px',
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
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
  mdH1: { fontSize: '32px', fontWeight: '600', marginTop: '32px', marginBottom: '16px' },
  mdH2: { fontSize: '28px', fontWeight: '600', marginTop: '28px', marginBottom: '14px' },
  mdH3: { fontSize: '24px', fontWeight: '600', marginTop: '24px', marginBottom: '12px' },
  mdP: { marginBottom: '16px' },
  mdInlineCode: {
    backgroundColor: tokens.colorNeutralBackground1Hover,
    padding: '2px 6px',
    borderRadius: '4px',
    fontSize: '14px',
    fontFamily: 'monospace',
  },
  mdBlockCode: {
    display: 'block',
    backgroundColor: '#0d1117',
    color: '#c9d1d9',
    padding: '16px',
    borderRadius: '8px',
    overflow: 'auto',
    fontSize: '14px',
    fontFamily: 'monospace',
    marginBottom: '16px',
  },
  mdPre: {
    backgroundColor: '#0d1117',
    padding: '16px',
    borderRadius: '8px',
    overflow: 'auto',
    marginBottom: '16px',
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
});

export function PostDetailPage() {
  const styles = useStyles();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [post, setPost] = useState<Post | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState(true);
  const [commentContent, setCommentContent] = useState('');
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [toc, setToc] = useState<TOCItem[]>([]);
  const [readingTime, setReadingTime] = useState(0);
  const [readingProgress, setReadingProgress] = useState(0);
  const [activeHeaderId, setActiveHeaderId] = useState<string>('');

  useEffect(() => {
    const handleScroll = () => {
      // Calculate progress
      const totalHeight = document.documentElement.scrollHeight - document.documentElement.clientHeight;
      const progress = (window.scrollY / totalHeight) * 100;
      setReadingProgress(progress);

      // Determine active header
      const headings = Array.from(document.querySelectorAll('h1, h2, h3'));
      for (const heading of headings) {
        const rect = heading.getBoundingClientRect();
        if (rect.top >= 0 && rect.top <= 150) {
          setActiveHeaderId(heading.id);
          break;
        }
      }
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  useEffect(() => {
    setIsAuthenticated(authApi.isAuthenticated());
    if (id) {
      fetchPost();
      fetchComments();
    }
  }, [id]);

  const fetchPost = async () => {
    if (!id) return;

    try {
      setLoading(true);
      const response = await postsApi.getPost(id);
      setPost(response.data);
      setToc(extractTOC(response.data.content));
      setReadingTime(calculateReadingTime(response.data.content));

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
  };

  const fetchComments = async () => {
    if (!id) return;

    try {
      const response = await postsApi.getPostComments(id);
      setComments(response.data);
    } catch (error) {
      console.error('Failed to fetch comments:', error);
    }
  };

  const handleCommentSubmit = async () => {
    if (!id || !commentContent.trim()) return;

    if (!isAuthenticated) {
      alert('请先登录');
      navigate('/login');
      return;
    }

    try {
      await postsApi.createPostComment(id, {
        content: commentContent,
      });
      setCommentContent('');
      fetchComments();
    } catch (error) {
      console.error('Failed to create comment:', error);
      alert('评论失败，请重试');
    }
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

  const getTextFromChildren = (children: any): string => {
    if (typeof children === 'string') return children;
    if (Array.isArray(children)) return children.map(getTextFromChildren).join('');
    if (children?.props?.children) return getTextFromChildren(children.props.children);
    return '';
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
    <div className={styles.container}>
      {/* 进度条 */}
      <div className={styles.progressBar} style={{ width: `${readingProgress}%` }} />

      {/* 目录 (桌面端) */}
      {toc.length > 0 && (
        <div className={styles.tocContainer}>
          <div className={styles.tocHeader}>
            <TextBulletListLtrRegular />
            <span>目录</span>
          </div>
          <ul className={styles.tocList}>
            {toc.map((item) => (
              <li key={item.id} style={{ paddingLeft: `${(item.level - 1) * 12}px` }}>
                <a
                  href={`#${item.id}`}
                  className={`${styles.tocItem} ${
                    activeHeaderId === item.id ? styles.activeTocItem : ''
                  }`}
                  onClick={(e) => {
                    e.preventDefault();
                    document.getElementById(item.id)?.scrollIntoView({ behavior: 'smooth' });
                    setActiveHeaderId(item.id);
                  }}
                >
                  {item.text}
                </a>
              </li>
            ))}
          </ul>
        </div>
      )}

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
                <ClockRegular fontSize={14} />
                <Caption1>预计阅读 {readingTime} 分钟</Caption1>
              </div>
              <div className={styles.metaItem}>
                <CertificateRegular fontSize={14} />
                <Caption1>CC BY-SA 4.0</Caption1>
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
          }
        />

        <Divider />

        {/* Markdown 内容 */}
        <div className={styles.content}>
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            rehypePlugins={[rehypeHighlight]}
            components={{
              h1: ({ node, ...props }) => {
                const text = getTextFromChildren(props.children);
                return <h1 id={slugify(text)} className={styles.mdH1} {...props} />;
              },
              h2: ({ node, ...props }) => {
                const text = getTextFromChildren(props.children);
                return <h2 id={slugify(text)} className={styles.mdH2} {...props} />;
              },
              h3: ({ node, ...props }) => {
                const text = getTextFromChildren(props.children);
                return <h3 id={slugify(text)} className={styles.mdH3} {...props} />;
              },
              p: ({ node, ...props }) => <p className={styles.mdP} {...props} />,
              code: ({ node, inline, ...props }: any) =>
                inline ? (
                  <code className={styles.mdInlineCode} {...props} />
                ) : (
                  <code className={styles.mdBlockCode} {...props} />
                ),
              pre: ({ node, ...props }) => <pre className={styles.mdPre} {...props} />,
              a: ({ node, ...props }) => (
                <a className={styles.mdLink} target="_blank" rel="noopener noreferrer" {...props} />
              ),
              ul: ({ node, ...props }) => <ul className={styles.mdList} {...props} />,
              ol: ({ node, ...props }) => <ol className={styles.mdList} {...props} />,
              blockquote: ({ node, ...props }) => (
                <blockquote className={styles.mdBlockquote} {...props} />
              ),
            }}
          >
            {post.content}
          </ReactMarkdown>

          {/* 授权许可 */}
          <div className={styles.licenseBlock}>
            <div className={styles.licenseTitle}>
              <CertificateRegular />
              <span>许可协议</span>
            </div>
            <div>
              本文采用{' '}
              <a
                href="https://creativecommons.org/licenses/by-sa/4.0/"
                target="_blank"
                rel="noreferrer"
                style={{ color: tokens.colorBrandForeground1, textDecoration: 'underline' }}
              >
                CC BY-SA 4.0 (GPL Compatible)
              </a>{' '}
              许可协议。转载请注明出处。
            </div>
          </div>
        </div>
      </Card>

      {/* 评论区 */}
      <Card className={styles.postCard}>
        <CardHeader
          header={<Body1 style={{ fontSize: '20px', fontWeight: '600' }}>评论</Body1>}
          description={<Caption1>{comments.length} 条评论</Caption1>}
        />

        {/* 评论输入 */}
        {isAuthenticated ? (
          <div className={styles.commentInputContainer}>
            <Textarea
              placeholder="写下你的评论..."
              value={commentContent}
              onChange={(_, data) => setCommentContent(data.value)}
              className={styles.commentTextarea}
            />
            <Button
              appearance="primary"
              onClick={handleCommentSubmit}
              disabled={!commentContent.trim()}
            >
              发表评论
            </Button>
          </div>
        ) : (
          <div className={styles.loginPrompt}>
            <Body1 style={{ marginBottom: '8px' }}>登录后发表评论</Body1>
            <Button appearance="primary" onClick={() => navigate('/login')}>
              登录
            </Button>
            <Button appearance="transparent" onClick={() => navigate('/register')}>
              注册
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
                  <strong className={styles.commentUser}>
                    {comment.github_username || '用户'}
                  </strong>
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
  );
}
