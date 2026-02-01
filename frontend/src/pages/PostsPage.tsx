/**
 * 文章列表页 - 显示所有文章
 */

import { useEffect, useRef, useState } from 'react';
import {
  Card,
  CardHeader,
  Badge,
  Caption1,
  Text,
  Button,
  Input,
  tokens,
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import {
  ArrowRightRegular,
  CalendarRegular,
  EyeRegular,
  SearchRegular,
} from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { postsApi } from '../api';
import type { Post } from '../types';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import { getPostExcerpt } from '../utils/markdown';

gsap.registerPlugin(ScrollTrigger);

const useStyles = makeStyles({
  pageContainer: {
    // Removed negative margin as discussed
  },
  contentSection: {
    position: 'relative',
    zIndex: 2,
    backgroundColor: tokens.colorNeutralBackground3,
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
    justifyContent: 'space-between',
    alignItems: 'flex-end',
    marginBottom: '40px',
  },
  sectionTitleGroup: {
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
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
  },
  searchInput: {
    width: '300px',
  },
  filtersContainer: {
    display: 'flex',
    gap: '12px',
    flexWrap: 'wrap',
    marginBottom: '40px',
  },
  filterBadge: {
    cursor: 'pointer',
  },
  postsGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(340px, 1fr))',
    gap: '28px',
  },
  postCard: {
    cursor: 'pointer',
    opacity: 0,
    transform: 'translateY(40px)',
    transition: 'box-shadow 0.3s ease',
    borderRadius: tokens.borderRadiusLarge,
  },
  cardHeader: {
    padding: '24px',
  },
  cardTitle: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 2,
    WebkitBoxOrient: 'vertical',
    marginBottom: '12px',
  },
  cardDescription: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 3,
    WebkitBoxOrient: 'vertical',
    color: tokens.colorNeutralForeground2,
    lineHeight: '1.6',
  },
  cardFooter: {
    display: 'flex',
    alignItems: 'center',
    gap: '20px',
    padding: '16px 24px',
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
  },
  metaItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    color: tokens.colorNeutralForeground2,
  },
  loadingCard: {
    height: '300px',
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
  viewButtonContainer: {
    marginLeft: 'auto',
  },
});

export function PostsPage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');

  const pageRef = useRef<HTMLDivElement>(null);
  const cardsRef = useRef<(HTMLDivElement | null)[]>([]);

  useEffect(() => {
    fetchPosts();
  }, []);

  useEffect(() => {
    const ctx = gsap.context(() => {
      cardsRef.current.forEach((card, index) => {
        if (card) {
          gsap.to(card, {
            opacity: 1,
            y: 0,
            duration: 0.6,
            delay: index * 0.08,
            ease: 'power3.out',
            scrollTrigger: {
              trigger: card,
              start: 'top 85%',
              toggleActions: 'play none none none',
            },
          });
        }
      });
    }, pageRef);

    return () => ctx.revert();
  }, [posts]);

  const fetchPosts = async () => {
    try {
      setLoading(true);
      const response = await postsApi.getPosts({
        page: 1,
        per_page: 50,
      });
      setPosts(response.data);
    } catch (error) {
      console.error('Failed to fetch posts:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      fetchPosts();
      return;
    }

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

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  return (
    <div ref={pageRef} className={styles.pageContainer}>
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
            <Button
              appearance="primary"
              onClick={handleSearch}
              size="large"
            >
              搜索
            </Button>
          </div>

          {/* 快速过滤 */}
          <div className={styles.filtersContainer}>
            <Badge size="extra-large" color="brand" appearance="filled" className={styles.filterBadge}>
              全部文章
            </Badge>
            <Badge size="extra-large" appearance="ghost" className={styles.filterBadge}>
              Rust
            </Badge>
            <Badge size="extra-large" appearance="ghost" className={styles.filterBadge}>
              React
            </Badge>
            <Badge size="extra-large" appearance="ghost" className={styles.filterBadge}>
              TypeScript
            </Badge>
            <Badge size="extra-large" appearance="ghost" className={styles.filterBadge}>
              Web 开发
            </Badge>
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
              {posts.map((post, index) => (
                <Card
                  key={post.id}
                  ref={(el) => { cardsRef.current[index] = el; }}
                  className={styles.postCard}
                  onClick={() => navigate(`/post/${post.id}`)}
                  onMouseEnter={(e) => {
                    gsap.to(e.currentTarget, {
                      y: -8,
                      boxShadow: '0 20px 40px rgba(0,0,0,0.15)',
                      duration: 0.3,
                      ease: 'power2.out',
                    });
                  }}
                  onMouseLeave={(e) => {
                    gsap.to(e.currentTarget, {
                      y: 0,
                      boxShadow: 'none',
                      duration: 0.3,
                      ease: 'power2.out',
                    });
                  }}
                >
                  <CardHeader
                    className={styles.cardHeader}
                    header={
                      <Text
                        weight="semibold"
                        size={500}
                        className={styles.cardTitle}
                      >
                        {post.title}
                      </Text>
                    }
                    description={
                      <Caption1 className={styles.cardDescription}>
                        {getPostExcerpt(post.content, 180)}
                      </Caption1>
                    }
                  />

                  <div className={styles.cardFooter}>
                    <div className={styles.metaItem}>
                      <CalendarRegular fontSize={14} />
                      <Caption1>{formatDate(post.created_at)}</Caption1>
                    </div>
                    <div className={styles.metaItem}>
                      <EyeRegular fontSize={14} />
                      <Caption1>{post.views}</Caption1>
                    </div>
                    <div className={styles.viewButtonContainer}>
                      <Button
                        appearance="transparent"
                        icon={<ArrowRightRegular />}
                        onClick={(e) => {
                          e.stopPropagation();
                          navigate(`/post/${post.id}`);
                        }}
                      >
                        阅读
                      </Button>
                    </div>
                  </div>
                </Card>
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
