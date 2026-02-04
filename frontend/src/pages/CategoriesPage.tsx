/**
 * 分类页面 - 层级分类和文章过滤
 */

import { useEffect, useState } from 'react';
import {
  Accordion,
  AccordionHeader,
  AccordionItem,
  AccordionPanel,
  Card,
  Body1,
  Title2,
  Spinner,
  tokens,
  makeStyles,
  Button,
} from '@fluentui/react-components';
import { FolderRegular, DismissRegular } from '@fluentui/react-icons';
import { categoriesApi, postsApi } from '../api';
import type { Category, Post } from '../types';
import { PostCard } from '../components/features/PostCard';

interface TreeNode extends Category {
  children: TreeNode[];
}

const useStyles = makeStyles({
  container: {
    display: 'flex',
    gap: '48px',
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '48px 24px',
    ['@media (max-width: 768px)']: {
      flexDirection: 'column',
      padding: '24px 16px',
      gap: '24px',
    },
  },
  sidebar: {
    flex: '0 0 300px',
    '@media (max-width: 768px)': {
      flex: 'auto',
      width: '100%',
    },
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
    backgroundColor: tokens.colorNeutralBackground1,
    border: 'none',
  },
  accordionHeaderContent: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
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
    flex: '1',
  },
  postsList: {
    display: 'flex',
    flexDirection: 'column',
    gap: '24px',
  },
  categoryHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: '32px',
    '@media (max-width: 768px)': {
      flexDirection: 'column',
      gap: '16px',
      alignItems: 'stretch',
    },
  },
  emptyText: {
    color: tokens.colorNeutralForeground3,
    textAlign: 'center',
    padding: '48px',
    fontSize: tokens.fontSizeBase400,
  },
  loadingContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '48px',
  },
  errorText: {
    color: tokens.colorPaletteRedForeground1,
    textAlign: 'center',
    padding: '48px',
    fontSize: tokens.fontSizeBase400,
  },
  clearButton: {
    flexShrink: 0,
    '@media (max-width: 768px)': {
      width: '100%',
    },
  },
});

export function CategoriesPage() {
  const styles = useStyles();
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(true);
  const [postsLoading, setPostsLoading] = useState(false);
  const [postsError, setPostsError] = useState<string | null>(null);
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    void fetchCategories();
  }, []);

  useEffect(() => {
    if (selectedCategory) {
      void fetchPostsByCategory(selectedCategory.id);
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
      setPostsLoading(true);
      setPostsError(null);

      const response = await postsApi.getPosts({
        category: categoryId,
        page: 1,
        per_page: 50,
      });

      setPosts(response.data);
    } catch (error) {
      console.error('Failed to fetch posts:', error);
      setPostsError('加载文章失败，请稍后重试');
      setPosts([]);
    } finally {
      setPostsLoading(false);
    }
  };

  const handleCategoryClick = (category: Category) => {
    setSelectedCategory(category);
  };

  const handleClearSelection = () => {
    setSelectedCategory(null);
    setPosts([]);
    setPostsError(null);
  };

  // 构建分类树
  const buildCategoryTree = (categories: Category[]): TreeNode[] => {
    const map = new Map<string, TreeNode>();
    categories.forEach((cat) => map.set(cat.id, { ...cat, children: [] }));

    const root: TreeNode[] = [];
    categories.forEach((cat) => {
      const node = map.get(cat.id);
      if (!node) return;

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

  const renderCategory = (category: TreeNode, level: number = 0) => (
    <AccordionItem key={category.id} value={category.id}>
      <AccordionHeader
        expandIconPosition="end"
        style={{ paddingLeft: `${level * 16}px` }}
        onClick={() => handleCategoryClick(category)}
      >
        <div className={styles.accordionHeaderContent}>
          <FolderRegular />
          <span className={styles.categoryName}>{category.name}</span>
          {category.description && (
            <span className={styles.categoryDesc}>{category.description}</span>
          )}
        </div>
      </AccordionHeader>
      {category.children && category.children.length > 0 && (
        <AccordionPanel>
          <div className={styles.accordionPanelContent}>
            {category.children.map((child) => renderCategory(child, level + 1))}
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
              {categoryTree.map((category) => renderCategory(category))}
            </Accordion>
          )}
        </Card>
      </div>

      {/* 右侧文章列表 */}
      {selectedCategory && (
        <div className={styles.mainContent}>
          <div className={styles.categoryHeader}>
            <div>
              <Title2>{selectedCategory.name}</Title2>
              <div className={styles.subtitle}>
                {selectedCategory.description && <Body1>{selectedCategory.description}</Body1>}
                <Body1>{posts.length} 篇文章</Body1>
              </div>
            </div>
            <Button
              appearance="subtle"
              icon={<DismissRegular />}
              onClick={handleClearSelection}
              className={styles.clearButton}
            >
              清除选择
            </Button>
          </div>

          {postsLoading ? (
            <div className={styles.loadingContainer}>
              <Spinner size="large" />
            </div>
          ) : postsError ? (
            <div className={styles.errorText}>{postsError}</div>
          ) : (
            <div className={styles.postsList}>
              {posts.length === 0 ? (
                <div className={styles.emptyText}>该分类下暂无文章</div>
              ) : (
                posts.map((post) => <PostCard key={post.id} post={post} />)
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
