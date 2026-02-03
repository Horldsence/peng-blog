/**
 * 标签页面 - 标签云和文章过滤
 */

import { useEffect, useState } from 'react';
import {
  Badge,
  Card,
  Body1,
  Title2,
  Spinner,
  tokens,
  makeStyles,
} from '@fluentui/react-components';
import { TagRegular } from '@fluentui/react-icons';
import { tagsApi, postsApi } from '../api';
import type { Tag, Post } from '../types';
import { PostCard } from '../components/features/PostCard';

const useStyles = makeStyles({
  container: {
    display: 'flex',
    gap: '48px',
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '48px 24px',
    ['@media (max-width: 768px)']: {
      flexDirection: 'column',
    },
  },
  sidebar: {
    flex: '0 0 300px',
  },
  mainContent: {
    flex: '1',
  },
  header: {
    marginBottom: '24px',
  },
  subtitle: {
    color: tokens.colorNeutralForeground2,
    marginTop: '8px',
  },
  tagCloudCard: {
    padding: '24px',
    borderRadius: tokens.borderRadiusLarge,
    backgroundColor: 'rgba(255, 255, 255, 0.5)',
    border: 'none',
  },
  tagHeader: {
    marginBottom: '16px',
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
  },
  tagTitle: {
    fontWeight: tokens.fontWeightSemibold,
  },
  tagsContainer: {
    display: 'flex',
    flexWrap: 'wrap',
    gap: '8px',
  },
  tagBadge: {
    cursor: 'pointer',
    padding: '6px 12px',
    transition: 'all 0.2s',
    ':hover': {
      transform: 'scale(1.05)',
    },
  },
  postsList: {
    display: 'flex',
    flexDirection: 'column',
    gap: '24px',
  },
  tagContentHeader: {
    marginBottom: '32px',
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
});

export function TagsPage() {
  const styles = useStyles();
  const [tags, setTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    void fetchTags();
  }, []);

  useEffect(() => {
    if (selectedTag) {
      void fetchPostsByTag(selectedTag);
    }
  }, [selectedTag]);

  const fetchTags = async () => {
    try {
      setLoading(true);
      const response = await tagsApi.getTags();
      setTags(response.data);
    } catch (error) {
      console.error('Failed to fetch tags:', error);
    } finally {
      setLoading(false);
    }
  };

  const fetchPostsByTag = async (tagId: string) => {
    try {
      const response = await postsApi.getPosts({
        tag: tagId,
        page: 1,
        per_page: 10,
      });
      setPosts(response.data);
    } catch (error) {
      console.error('Failed to fetch posts:', error);
    }
  };

  const handleTagClick = (tag: Tag) => {
    setSelectedTag(tag.id);
  };

  if (loading) {
    return (
      <div className={styles.loadingContainer}>
        <Spinner size="large" />
      </div>
    );
  }

  const currentTagName = tags.find((t) => t.id === selectedTag)?.name;

  return (
    <div className={styles.container}>
      {/* 左侧标签云 */}
      <div className={styles.sidebar}>
        <div className={styles.header}>
          <Title2>标签</Title2>
          <div className={styles.subtitle}>
            <Body1>按标签浏览文章</Body1>
          </div>
        </div>

        <Card className={styles.tagCloudCard}>
          <div className={styles.tagHeader}>
            <TagRegular />
            <Body1 className={styles.tagTitle}>所有标签</Body1>
          </div>
          <div className={styles.tagsContainer}>
            {tags.length === 0 ? (
              <Body1 className={styles.emptyText}>暂无标签</Body1>
            ) : (
              tags.map((tag) => (
                <Badge
                  key={tag.id}
                  size="large"
                  color={selectedTag === tag.id ? 'brand' : 'success'}
                  appearance={selectedTag === tag.id ? 'filled' : 'ghost'}
                  className={styles.tagBadge}
                  onClick={() => handleTagClick(tag)}
                >
                  {tag.name}
                </Badge>
              ))
            )}
          </div>
        </Card>
      </div>

      {/* 右侧文章列表 */}
      {selectedTag && (
        <div className={styles.mainContent}>
          <div className={styles.tagContentHeader}>
            <Title2>#{currentTagName}</Title2>
            <div className={styles.subtitle}>{posts.length} 篇文章</div>
          </div>

          <div className={styles.postsList}>
            {posts.length === 0 ? (
              <div className={styles.emptyText}>该标签下暂无文章</div>
            ) : (
              posts.map((post) => <PostCard key={post.id} post={post} />)
            )}
          </div>
        </div>
      )}
    </div>
  );
}
