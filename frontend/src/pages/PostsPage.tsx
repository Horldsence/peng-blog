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

const styles = {
  pageContainer: {
    margin: '-32px',
  } as React.CSSProperties,

  contentSection: {
    position: 'relative',
    zIndex: 2,
    backgroundColor: tokens.colorNeutralBackground3,
    minHeight: 'calc(100vh - 64px)',
    padding: '48px 0',
  } as React.CSSProperties,

  contentInner: {
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '0 56px',
  } as React.CSSProperties,

  sectionHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-end',
    marginBottom: '40px',
  } as React.CSSProperties,

  sectionTitleGroup: {
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
  } as React.CSSProperties,

  sectionTitle: {
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightBold,
    color: tokens.colorNeutralForeground1,
    margin: 0,
    letterSpacing: '-0.01em',
  } as React.CSSProperties,

  sectionSubtitle: {
    fontSize: tokens.fontSizeBase400,
    color: tokens.colorNeutralForeground2,
    margin: 0,
  } as React.CSSProperties,

  // 搜索区域
  searchContainer: {
    display: 'flex',
    gap: '12px',
    alignItems: 'center',
    marginBottom: '32px',
  } as React.CSSProperties,

  searchInput: {
    width: '300px',
  } as React.CSSProperties,

  // 快速过滤
  filtersContainer: {
    display: 'flex',
    gap: '12px',
    flexWrap: 'wrap',
    marginBottom: '40px',
  } as React.CSSProperties,

  // 文章网格
  postsGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(340px, 1fr))',
    gap: '28px',
  } as React.CSSProperties,

  postCard: {
    cursor: 'pointer',
    opacity: 0,
    transform: 'translateY(40px)',
    transition: 'box-shadow 0.3s ease',
    borderRadius: tokens.borderRadiusLarge,
  } as React.CSSProperties,

  cardHeader: {
    padding: '24px',
  } as React.CSSProperties,

  cardTitle: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 2,
    WebkitBoxOrient: 'vertical',
    marginBottom: '12px',
  } as React.CSSProperties,

  cardDescription: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 3,
    WebkitBoxOrient: 'vertical',
    color: tokens.colorNeutralForeground2,
    lineHeight: '1.6',
  } as React.CSSProperties,

  cardFooter: {
    display: 'flex',
    alignItems: 'center',
    gap: '20px',
    padding: '16px 24px',
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
  } as React.CSSProperties,

  metaItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    color: tokens.colorNeutralForeground2,
  } as React.CSSProperties,

  loadingCard: {
    height: '300px',
    borderRadius: tokens.borderRadiusLarge,
    background: `linear-gradient(90deg, ${tokens.colorNeutralBackground2} 25%, ${tokens.colorNeutralBackground1} 50%, ${tokens.colorNeutralBackground2} 75%)`,
    backgroundSize: '1000px 100%',
    animation: 'shimmer 1.5s infinite linear',
  } as React.CSSProperties,

  emptyState: {
    textAlign: 'center',
    padding: '80px 48px',
    color: tokens.colorNeutralForeground2,
  } as React.CSSProperties,

  pagination: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    gap: '16px',
    marginTop: '48px',
  } as React.CSSProperties,
};

export function PostsPage() {
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
    <div ref={pageRef} style={styles.pageContainer}>
      <section style={styles.contentSection}>
        <div style={styles.contentInner}>
          {/* 区块头部 */}
          <div style={styles.sectionHeader}>
            <div style={styles.sectionTitleGroup}>
              <h1 style={styles.sectionTitle}>文章列表</h1>
              <p style={styles.sectionSubtitle}>探索所有技术文章</p>
            </div>
          </div>

          {/* 搜索框 */}
          <div style={styles.searchContainer}>
            <Input
              placeholder="搜索文章..."
              value={searchQuery}
              onChange={(_, data) => setSearchQuery(data.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              style={styles.searchInput}
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
          <div style={styles.filtersContainer}>
            <Badge size="extra-large" color="brand" appearance="filled">
              全部文章
            </Badge>
            <Badge size="extra-large" appearance="ghost" style={{ cursor: 'pointer' }}>
              Rust
            </Badge>
            <Badge size="extra-large" appearance="ghost" style={{ cursor: 'pointer' }}>
              React
            </Badge>
            <Badge size="extra-large" appearance="ghost" style={{ cursor: 'pointer' }}>
              TypeScript
            </Badge>
            <Badge size="extra-large" appearance="ghost" style={{ cursor: 'pointer' }}>
              Web 开发
            </Badge>
          </div>

          {/* 文章网格 */}
          {loading ? (
            <div style={styles.postsGrid}>
              {[1, 2, 3, 4, 5, 6].map((i) => (
                <div key={i} style={styles.loadingCard} />
              ))}
            </div>
          ) : posts.length === 0 ? (
            <div style={styles.emptyState}>
              <Text size={500}>暂无文章</Text>
            </div>
          ) : (
            <div style={styles.postsGrid}>
              {posts.map((post, index) => (
                <Card
                  key={post.id}
                  ref={(el) => { cardsRef.current[index] = el; }}
                  style={styles.postCard}
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
                    style={styles.cardHeader}
                    header={
                      <Text
                        weight="semibold"
                        size={500}
                        style={styles.cardTitle}
                      >
                        {post.title}
                      </Text>
                    }
                    description={
                      <Caption1 style={styles.cardDescription}>
                        {getPostExcerpt(post.content, 180)}
                      </Caption1>
                    }
                  />

                  <div style={styles.cardFooter}>
                    <div style={styles.metaItem}>
                      <CalendarRegular fontSize={14} />
                      <Caption1>{formatDate(post.created_at)}</Caption1>
                    </div>
                    <div style={styles.metaItem}>
                      <EyeRegular fontSize={14} />
                      <Caption1>{post.views}</Caption1>
                    </div>
                    <div style={{ marginLeft: 'auto' }}>
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
