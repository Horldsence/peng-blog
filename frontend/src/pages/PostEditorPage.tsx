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
  Dropdown,
  Combobox,
  Option,
  Tag,
  TagGroup,
  useId,
} from '@fluentui/react-components';
import { ArrowLeftRegular, SaveRegular, SendRegular, DismissRegular } from '@fluentui/react-icons';
import { postsApi, categoriesApi, tagsApi, authApi } from '../api';
import { useToast } from '../components/ui/Toast';
import type { PostCreateRequest, Category, Tag as TagModel } from '../types';

export function PostEditorPage() {
  const { id } = useParams<{ id?: string }>();
  const navigate = useNavigate();
  const toast = useToast();
  const isEditing = Boolean(id);
  const comboId = useId('tags-combo');
  const categoryId = useId('category-dropdown');

  const [currentUser] = useState(() => authApi.getCurrentUser());
  const isAdmin = !!(currentUser?.permissions && currentUser.permissions & 16); // USER_MANAGE = 16

  const [title, setTitle] = useState<string>('');
  const [content, setContent] = useState<string>('');
  const [published, setPublished] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [fetchLoading, setFetchLoading] = useState<boolean>(isEditing);
  const [error, setError] = useState<string>('');

  // 分类和标签相关状态
  const [selectedCategoryId, setSelectedCategoryId] = useState<string | undefined>(undefined);
  const [selectedTags, setSelectedTags] = useState<string[]>([]); // tag IDs
  const [categories, setCategories] = useState<Category[]>([]);
  const [allTags, setAllTags] = useState<TagModel[]>([]);
  const [tagQuery, setTagQuery] = useState<string>('');
  const [isLoadingOptions, setIsLoadingOptions] = useState<boolean>(true);

  // 过滤标签
  const filteredTags = allTags.filter((tag) =>
    tag.name.toLowerCase().includes(tagQuery.toLowerCase())
  );

  // 加载分类和标签列表
  useEffect(() => {
    const fetchOptions = async () => {
      setIsLoadingOptions(true);
      try {
        const [categoriesRes, tagsRes] = await Promise.all([
          categoriesApi.getCategories({ per_page: 100 }),
          tagsApi.getTags({ per_page: 100 }),
        ]);
        setCategories(categoriesRes.data);
        setAllTags(tagsRes.data);
      } catch (err) {
        console.error('加载分类/标签失败:', err);
      } finally {
        setIsLoadingOptions(false);
      }
    };
    void fetchOptions();
  }, []);

  // 加载文章数据（编辑模式）
  useEffect(() => {
    const fetchPost = async () => {
      if (!id) return;

      setFetchLoading(true);
      try {
        const [postRes, tagsRes] = await Promise.all([
          postsApi.getPost(id),
          postsApi.getPostTags(id),
        ]);
        const post = postRes.data;
        setTitle(post.title);
        setContent(post.content);
        setPublished(!!post.published_at);
        setSelectedCategoryId(post.category_id ?? undefined);
        // 加载文章现有标签
        const postTags = tagsRes.data;
        setSelectedTags(postTags.map((t: TagModel) => t.id));
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '获取文章失败';
        setError(errorMessage);
        console.error('获取文章失败:', err);
      } finally {
        setFetchLoading(false);
      }
    };

    if (isEditing) {
      void fetchPost();
    }
  }, [id, isEditing]);

  // 同步标签：添加新标签，删除取消的标签
  const syncTags = async (postId: string, currentTagIds: string[], newTagIds: string[]) => {
    const toAdd = newTagIds.filter((id) => !currentTagIds.includes(id));
    const toRemove = currentTagIds.filter((id) => !newTagIds.includes(id));

    await Promise.all([
      ...toAdd.map((tagId) => postsApi.addPostTag(postId, tagId)),
      ...toRemove.map((tagId) => postsApi.removePostTag(postId, tagId)),
    ]);
  };

  // 创建新分类
  const createNewCategory = async () => {
    // eslint-disable-next-line no-alert
    const name = window.prompt('请输入新分类名称:');
    if (!name) return;

    try {
      const res = await categoriesApi.createCategory({ name, slug: name });
      const newCategory = res.data;
      setCategories([...categories, newCategory]);
      setSelectedCategoryId(newCategory.id);
    } catch (e) {
      console.error('创建分类失败', e);
      toast.showError('创建分类失败');
    }
  };

  // 创建新标签
  const createNewTag = async () => {
    const tagName = tagQuery.trim();
    if (!tagName || !isAdmin) return;

    try {
      const res = await tagsApi.createTag({ name: tagName, slug: tagName });
      const newTag = res.data;
      setAllTags([...allTags, newTag]);
      setSelectedTags([...selectedTags, newTag.id]);
      setTagQuery('');
    } catch (e) {
      console.error('创建标签失败', e);
      toast.showError('创建标签失败: 可能已存在');
    }
  };

  // 处理标签选择
  const onTagSelect = async (
    _: unknown,
    data: { optionValue?: string; selectedOptions?: string[] }
  ) => {
    if (data.optionValue === 'create-new') {
      await createNewTag();
      return;
    }

    if (data.selectedOptions) {
      setSelectedTags(data.selectedOptions);
      setTagQuery('');
    }
  };

  const handleSubmit = async (shouldPublish: boolean = false) => {
    if (!title.trim() || !content.trim()) {
      setError('标题和内容不能为空');
      return;
    }

    setLoading(true);
    setError('');

    try {
      let postId: string;
      let originalTagIds: string[] = [];

      if (isEditing && id) {
        postId = id;
        // 获取当前文章的标签
        const currentTagsRes = await postsApi.getPostTags(id);
        originalTagIds = currentTagsRes.data.map((t: TagModel) => t.id);

        // 更新文章内容和分类
        await postsApi.patchPost(id, {
          title: title.trim(),
          content: content.trim(),
          category_id: selectedCategoryId ?? null,
        });

        // 同步标签
        await syncTags(postId, originalTagIds, selectedTags);

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
        postId = newPost.id;

        // 设置分类
        if (selectedCategoryId) {
          await postsApi.patchPost(postId, { category_id: selectedCategoryId });
        }

        // 添加标签
        await Promise.all(selectedTags.map((tagId) => postsApi.addPostTag(postId, tagId)));

        if (shouldPublish) {
          await postsApi.publishPost(newPost.id);
        }

        navigate(`/post/${newPost.id}`);
        return;
      }

      if (shouldPublish) {
        toast.showSuccess(isEditing ? '更新并发布成功！' : '发布成功！');
      } else {
        toast.showSuccess(isEditing ? '保存成功！' : '草稿保存成功！');
      }

      if (isEditing) {
        navigate(`/post/${id}`);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '保存失败';
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
              style={{ width: '100%' }}
              textarea={{ style: { minHeight: '400px' } }}
              disabled={loading}
              resize="vertical"
            />
          </div>

          {/* 分类和标签选择 */}
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '24px' }}>
            {/* 分类选择 */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                }}
              >
                <label id={categoryId} style={{ fontWeight: '600' }}>
                  分类
                </label>
                {isAdmin && (
                  <Button
                    size="small"
                    appearance="subtle"
                    onClick={() => {
                      void createNewCategory();
                    }}
                  >
                    新建
                  </Button>
                )}
              </div>
              <Dropdown
                aria-labelledby={categoryId}
                placeholder="选择分类"
                value={categories.find((c) => c.id === selectedCategoryId)?.name ?? ''}
                selectedOptions={selectedCategoryId ? [selectedCategoryId] : []}
                onOptionSelect={(_, data) => {
                  if (data.optionValue) setSelectedCategoryId(data.optionValue);
                }}
                disabled={loading || isLoadingOptions}
              >
                {categories.map((c) => (
                  <Option key={c.id} value={c.id} text={c.name}>
                    {c.name}
                  </Option>
                ))}
              </Dropdown>
            </div>

            {/* 标签选择 */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              <label id={comboId} style={{ fontWeight: '600' }}>
                标签
              </label>
              <div style={{ display: 'flex', gap: '8px', alignItems: 'flex-start' }}>
                <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '8px' }}>
                  <Combobox
                    aria-labelledby={comboId}
                    placeholder="搜索或添加标签"
                    onChange={(ev) => setTagQuery(ev.target.value)}
                    value={tagQuery}
                    selectedOptions={selectedTags}
                    onOptionSelect={(_, data) => {
                      void onTagSelect(_, data);
                    }}
                    disabled={loading || isLoadingOptions}
                    freeform
                    multiselect
                  >
                    {filteredTags.map((t) => (
                      <Option key={t.id} value={t.id} text={t.name}>
                        {t.name}
                      </Option>
                    ))}
                    {isAdmin &&
                      tagQuery &&
                      !allTags.some((t) => t.name.toLowerCase() === tagQuery.toLowerCase()) && (
                        <Option key="create-new" value="create-new" text={`创建 "${tagQuery}"`}>
                          创建 "{tagQuery}"
                        </Option>
                      )}
                  </Combobox>
                  {/* 已选标签展示 */}
                  {selectedTags.length > 0 && (
                    <TagGroup aria-label="Selected tags" style={{ flexWrap: 'wrap' }}>
                      {selectedTags.map((tagId) => {
                        const tag = allTags.find((t) => t.id === tagId);
                        if (!tag) return null;
                        return (
                          <Tag
                            key={tagId}
                            dismissible
                            shape="rounded"
                            dismissIcon={
                              <DismissRegular
                                onClick={() =>
                                  setSelectedTags(selectedTags.filter((id) => id !== tagId))
                                }
                              />
                            }
                          >
                            {tag.name}
                          </Tag>
                        );
                      })}
                    </TagGroup>
                  )}
                </div>
              </div>
            </div>
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
              onClick={() => void handleSubmit(false)}
              disabled={loading}
            >
              {loading ? '保存中...' : '保存草稿'}
            </Button>
            <Button
              appearance="primary"
              icon={<SendRegular />}
              onClick={() => {
                void handleSubmit(true);
              }}
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
