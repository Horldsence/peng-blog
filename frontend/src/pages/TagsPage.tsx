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
} from '@fluentui/react-components';
import { TagRegular } from '@fluentui/react-icons';
import { tagsApi, postsApi } from '../api';
import type { Tag, Post } from '../types';

export function TagsPage() {
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
      <div style={{ display: 'flex', justifyContent: 'center', padding: '48px' }}>
        <Spinner size="large" />
      </div>
    );
  }

  return (
    <div>
      <div style={{ marginBottom: '32px' }}>
        <Title2>标签</Title2>
        <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
          按标签浏览文章
        </Body1>
      </div>

      {/* 标签云 */}
      <Card style={{ marginBottom: '32px', padding: '24px' }}>
        <div style={{ marginBottom: '16px', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <TagRegular />
          <Body1 style={{ fontWeight: '600' }}>标签云</Body1>
        </div>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px' }}>
          {tags.length === 0 ? (
            <Body1 style={{ color: 'var(--colorNeutralForeground3)' }}>暂无标签</Body1>
          ) : (
            tags.map((tag) => (
              <Badge
                key={tag.id}
                size="large"
                color={selectedTag === tag.id ? 'brand' : 'success'}
                appearance={selectedTag === tag.id ? 'filled' : 'ghost'}
                style={{ cursor: 'pointer', padding: '8px 16px' }}
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
        <Card>
          <CardHeader
            header={<Body1 style={{ fontWeight: '600' }}>该标签下的文章</Body1>}
            description={<Body1>{posts.length} 篇文章</Body1>}
          />
          <div>
            {posts.length === 0 ? (
              <Body1 style={{ color: 'var(--colorNeutralForeground3)', padding: '24px' }}>
                该标签下暂无文章
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
      )}
    </div>
  );
}
