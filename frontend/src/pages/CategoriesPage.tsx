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
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import {
  FolderRegular,
} from '@fluentui/react-icons';
import { categoriesApi, postsApi } from '../api';
import type { Category, Post } from '../types';

const useStyles = makeStyles({
  container: {
    display: 'flex',
    gap: '24px',
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '32px',
    ['@media (max-width: 768px)']: {
      flexDirection: 'column',
    },
  },
  sidebar: {
    flex: '1',
  },
  header: {
    marginBottom: '24px',
  },
  subtitle: {
    color: tokens.colorNeutralForeground2,
    marginTop: '8px',
  },
  categoriesCard: {
    padding: '16px',
    borderRadius: tokens.borderRadiusLarge,
  },
  accordionHeaderContent: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flex: '1',
    cursor: 'pointer',
  },
  categoryName: {
    fontWeight: tokens.fontWeightMedium,
  },
  categoryDesc: {
    color: tokens.colorNeutralForeground3,
    fontSize: '14px',
  },
  accordionPanelContent: {
    marginLeft: '16px',
  },
  mainContent: {
    flex: '2',
  },
  postsCard: {
    borderRadius: tokens.borderRadiusLarge,
  },
  postsHeader: {
    fontWeight: tokens.fontWeightSemibold,
    fontSize: '18px',
  },
  postsHeaderDesc: {
    color: tokens.colorNeutralForeground2,
  },
  emptyText: {
    color: tokens.colorNeutralForeground3,
    padding: '24px',
  },
  postItem: {
    padding: '16px',
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    cursor: 'pointer',
    ':hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    },
    ':last-child': {
      borderBottom: 'none',
    },
  },
  postTitle: {
    fontWeight: tokens.fontWeightSemibold,
    marginBottom: '8px',
    display: 'block',
  },
  postExcerpt: {
    color: tokens.colorNeutralForeground2,
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 2,
    WebkitBoxOrient: 'vertical',
    lineHeight: '1.5',
  },
  loadingContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '48px',
  },
});

export function CategoriesPage() {
  const styles = useStyles();
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
          className={styles.accordionHeaderContent}
          onClick={(e) => {
            e.stopPropagation();
            handleCategoryClick(category);
          }}
        >
          <FolderRegular />
          <span className={styles.categoryName}>{category.name}</span>
          {category.description && (
            <span className={styles.categoryDesc}>
              {category.description}
            </span>
          )}
        </div>
      </AccordionHeader>
      {category.children && category.children.length > 0 && (
        <AccordionPanel>
          <div className={styles.accordionPanelContent}>
            {category.children.map((child: any) => renderCategory(child, level + 1))}
          </div>
        </AccordionPanel>
      )}
    </AccordionItem>
  );

  if (loading) {
    return (
      <div className={styles.loadingContainer}>
        <Spinner size="large" />
      </div>
    );
  }

  const categoryTree = buildCategoryTree(categories);

  return (
    <div className={styles.container}>
      {/* 左侧分类树 */}
      <div className={styles.sidebar}>
        <div className={styles.header}>
          <Title2>分类</Title2>
          <div className={styles.subtitle}>
            <Body1>浏览不同分类的文章</Body1>
          </div>
        </div>

        <Card className={styles.categoriesCard}>
          {categoryTree.length === 0 ? (
            <Body1 className={styles.emptyText}>暂无分类</Body1>
          ) : (
            <Accordion collapsible defaultOpenItems={[]}>
              {categoryTree.map(category => renderCategory(category))}
            </Accordion>
          )}
        </Card>
      </div>

      {/* 右侧文章列表 */}
      {selectedCategory && (
        <div className={styles.mainContent}>
          <Card className={styles.postsCard}>
            <CardHeader
              header={
                <Body1 className={styles.postsHeader}>
                  {selectedCategory.name}
                </Body1>
              }
              description={
                selectedCategory.description ? (
                  <Body1 className={styles.postsHeaderDesc}>
                    {selectedCategory.description}
                  </Body1>
                ) : (
                  <Body1>{posts.length} 篇文章</Body1>
                )
              }
            />
            <div>
              {posts.length === 0 ? (
                <Body1 className={styles.emptyText}>
                  该分类下暂无文章
                </Body1>
              ) : (
                posts.map((post) => (
                  <div
                    key={post.id}
                    className={styles.postItem}
                    onClick={() => navigate(`/post/${post.id}`)}
                  >
                    <Body1 className={styles.postTitle}>
                      {post.title}
                    </Body1>
                    <Body1 className={styles.postExcerpt}>
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
