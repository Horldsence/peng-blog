/**
 * 标签页面 - 标签云和文章过滤
 */

import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Badge,
  Card,
  CardHeader,
  Body1,
  Title2,
  Spinner,
  tokens,
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import { TagRegular } from '@fluentui/react-icons';
import { tagsApi, postsApi } from '../api';
import type { Tag, Post } from '../types';

const useStyles = makeStyles({
  container: {
    padding: '32px',
    maxWidth: '1200px',
    margin: '0 auto',
  },
  header: {
    marginBottom: '32px',
  },
  subtitle: {
    color: tokens.colorNeutralForeground2,
    marginTop: '8px',
  },
  tagCloudCard: {
    marginBottom: '32px',
    padding: '24px',
    borderRadius: tokens.borderRadiusLarge,
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
    gap: '12px',
  },
  emptyText: {
    color: tokens.colorNeutralForeground3,
  },
  tagBadge: {
    cursor: 'pointer',
    padding: '8px 16px',
  },
  postsCard: {
    borderRadius: tokens.borderRadiusLarge,
  },
  postsHeader: {
    fontWeight: tokens.fontWeightSemibold,
  },
  emptyPosts: {
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

export function TagsPage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const [tags, setTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    fetchTags();
  }, []);

  useEffect(() => {
    if (selectedTag) {
      fetchPostsByTag(selectedTag);
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

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <Title2>标签</Title2>
        <div className={styles.subtitle}>
          <Body1>按标签浏览文章</Body1>
        </div>
      </div>

      {/* 标签云 */}
      <Card className={styles.tagCloudCard}>
        <div className={styles.tagHeader}>
          <TagRegular />
          <Body1 className={styles.tagTitle}>标签云</Body1>
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

      {/* 该标签的文章 */}
      {selectedTag && (
        <Card className={styles.postsCard}>
          <CardHeader
            header={<Body1 className={styles.postsHeader}>该标签下的文章</Body1>}
            description={<Body1>{posts.length} 篇文章</Body1>}
          />
          <div>
            {posts.length === 0 ? (
              <Body1 className={styles.emptyPosts}>
                该标签下暂无文章
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
      )}
    </div>
  );
}
