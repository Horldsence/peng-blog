/**
 * 文章列表页 - 显示所有文章
 */

import { useEffect, useState } from 'react';
import {
  Text,
  Button,
  Input,
  TabList,
  Tab,
  Tag,
  tokens,
  makeStyles,
} from '@fluentui/react-components';
import {
  SearchRegular,
} from '@fluentui/react-icons';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { postsApi, categoriesApi, tagsApi } from '../api';
import type { Post, Category, Tag as TagModel } from '../types';
import { PostCard } from '../components/features/PostCard';

const useStyles = makeStyles({
  pageContainer: {},
  contentSection: {
    position: 'relative',
    zIndex: 2,
    backgroundColor: 'transparent',
    minHeight: 'calc(100vh - 64px)',
    padding: '48px 0',
  },
  contentInner: {
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '0 56px',
  },
  sectionHeader: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: '40px',
    textAlign: 'center',
  },
  sectionTitleGroup: {
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    alignItems: 'center',
  },
  sectionTitle: {
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightBold,
    color: tokens.colorNeutralForeground1,
    margin: '0',
    letterSpacing: '-0.01em',
  },
  sectionSubtitle: {
    fontSize: tokens.fontSizeBase400,
    color: tokens.colorNeutralForeground2,
    margin: '0',
  },
  searchContainer: {
    display: 'flex',
    gap: '12px',
    alignItems: 'center',
    marginBottom: '32px',
    justifyContent: 'center',
  },
  searchInput: {
    width: '300px',
  },
  filtersContainer: {
    display: 'flex',
    gap: '12px',
    flexWrap: 'wrap',
    marginBottom: '40px',
    maxWidth: '800px',
    margin: '0 auto 40px auto',
  },
  filterBadge: {
    cursor: 'pointer',
  },
  postsGrid: {
    display: 'flex',
    flexDirection: 'column',
    gap: '24px',
    maxWidth: '800px',
    margin: '0 auto',
  },

  loadingCard: {
    height: '200px',
    borderRadius: tokens.borderRadiusLarge,
    background: `linear-gradient(90deg, ${tokens.colorNeutralBackground2} 25%, ${tokens.colorNeutralBackground1} 50%, ${tokens.colorNeutralBackground2} 75%)`,
    backgroundSize: '1000px 100%',
    animationName: 'shimmer',
    animationDuration: '1.5s',
    animationIterationCount: 'infinite',
    animationTimingFunction: 'linear',
  },
  emptyState: {
    textAlign: 'center',
    padding: '80px 48px',
    color: tokens.colorNeutralForeground2,
  },
  pagination: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    gap: '16px',
    marginTop: '48px',
  },

});

export function PostsPage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState(searchParams.get('q') || '');

  const [categories, setCategories] = useState<Category[]>([]);
  const [tags, setTags] = useState<TagModel[]>([]);
  const [selectedCategoryId, setSelectedCategoryId] = useState<string>('all');
  const [selectedTagIds, setSelectedTagIds] = useState<string[]>([]);

  useEffect(() => {
    const loadFilters = async () => {
      try {
        const [categoriesRes, tagsRes] = await Promise.all([
          categoriesApi.getCategories({ per_page: 100 }),
          tagsApi.getTags({ per_page: 100 }),
        ]);
        setCategories(categoriesRes.data);
        setTags(tagsRes.data);
      } catch (err) {
        console.error('加载筛选选项失败:', err);
      }
    };
    loadFilters();
  }, []);

  useEffect(() => {
    if (searchQuery) {
      handleSearch();
    } else {
      fetchPosts();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedCategoryId, selectedTagIds]);

  const fetchPosts = async () => {
    try {
      setLoading(true);
      const params: any = {
        page: 1,
        per_page: 50,
      };

      if (selectedCategoryId !== 'all') {
        params.category = selectedCategoryId;
      }

      if (selectedTagIds.length > 0) {
        params.tag = selectedTagIds[selectedTagIds.length - 1];
      }

      const response = await postsApi.getPosts(params);
      setPosts(response.data);
    } catch (error) {
      console.error('Failed to fetch posts:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async () => {
    if (searchQuery.trim()) {
      setSearchParams({ q: searchQuery });
    } else {
      setSearchParams({});
    }

    if (!searchQuery.trim()) {
      fetchPosts();
      return;
    }

    // 搜索时清除筛选
    setSelectedCategoryId('all');
    setSelectedTagIds([]);

    try {
      setLoading(true);
      const response = await postsApi.searchPosts({
        q: searchQuery,
        per_page: 50,
      });
      setPosts(response.data);
    } catch (error) {
      console.error('Search failed:', error);
    } finally {
      setLoading(false);
    }
  };





  return (
    <div className={styles.pageContainer}>
      <section className={styles.contentSection}>
        <div className={styles.contentInner}>
          {/* 区块头部 */}
          <div className={styles.sectionHeader}>
            <div className={styles.sectionTitleGroup}>
              <h1 className={styles.sectionTitle}>文章列表</h1>
              <p className={styles.sectionSubtitle}>探索所有技术文章</p>
            </div>
          </div>

          {/* 搜索框 */}
          <div className={styles.searchContainer}>
            <Input
              placeholder="搜索文章..."
              value={searchQuery}
              onChange={(_, data) => setSearchQuery(data.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              className={styles.searchInput}
              contentBefore={<SearchRegular />}
              size="large"
            />
            <Button appearance="primary" onClick={handleSearch} size="large">
              搜索
            </Button>
          </div>

          {/* 快速过滤 */}
          <div className={styles.filtersContainer} style={{ flexDirection: 'column', gap: '16px' }}>
            <div style={{ width: '100%' }}>
              <TabList
                selectedValue={selectedCategoryId}
                onTabSelect={(_, data) => {
                  setSearchQuery('');
                  setSearchParams({});
                  setSelectedCategoryId(String(data.value));
                }}
              >
                <Tab value="all">全部</Tab>
                {categories.map((c) => (
                  <Tab key={c.id} value={c.id}>
                    {c.name}
                  </Tab>
                ))}
              </TabList>
            </div>

            <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
              {tags.map((t) => {
                const isSelected = selectedTagIds.includes(t.id);
                return (
                  <Tag
                    key={t.id}
                    appearance={isSelected ? 'filled' : 'outline'}
                    shape="circular"
                    style={{
                      cursor: 'pointer',
                      ...(isSelected && {
                        backgroundColor: tokens.colorBrandBackground,
                        color: tokens.colorNeutralForegroundOnBrand,
                        borderColor: 'transparent',
                      }),
                    }}
                    onClick={() => {
                      setSearchQuery('');
                      setSearchParams({});
                      if (isSelected) {
                        setSelectedTagIds(selectedTagIds.filter((id) => id !== t.id));
                      } else {
                        setSelectedTagIds([...selectedTagIds, t.id]);
                      }
                    }}
                  >
                    {t.name}
                  </Tag>
                );
              })}
            </div>
          </div>

          {/* 文章网格 */}
          {loading ? (
            <div className={styles.postsGrid}>
              {[1, 2, 3, 4, 5, 6].map((i) => (
                <div key={i} className={styles.loadingCard} />
              ))}
            </div>
          ) : posts.length === 0 ? (
            <div className={styles.emptyState}>
              <Text size={500}>暂无文章</Text>
            </div>
          ) : (
            <div className={styles.postsGrid}>
              {posts.map((post) => (
                <PostCard key={post.id} post={post} />
              ))}
            </div>
          )}
        </div>
      </section>

      {/* Shimmer动画 */}
      <style>{`
        @keyframes shimmer {
          0% { background-position: -1000px 0; }
          100% { background-position: 1000px 0; }
        }
      `}</style>
    </div>
  );
}