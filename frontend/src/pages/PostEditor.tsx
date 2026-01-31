import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { postsApi } from '../api';
import type { PostCreateRequest, PostUpdateRequest } from '../types';

const PostEditor: React.FC = () => {
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
      <div className="post-editor-page">
        <div className="loading-state">
          <p>加载中...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="post-editor-page">
      <div className="editor-container">
        <div className="editor-header">
          <h1>{isEditing ? '编辑文章' : '新建文章'}</h1>
          <div className="editor-actions">
            <button
              onClick={() => navigate(-1)}
              className="cancel-button"
              disabled={loading}
            >
              取消
            </button>
            <button
              onClick={() => handleSubmit(false)}
              className="save-button"
              disabled={loading}
            >
              {loading ? '保存中...' : '保存草稿'}
            </button>
            <button
              onClick={() => handleSubmit(true)}
              className="publish-button"
              disabled={loading}
            >
              {loading ? '发布中...' : isEditing && published ? '更新发布' : '发布'}
            </button>
          </div>
        </div>

        {error && (
          <div className="error-message">
            {error}
            <button onClick={() => setError('')}>×</button>
          </div>
        )}

        <form onSubmit={(e) => { e.preventDefault(); handleSubmit(published); }} className="editor-form">
          <div className="form-group">
            <label htmlFor="post-title">文章标题</label>
            <input
              type="text"
              id="post-title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="输入文章标题..."
              disabled={loading}
              className="title-input"
            />
          </div>

          <div className="form-group">
            <label htmlFor="post-content">文章内容</label>
            <textarea
              id="post-content"
              value={content}
              onChange={(e) => setContent(e.target.value)}
              placeholder="在这里写下你的文章内容..."
              disabled={loading}
              className="content-textarea"
              rows={20}
            />
          </div>

          <div className="form-group checkbox-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={published}
                onChange={(e) => setPublished(e.target.checked)}
                disabled={loading}
              />
              <span>立即发布（取消勾选保存为草稿）</span>
            </label>
          </div>
        </form>

        <div className="editor-footer">
          <div className="word-count">
            <span>标题: {title.length} 字符</span>
            <span>内容: {content.length} 字符</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PostEditor;
