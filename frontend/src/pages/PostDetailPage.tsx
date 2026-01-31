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
} from '@fluentui/react-components';
import {
  ArrowLeftRegular,
  EditRegular,
  CalendarRegular,
  EyeRegular,
} from '@fluentui/react-icons';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import { postsApi, authApi, statsApi } from '../api';
import type { Post, Comment } from '../types';
import 'highlight.js/styles/github-dark.css';

export function PostDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [post, setPost] = useState<Post | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState(true);
  const [commentContent, setCommentContent] = useState('');
  const [isAuthenticated, setIsAuthenticated] = useState(false);

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

      // 记录阅读量
      await statsApi.recordPostView(id);
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

  if (loading) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', padding: '48px' }}>
        <Spinner size="large" />
      </div>
    );
  }

  if (!post) {
    return (
      <div style={{ textAlign: 'center', padding: '48px' }}>
        <Body1>文章不存在</Body1>
        <Button appearance="primary" onClick={() => navigate('/')}>
          返回首页
        </Button>
      </div>
    );
  }

  return (
    <div style={{ maxWidth: '800px', margin: '0 auto' }}>
      {/* 返回按钮 */}
      <Button
        appearance="transparent"
        icon={<ArrowLeftRegular />}
        onClick={() => navigate(-1)}
        style={{ marginBottom: '16px' }}
      >
        返回
      </Button>

      {/* 文章卡片 */}
      <Card style={{ marginBottom: '32px' }}>
        <CardHeader
          header={
            <Title1 style={{ fontSize: '36px', fontWeight: '600' }}>
              {post.title}
            </Title1>
          }
          description={
            <div style={{ display: 'flex', gap: '16px', marginTop: '16px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                <CalendarRegular fontSize={14} />
                <Caption1>{formatDate(post.created_at)}</Caption1>
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                <EyeRegular fontSize={14} />
                <Caption1>{post.views} 次阅读</Caption1>
              </div>
              {isAuthenticated && (
                <div style={{ marginLeft: 'auto', display: 'flex', gap: '8px' }}>
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
        <div
          style={{
            padding: '24px 0',
            lineHeight: '1.8',
            fontSize: '16px',
          }}
        >
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            rehypePlugins={[rehypeHighlight]}
            components={{
              h1: ({ node, ...props }) => (
                <h1 style={{ fontSize: '32px', fontWeight: '600', marginTop: '32px', marginBottom: '16px' }} {...props} />
              ),
              h2: ({ node, ...props }) => (
                <h2 style={{ fontSize: '28px', fontWeight: '600', marginTop: '28px', marginBottom: '14px' }} {...props} />
              ),
              h3: ({ node, ...props }) => (
                <h3 style={{ fontSize: '24px', fontWeight: '600', marginTop: '24px', marginBottom: '12px' }} {...props} />
              ),
              p: ({ node, ...props }) => (
                <p style={{ marginBottom: '16px' }} {...props} />
              ),
              code: ({ node, inline, ...props }: any) =>
                inline ? (
                  <code
                    style={{
                      backgroundColor: 'var(--colorNeutralBackground1Hover)',
                      padding: '2px 6px',
                      borderRadius: '4px',
                      fontSize: '14px',
                      fontFamily: 'monospace',
                    }}
                    {...props}
                  />
                ) : (
                  <code
                    style={{
                      display: 'block',
                      backgroundColor: '#0d1117',
                      color: '#c9d1d9',
                      padding: '16px',
                      borderRadius: '8px',
                      overflow: 'auto',
                      fontSize: '14px',
                      fontFamily: 'monospace',
                      marginBottom: '16px',
                    }}
                    {...props}
                  />
                ),
              pre: ({ node, ...props }) => (
                <pre
                  style={{
                    backgroundColor: '#0d1117',
                    padding: '16px',
                    borderRadius: '8px',
                    overflow: 'auto',
                    marginBottom: '16px',
                  }}
                  {...props}
                />
              ),
              a: ({ node, ...props }) => (
                <a
                  style={{ color: 'var(--colorBrandForeground1)', textDecoration: 'underline' }}
                  target="_blank"
                  rel="noopener noreferrer"
                  {...props}
                />
              ),
              ul: ({ node, ...props }) => (
                <ul style={{ marginBottom: '16px', paddingLeft: '24px' }} {...props} />
              ),
              ol: ({ node, ...props }) => (
                <ol style={{ marginBottom: '16px', paddingLeft: '24px' }} {...props} />
              ),
              blockquote: ({ node, ...props }) => (
                <blockquote
                  style={{
                    borderLeft: '4px solid var(--colorBrandStroke1)',
                    paddingLeft: '16px',
                    marginLeft: 0,
                    fontStyle: 'italic',
                    color: 'var(--colorNeutralForeground2)',
                    marginBottom: '16px',
                  }}
                  {...props}
                />
              ),
            }}
          >
            {post.content}
          </ReactMarkdown>
        </div>
      </Card>

      {/* 评论区 */}
      <Card>
        <CardHeader
          header={<Body1 style={{ fontSize: '20px', fontWeight: '600' }}>评论</Body1>}
          description={<Caption1>{comments.length} 条评论</Caption1>}
        />

        {/* 评论输入 */}
        {isAuthenticated ? (
          <div style={{ marginBottom: '24px' }}>
            <Textarea
              placeholder="写下你的评论..."
              value={commentContent}
              onChange={(_, data) => setCommentContent(data.value)}
              style={{ marginBottom: '12px', minHeight: '100px' }}
            />
            <Button appearance="primary" onClick={handleCommentSubmit} disabled={!commentContent.trim()}>
              发表评论
            </Button>
          </div>
        ) : (
          <div style={{ marginBottom: '24px', padding: '16px', backgroundColor: 'var(--colorNeutralBackground1)' }}>
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
        <div style={{ marginTop: '16px' }}>
          {comments.length === 0 ? (
            <div style={{ textAlign: 'center', padding: '32px', color: 'var(--colorNeutralForeground3)' }}>
              <Caption1>暂无评论，快来发表第一条评论吧！</Caption1>
            </div>
          ) : (
            comments.map((comment) => (
              <div
                key={comment.id}
                style={{
                  padding: '16px 0',
                  borderBottom: '1px solid var(--colorNeutralStroke1)',
                }}
              >
                <div style={{ marginBottom: '8px' }}>
                  <strong style={{ fontWeight: '600' }}>
                    {comment.github_username || '用户'}
                  </strong>
                  <Caption1 style={{ marginLeft: '8px', color: 'var(--colorNeutralForeground3)' }}>
                    {formatDate(comment.created_at)}
                  </Caption1>
                </div>
                <Body1 style={{ lineHeight: '1.6' }}>{comment.content}</Body1>
              </div>
            ))
          )}
        </div>
      </Card>
    </div>
  );
}
