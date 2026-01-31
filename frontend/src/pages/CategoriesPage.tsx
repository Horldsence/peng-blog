/**
 * 分类页面 - 层级分类和文章过滤
 */

import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Accordion,
  AccordionHeader,
  AccordionItem,
  AccordionPanel,
  Card,
  CardHeader,
  Body1,
  Title2,
  Spinner,
  tokens,
} from '@fluentui/react-components';
import {
  FolderRegular,
} from '@fluentui/react-icons';
import { categoriesApi, postsApi } from '../api';
import type { Category, Post } from '../types';

export function CategoriesPage() {
  const navigate = useNavigate();
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    fetchCategories();
  }, []);

  useEffect(() => {
    if (selectedCategory) {
      fetchPostsByCategory(selectedCategory.id);
    }
  }, [selectedCategory]);

  const fetchCategories = async () => {
    try {
      setLoading(true);
      const response = await categoriesApi.getCategories();
      setCategories(response.data);
    } catch (error) {
      console.error('Failed to fetch categories:', error);
    } finally {
      setLoading(false);
    }
  };

  const fetchPostsByCategory = async (categoryId: string) => {
    try {
      const response = await postsApi.getPosts({
        category: categoryId,
        page: 1,
        per_page: 10,
      });
      setPosts(response.data);
    } catch (error) {
      console.error('Failed to fetch posts:', error);
    }
  };

  const handleCategoryClick = (category: Category) => {
    setSelectedCategory(category);
  };

  // 构建分类树
  const buildCategoryTree = (categories: Category[]): any[] => {
    const map = new Map<string, any>();
    categories.forEach(cat => map.set(cat.id, { ...cat, children: [] }));

    const root: Category[] = [];
    categories.forEach(cat => {
      const node = map.get(cat.id)!;
      if (cat.parent_id) {
        const parent = map.get(cat.parent_id);
        if (parent) {
          if (!parent.children) parent.children = [];
          parent.children.push(node);
        }
      } else {
        root.push(node);
      }
    });

    return root;
  };

  const renderCategory = (category: any, level: number = 0) => (
    <AccordionItem key={category.id} value={category.id}>
      <AccordionHeader
        expandIconPosition="end"
        style={{ paddingLeft: `${level * 16}px` }}
      >
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            flex: 1,
            cursor: 'pointer',
          }}
          onClick={(e) => {
            e.stopPropagation();
            handleCategoryClick(category);
          }}
        >
          <FolderRegular />
          <span style={{ fontWeight: '500' }}>{category.name}</span>
          {category.description && (
            <span style={{ color: 'var(--colorNeutralForeground3)', fontSize: '14px' }}>
              {category.description}
            </span>
          )}
        </div>
      </AccordionHeader>
      {category.children && category.children.length > 0 && (
        <AccordionPanel>
          <div style={{ marginLeft: '16px' }}>
            {category.children.map((child: any) => renderCategory(child, level + 1))}
          </div>
        </AccordionPanel>
      )}
    </AccordionItem>
  );

  if (loading) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', padding: '48px' }}>
        <Spinner size="large" />
      </div>
    );
  }

  const categoryTree = buildCategoryTree(categories);

  return (
    <div style={{ display: 'flex', gap: '24px' }}>
      {/* 左侧分类树 */}
      <div style={{ flex: 1 }}>
        <div style={{ marginBottom: '24px' }}>
          <Title2>分类</Title2>
          <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
            浏览不同分类的文章
          </Body1>
        </div>

        <Card style={{ padding: '16px', borderRadius: tokens.borderRadiusLarge }}>
          {categoryTree.length === 0 ? (
            <Body1 style={{ color: 'var(--colorNeutralForeground3)' }}>暂无分类</Body1>
          ) : (
            <Accordion collapsible defaultOpenItems={[]}>
              {categoryTree.map(category => renderCategory(category))}
            </Accordion>
          )}
        </Card>
      </div>

      {/* 右侧文章列表 */}
      {selectedCategory && (
        <div style={{ flex: 2 }}>
          <Card style={{ borderRadius: tokens.borderRadiusLarge }}>
            <CardHeader
              header={
                <Body1 style={{ fontWeight: '600', fontSize: '18px' }}>
                  {selectedCategory.name}
                </Body1>
              }
              description={
                selectedCategory.description ? (
                  <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
                    {selectedCategory.description}
                  </Body1>
                ) : (
                  <Body1>{posts.length} 篇文章</Body1>
                )
              }
            />
            <div>
              {posts.length === 0 ? (
                <Body1 style={{ color: 'var(--colorNeutralForeground3)', padding: '24px' }}>
                  该分类下暂无文章
                </Body1>
              ) : (
                posts.map((post) => (
                  <div
                    key={post.id}
                    style={{
                      padding: '16px',
                      borderBottom: '1px solid var(--colorNeutralStroke1)',
                      cursor: 'pointer',
                    }}
                    onClick={() => navigate(`/post/${post.id}`)}
                  >
                    <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>
                      {post.title}
                    </Body1>
                    <Body1
                      style={{
                        color: 'var(--colorNeutralForeground2)',
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                        display: '-webkit-box',
                        WebkitLineClamp: 2,
                        WebkitBoxOrient: 'vertical',
                      }}
                    >
                      {post.content.substring(0, 100)}...
                    </Body1>
                  </div>
                ))
              )}
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
