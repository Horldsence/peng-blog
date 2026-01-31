/**
 * 主页 - 文章卡片网格
 */

import { useEffect, useState } from 'react';
import {
  Card,
  CardHeader,
  Badge,
  Caption1,
  Text,
  Button,
  Spinner,
  Input,
} from '@fluentui/react-components';
import {
  ArrowRightRegular,
  CalendarRegular,
  EyeRegular,
} from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { postsApi } from '../api';
import type { Post } from '../types';

export function HomePage() {
  const navigate = useNavigate();
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentPage] = useState(1);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory] = useState<string>('');
  const [selectedTag] = useState<string>('');

  useEffect(() => {
    fetchPosts();
  }, [currentPage, selectedCategory, selectedTag]);

  const fetchPosts = async () => {
    try {
      setLoading(true);
      const response = await postsApi.getPosts({
        page: currentPage,
        per_page: 12,
        category: selectedCategory || undefined,
        tag: selectedTag || undefined,
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
        per_page: 12,
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
    <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '32px' }}>
      {/* 头部 */}
      <div style={{ marginBottom: '32px' }}>
        <h1 style={{ fontSize: '32px', fontWeight: '600', marginBottom: '16px' }}>
          欢迎来到 Peng Blog
        </h1>
        <Text size={500} style={{ color: 'var(--colorNeutralForeground2)' }}>
          探索技术文章、教程和见解
        </Text>
      </div>

      {/* 搜索和过滤 */}
      <div style={{ marginBottom: '32px', display: 'flex', gap: '12px' }}>
        <Input
          placeholder="搜索文章..."
          value={searchQuery}
          onChange={(_, data) => setSearchQuery(data.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
          style={{ flex: 1 }}
          size="large"
        />
        <Button appearance="primary" onClick={handleSearch} size="large">
          搜索
        </Button>
      </div>

      {/* 快速过滤 */}
      <div style={{ marginBottom: '32px', display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
        <Badge size="large" color="brand" appearance="filled">
          全部文章
        </Badge>
        <Badge size="large" appearance="ghost" style={{ cursor: 'pointer' }}>
          Rust
        </Badge>
        <Badge size="large" appearance="ghost" style={{ cursor: 'pointer' }}>
          React
        </Badge>
        <Badge size="large" appearance="ghost" style={{ cursor: 'pointer' }}>
          TypeScript
        </Badge>
        <Badge size="large" appearance="ghost" style={{ cursor: 'pointer' }}>
          Web 开发
        </Badge>
      </div>

      {/* 文章网格 */}
      {loading ? (
        <div style={{ display: 'flex', justifyContent: 'center', padding: '48px' }}>
          <Spinner size="large" />
        </div>
      ) : posts.length === 0 ? (
        <div style={{ textAlign: 'center', padding: '48px' }}>
          <Text size={500}>暂无文章</Text>
        </div>
      ) : (
        <>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(320px, 1fr))',
              gap: '24px',
              marginBottom: '32px',
            }}
          >
            {posts.map((post) => (
              <Card
                key={post.id}
                style={{
                  cursor: 'pointer',
                  transition: 'transform 0.2s, box-shadow 0.2s',
                  height: '100%',
                  display: 'flex',
                  flexDirection: 'column',
                }}
                onClick={() => navigate(`/post/${post.id}`)}
                onMouseEnter={(e) => {
                  e.currentTarget.style.transform = 'translateY(-4px)';
                  e.currentTarget.style.boxShadow = 'var(--shadow8)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.transform = 'translateY(0)';
                  e.currentTarget.style.boxShadow = 'var(--shadow4)';
                }}
              >
                <CardHeader
                  header={
                    <Text
                      weight="semibold"
                      size={500}
                      style={{
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                        display: '-webkit-box',
                        WebkitLineClamp: 2,
                        WebkitBoxOrient: 'vertical',
                      }}
                    >
                      {post.title}
                    </Text>
                  }
                  description={
                    <Caption1
                      style={{
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                        display: '-webkit-box',
                        WebkitLineClamp: 3,
                        WebkitBoxOrient: 'vertical',
                        color: 'var(--colorNeutralForeground2)',
                      }}
                    >
                      {post.content.substring(0, 150)}...
                    </Caption1>
                  }
                />

                <div style={{ flex: 1 }} />

                <div
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '16px',
                    padding: '16px',
                    borderTop: '1px solid var(--colorNeutralStroke1)',
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                    <CalendarRegular fontSize={14} />
                    <Caption1>{formatDate(post.created_at)}</Caption1>
                  </div>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
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
        </>
      )}
    </div>
  );
}
