import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Card,
  CardHeader,
  Button,
  Input,
  Textarea,
  Title2,
  Body1,
  Caption1,
  Spinner,
  tokens,
} from '@fluentui/react-components';
import { ArrowLeftRegular, SaveRegular, SendRegular } from '@fluentui/react-icons';
import { postsApi } from '../api';
import type { PostCreateRequest, PostUpdateRequest } from '../types';

export function PostEditorPage() {
  const { id } = useParams<{ id?: string }>();
  const navigate = useNavigate();
  const isEditing = Boolean(id);

  const [title, setTitle] = useState<string>('');
  const [content, setContent] = useState<string>('');
  const [published, setPublished] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [fetchLoading, setFetchLoading] = useState<boolean>(isEditing);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    const fetchPost = async () => {
      if (!id) return;

      setFetchLoading(true);
      try {
        const response = await postsApi.getPost(id);
        const post = response.data;
        setTitle(post.title);
        setContent(post.content);
        setPublished(!!post.published_at);
      } catch (err: any) {
        const errorMessage = err.message || '获取文章失败';
        setError(errorMessage);
        console.error('获取文章失败:', err);
      } finally {
        setFetchLoading(false);
      }
    };

    if (isEditing) {
      fetchPost();
    }
  }, [id, isEditing]);

  const handleSubmit = async (shouldPublish: boolean = false) => {
    if (!title.trim() || !content.trim()) {
      setError('标题和内容不能为空');
      return;
    }

    setLoading(true);
    setError('');

    try {
      if (isEditing && id) {
        const updateData: PostUpdateRequest = {
          title: title.trim(),
          content: content.trim(),
        };
        await postsApi.updatePost(id, updateData);

        if (shouldPublish) {
          await postsApi.publishPost(id);
        } else if (!shouldPublish && published) {
          await postsApi.unpublishPost(id);
        }
      } else {
        const createData: PostCreateRequest = {
          title: title.trim(),
          content: content.trim(),
        };
        const response = await postsApi.createPost(createData);
        const newPost = response.data;

        if (shouldPublish) {
          await postsApi.publishPost(newPost.id);
        }

        navigate(`/post/${newPost.id}`);
        return;
      }

      if (shouldPublish) {
        alert(isEditing ? '更新并发布成功！' : '发布成功！');
      } else {
        alert(isEditing ? '保存成功！' : '草稿保存成功！');
      }

      if (isEditing) {
        navigate(`/post/${id}`);
      }
    } catch (err: any) {
      const errorMessage = err.message || '保存失败';
      setError(errorMessage);
      console.error('保存失败:', err);
    } finally {
      setLoading(false);
    }
  };

  if (fetchLoading) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', padding: '48px' }}>
        <Spinner size="large" />
      </div>
    );
  }

  return (
    <div style={{ maxWidth: '900px', margin: '0 auto' }}>
      {/* 返回按钮 */}
      <Button
        appearance="transparent"
        icon={<ArrowLeftRegular />}
        onClick={() => navigate(-1)}
        style={{ marginBottom: '16px' }}
      >
        返回
      </Button>

      {/* 编辑器卡片 */}
      <Card style={{ borderRadius: tokens.borderRadiusLarge }}>
        <CardHeader header={<Title2>{isEditing ? '编辑文章' : '新建文章'}</Title2>} />

        {/* 错误提示 */}
        {error && (
          <div
            style={{
              padding: '12px 16px',
              marginBottom: '16px',
              backgroundColor: 'var(--colorStatusDangerBackground1)',
              border: '1px solid var(--colorStatusDangerBorder1)',
              borderRadius: tokens.borderRadiusMedium,
              color: 'var(--colorStatusDangerForeground1)',
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
            }}
          >
            <Body1>{error}</Body1>
            <Button appearance="transparent" size="small" onClick={() => setError('')}>
              ×
            </Button>
          </div>
        )}

        {/* 表单 */}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
          {/* 标题输入 */}
          <div>
            <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>文章标题</Body1>
            <Input
              placeholder="输入文章标题..."
              value={title}
              onChange={(_, data) => setTitle(data.value)}
              style={{ width: '100%' }}
              size="large"
              disabled={loading}
            />
          </div>

          {/* 内容输入 */}
          <div>
            <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>文章内容</Body1>
            <Textarea
              placeholder="在这里写下你的文章内容..."
              value={content}
              onChange={(_, data) => setContent(data.value)}
              style={{ width: '100%', minHeight: '400px' }}
              disabled={loading}
              resize="vertical"
            />
          </div>

          {/* 发布选项 */}
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '12px 16px',
              backgroundColor: 'var(--colorNeutralBackground1)',
              borderRadius: tokens.borderRadiusMedium,
            }}
          >
            <input
              type="checkbox"
              id="publish-checkbox"
              checked={published}
              onChange={(e) => setPublished(e.target.checked)}
              disabled={loading}
              style={{ width: '18px', height: '18px' }}
            />
            <label htmlFor="publish-checkbox" style={{ cursor: 'pointer' }}>
              <Body1>立即发布（取消勾选保存为草稿）</Body1>
            </label>
          </div>

          {/* 操作按钮 */}
          <div style={{ display: 'flex', gap: '12px', justifyContent: 'flex-end' }}>
            <Button appearance="secondary" onClick={() => navigate(-1)} disabled={loading}>
              取消
            </Button>
            <Button
              appearance="secondary"
              icon={<SaveRegular />}
              onClick={() => handleSubmit(false)}
              disabled={loading}
            >
              {loading ? '保存中...' : '保存草稿'}
            </Button>
            <Button
              appearance="primary"
              icon={<SendRegular />}
              onClick={() => handleSubmit(true)}
              disabled={loading}
            >
              {loading ? '发布中...' : isEditing && published ? '更新发布' : '发布'}
            </Button>
          </div>

          {/* 字数统计 */}
          <div
            style={{
              display: 'flex',
              gap: '24px',
              padding: '12px 16px',
              backgroundColor: 'var(--colorNeutralBackground2)',
              borderRadius: tokens.borderRadiusMedium,
              color: 'var(--colorNeutralForeground2)',
            }}
          >
            <Caption1>标题: {title.length} 字符</Caption1>
            <Caption1>内容: {content.length} 字符</Caption1>
          </div>
        </div>
      </Card>
    </div>
  );
}
