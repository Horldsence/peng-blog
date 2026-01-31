/**
 * 搜索页面 - 实时搜索和结果展示
 */

import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Input,
  Card,
  CardHeader,
  Body1,
  Title2,
  Spinner,
} from '@fluentui/react-components';
import {
  SearchRegular,
  CalendarRegular,
  EyeRegular,
} from '@fluentui/react-icons';
import { postsApi } from '../api';
import type { Post } from '../types';

export function SearchPage() {
  const navigate = useNavigate();
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<Post[]>([]);
  const [searching, setSearching] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (query.trim()) {
        performSearch(query);
      } else {
        setResults([]);
        setHasSearched(false);
      }
    }, 300);

    return () => clearTimeout(timeoutId);
  }, [query]);

  const performSearch = async (searchQuery: string) => {
    try {
      setSearching(true);
      setHasSearched(true);
      const response = await postsApi.searchPosts({
        q: searchQuery,
        page: 1,
        per_page: 20,
      });
      setResults(response.data);
    } catch (error) {
      console.error('Search failed:', error);
      setResults([]);
    } finally {
      setSearching(false);
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

  const highlightMatch = (text: string, query: string) => {
    if (!query.trim()) return text;

    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
    const parts = text.split(regex);

    return parts.map((part, index) =>
      regex.test(part) ? (
        <mark
          key={index}
          style={{
            backgroundColor: 'var(--colorBrandBackground1)',
            color: 'var(--colorBrandForeground1)',
            padding: '2px 4px',
            borderRadius: '2px',
          }}
        >
          {part}
        </mark>
      ) : (
        part
      )
    );
  };

  return (
    <div style={{ maxWidth: '800px', margin: '0 auto' }}>
      <div style={{ marginBottom: '32px' }}>
        <Title2>搜索文章</Title2>
        <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
          输入关键词搜索相关内容
        </Body1>
      </div>

      {/* 搜索框 */}
      <div style={{ marginBottom: '32px' }}>
        <Input
          size="large"
          placeholder="搜索文章标题、内容..."
          value={query}
          onChange={(_, data) => setQuery(data.value)}
          contentBefore={<SearchRegular />}
          autoFocus
          style={{ width: '100%' }}
        />
      </div>

      {/* 搜索状态 */}
      {searching && (
        <div style={{ display: 'flex', justifyContent: 'center', padding: '48px' }}>
          <Spinner size="large" />
          <Body1 style={{ marginLeft: '16px' }}>搜索中...</Body1>
        </div>
      )}

      {/* 搜索结果 */}
      {!searching && hasSearched && (
        <>
          {results.length === 0 ? (
            <Card style={{ padding: '48px', textAlign: 'center' }}>
              <Body1 style={{ fontSize: '16px', color: 'var(--colorNeutralForeground2)' }}>
                未找到与 "{query}" 相关的文章
              </Body1>
              <Body1 style={{ marginTop: '8px', color: 'var(--colorNeutralForeground3)' }}>
                请尝试其他关键词
              </Body1>
            </Card>
          ) : (
            <>
              <div style={{ marginBottom: '16px' }}>
                <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
                  找到 <strong>{results.length}</strong> 篇文章
                </Body1>
              </div>

              <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                {results.map((post) => (
                  <Card
                    key={post.id}
                    style={{
                      cursor: 'pointer',
                      transition: 'transform 0.2s, box-shadow 0.2s',
                    }}
                    onClick={() => navigate(`/post/${post.id}`)}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.transform = 'translateY(-2px)';
                      e.currentTarget.style.boxShadow = 'var(--shadow8)';
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.transform = 'translateY(0)';
                      e.currentTarget.style.boxShadow = 'var(--shadow4)';
                    }}
                  >
                    <CardHeader
                      header={
                        <Body1
                          style={{
                            fontWeight: '600',
                            fontSize: '18px',
                            overflow: 'hidden',
                            textOverflow: 'ellipsis',
                            display: '-webkit-box',
                            WebkitLineClamp: 2,
                            WebkitBoxOrient: 'vertical',
                          }}
                        >
                          {highlightMatch(post.title, query)}
                        </Body1>
                      }
                      description={
                        <div>
                          <Body1
                            style={{
                              overflow: 'hidden',
                              textOverflow: 'ellipsis',
                              display: '-webkit-box',
                              WebkitLineClamp: 3,
                              WebkitBoxOrient: 'vertical',
                              color: 'var(--colorNeutralForeground2)',
                              lineHeight: '1.6',
                            }}
                          >
                            {highlightMatch(post.content.substring(0, 200), query)}
                          </Body1>

                          <div
                            style={{
                              display: 'flex',
                              gap: '16px',
                              marginTop: '12px',
                              alignItems: 'center',
                            }}
                          >
                            <div
                              style={{
                                display: 'flex',
                                alignItems: 'center',
                                gap: '4px',
                                color: 'var(--colorNeutralForeground3)',
                              }}
                            >
                              <CalendarRegular fontSize={14} />
                              <Body1 style={{ fontSize: '14px' }}>
                                {formatDate(post.created_at)}
                              </Body1>
                            </div>
                            <div
                              style={{
                                display: 'flex',
                                alignItems: 'center',
                                gap: '4px',
                                color: 'var(--colorNeutralForeground3)',
                              }}
                            >
                              <EyeRegular fontSize={14} />
                              <Body1 style={{ fontSize: '14px' }}>{post.views}</Body1>
                            </div>
                          </div>
                        </div>
                      }
                    />
                  </Card>
                ))}
              </div>
            </>
          )}
        </>
      )}

      {/* 搜索提示 */}
      {!hasSearched && !query && (
        <Card style={{ padding: '32px' }}>
          <Body1 style={{ fontWeight: '600', marginBottom: '16px' }}>
            搜索提示
          </Body1>
          <ul style={{ margin: 0, paddingLeft: '20px', color: 'var(--colorNeutralForeground2)' }}>
            <li>搜索文章标题和内容</li>
            <li>支持关键词模糊匹配</li>
            <li>搜索结果会高亮匹配内容</li>
          </ul>
        </Card>
      )}
    </div>
  );
}
