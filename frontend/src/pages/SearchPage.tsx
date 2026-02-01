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
  tokens,
  makeStyles,
} from '@fluentui/react-components';
import { SearchRegular, CalendarRegular, EyeRegular } from '@fluentui/react-icons';
import { postsApi } from '../api';
import { useToast } from '../components/ui/Toast';
import type { Post } from '../types';

const useStyles = makeStyles({
  container: {
    maxWidth: '800px',
    margin: '0 auto',
    padding: '32px',
  },
  header: {
    marginBottom: '32px',
  },
  subtitle: {
    color: tokens.colorNeutralForeground2,
    marginTop: '8px',
  },
  searchBoxContainer: {
    marginBottom: '32px',
  },
  searchInput: {
    width: '100%',
  },
  loadingContainer: {
    display: 'flex',
    justifyContent: 'center',
    padding: '48px',
    animationName: 'fadeIn',
    animationDuration: '0.3s',
    animationTimingFunction: 'ease-in',
  },
  loadingText: {
    marginLeft: '16px',
  },
  successContainer: {
    padding: '16px',
    marginBottom: '24px',
    backgroundColor: tokens.colorPaletteGreenBackground1,
    border: `1px solid ${tokens.colorPaletteGreenBorder1}`,
    borderRadius: tokens.borderRadiusLarge,
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    animationName: 'slideDown',
    animationDuration: '0.4s',
    animationTimingFunction: 'ease-out',
  },
  successIcon: {
    fontSize: '20px',
  },
  successText: {
    color: tokens.colorPaletteGreenForeground1,
    fontWeight: tokens.fontWeightMedium,
  },
  emptyCard: {
    padding: '48px',
    textAlign: 'center',
    borderRadius: tokens.borderRadiusLarge,
  },
  emptyText: {
    fontSize: '16px',
    color: tokens.colorNeutralForeground2,
  },
  emptySubtext: {
    marginTop: '8px',
    color: tokens.colorNeutralForeground3,
  },
  resultsHeader: {
    marginBottom: '16px',
  },
  resultsCount: {
    color: tokens.colorNeutralForeground2,
  },
  resultsList: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
  },
  resultCard: {
    cursor: 'pointer',
    transition: 'transform 0.2s, box-shadow 0.2s',
    borderRadius: tokens.borderRadiusLarge,
    ':hover': {
      transform: 'translateY(-2px)',
      boxShadow: tokens.shadow8,
    },
  },
  resultTitle: {
    fontWeight: tokens.fontWeightSemibold,
    fontSize: '18px',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 2,
    WebkitBoxOrient: 'vertical',
  },
  resultDescription: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 3,
    WebkitBoxOrient: 'vertical',
    color: tokens.colorNeutralForeground2,
    lineHeight: '1.6',
  },
  resultMeta: {
    display: 'flex',
    gap: '16px',
    marginTop: '12px',
    alignItems: 'center',
  },
  metaItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    color: tokens.colorNeutralForeground3,
  },
  metaText: {
    fontSize: '14px',
  },
  tipsCard: {
    padding: '32px',
    borderRadius: tokens.borderRadiusLarge,
  },
  tipsTitle: {
    fontWeight: tokens.fontWeightSemibold,
    marginBottom: '16px',
  },
  tipsList: {
    margin: '0',
    paddingLeft: '20px',
    color: tokens.colorNeutralForeground2,
  },
  highlight: {
    backgroundColor: tokens.colorBrandBackground2,
    color: tokens.colorBrandForeground1,
    padding: '2px 4px',
    borderRadius: '2px',
  },
});

export function SearchPage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const toast = useToast();
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<Post[]>([]);
  const [searching, setSearching] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);
  const [showSuccessAnimation, setShowSuccessAnimation] = useState(false);

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
      setShowSuccessAnimation(false);

      const response = await postsApi.searchPosts({
        q: searchQuery,
        page: 1,
        per_page: 20,
      });
      setResults(response.data);

      if (response.data.length > 0) {
        setShowSuccessAnimation(true);
        setTimeout(() => setShowSuccessAnimation(false), 2000);
        toast.showSuccess(`找到 ${response.data.length} 篇相关文章`);
      } else {
        toast.showInfo('未找到相关文章，请尝试其他关键词');
      }
    } catch (error) {
      console.error('Search failed:', error);
      toast.showError('搜索失败，请稍后重试');
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
        <mark key={index} className={styles.highlight}>
          {part}
        </mark>
      ) : (
        part
      )
    );
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <Title2>搜索文章</Title2>
        <div className={styles.subtitle}>
          <Body1>输入关键词搜索相关内容</Body1>
        </div>
      </div>

      {/* 搜索框 */}
      <div className={styles.searchBoxContainer}>
        <Input
          size="large"
          placeholder="搜索文章标题、内容..."
          value={query}
          onChange={(_, data) => setQuery(data.value)}
          contentBefore={<SearchRegular />}
          autoFocus
          className={styles.searchInput}
        />
      </div>

      {/* 搜索状态 */}
      {searching && (
        <div className={styles.loadingContainer}>
          <Spinner size="large" />
          <Body1 className={styles.loadingText}>搜索中...</Body1>
        </div>
      )}

      {/* 搜索成功提示 */}
      {showSuccessAnimation && !searching && hasSearched && results.length > 0 && (
        <div className={styles.successContainer}>
          <span className={styles.successIcon}>✨</span>
          <Body1 className={styles.successText}>
            找到 <strong>{results.length}</strong> 篇相关文章
          </Body1>
        </div>
      )}

      {/* 搜索结果 */}
      {!searching && hasSearched && (
        <>
          {results.length === 0 ? (
            <Card className={styles.emptyCard}>
              <Body1 className={styles.emptyText}>未找到与 "{query}" 相关的文章</Body1>
              <Body1 className={styles.emptySubtext}>请尝试其他关键词</Body1>
            </Card>
          ) : (
            <>
              <div className={styles.resultsHeader}>
                <Body1 className={styles.resultsCount}>
                  找到 <strong>{results.length}</strong> 篇文章
                </Body1>
              </div>

              <div className={styles.resultsList}>
                {results.map((post) => (
                  <Card
                    key={post.id}
                    className={styles.resultCard}
                    onClick={() => navigate(`/post/${post.id}`)}
                  >
                    <CardHeader
                      header={
                        <Body1 className={styles.resultTitle}>
                          {highlightMatch(post.title, query)}
                        </Body1>
                      }
                      description={
                        <div>
                          <Body1 className={styles.resultDescription}>
                            {highlightMatch(post.content.substring(0, 200), query)}
                          </Body1>

                          <div className={styles.resultMeta}>
                            <div className={styles.metaItem}>
                              <CalendarRegular fontSize={14} />
                              <Body1 className={styles.metaText}>
                                {formatDate(post.created_at)}
                              </Body1>
                            </div>
                            <div className={styles.metaItem}>
                              <EyeRegular fontSize={14} />
                              <Body1 className={styles.metaText}>{post.views}</Body1>
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
        <Card className={styles.tipsCard}>
          <Body1 className={styles.tipsTitle}>搜索提示</Body1>
          <ul className={styles.tipsList}>
            <li>搜索文章标题和内容</li>
            <li>支持关键词模糊匹配</li>
            <li>搜索结果会高亮匹配内容</li>
          </ul>
        </Card>
      )}
    </div>
  );
}
