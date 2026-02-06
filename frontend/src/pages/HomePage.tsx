import { useEffect, useState } from 'react';
import { Button, Input, tokens, makeStyles, mergeClasses } from '@fluentui/react-components';
import { SearchRegular, ChevronDownRegular } from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { postsApi } from '../api';
import { useTheme } from '../contexts/ThemeContext';
import { motion } from 'framer-motion';

const useStyles = makeStyles({
  pageContainer: {
    position: 'relative',
    height: '100%',
  },
  heroSection: {
    position: 'relative',
    height: '100%',
    width: '100%',
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'center',
  },
  heroOverlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    background: `radial-gradient(circle at 0% 100%, ${tokens.colorNeutralBackground1} 0%, transparent 60%)`,
    zIndex: 0,
    pointerEvents: 'none',
  },
  heroContent: {
    position: 'relative',
    zIndex: 2,
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'space-between',
    padding: '32px 42px',
    boxSizing: 'border-box',
    '@media (max-width: 768px)': {
      padding: '24px 20px',
    },
  },
  searchContainer: {
    display: 'flex',
    gap: '12px',
    alignItems: 'center',
    alignSelf: 'flex-end',
    '@media (max-width: 768px)': {
      width: '100%',
      justifyContent: 'center',
    },
  },
  searchInput: {
    width: '300px',
    '@media (max-width: 768px)': {
      width: '100%',
      flex: 1,
    },
    backgroundColor: 'rgba(255,255,255,0.8)',
    borderRadius: tokens.borderRadiusCircular,
    border: '1px solid rgba(255,255,255,0.3)',
    backdropFilter: 'blur(10px)',
    boxShadow: '0 4px 20px rgba(0,0,0,0.1)',
    '& input': {
      backgroundColor: 'transparent',
    },
  },
  searchInputDark: {
    backgroundColor: 'rgba(0, 0, 0, 0.6)',
    border: '1px solid rgba(255, 255, 255, 0.1)',
    boxShadow: '0 4px 20px rgba(0,0,0,0.3)',
  },
  searchButton: {
    borderRadius: tokens.borderRadiusCircular,
    border: '1px solid rgba(255,255,255,0.3)',
    boxShadow: '0 4px 20px rgba(0,0,0,0.15)',
    zIndex: 2,
  },
  welcomeContainer: {
    maxWidth: '550px',
    padding: '16px 0',
  },
  welcomeTitle: {
    fontSize: '64px',
    fontWeight: tokens.fontWeightBold,
    color: tokens.colorNeutralForeground1,
    margin: '0 0 24px 0',
    lineHeight: '1.1',
    letterSpacing: '-0.02em',
    filter: 'drop-shadow(0 2px 10px rgba(0,0,0,0.1))',
    '@media (max-width: 768px)': {
      fontSize: '42px',
    },
  },
  welcomeSubtitle: {
    fontSize: tokens.fontSizeBase500,
    color: tokens.colorNeutralForeground2,
    margin: '0 0 24px 0',
    textShadow: '0 2px 10px rgba(0,0,0,0.1)',
    lineHeight: '1.7',
    maxWidth: '450px',
  },
  statsRow: {
    display: 'flex',
    gap: '32px',
    '@media (max-width: 768px)': {
      gap: '24px',
    },
  },
  statItem: {
    display: 'flex',
    flexDirection: 'column',
  },
  statNumber: {
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightBold,
    color: tokens.colorNeutralForeground1,
    textShadow: '0 2px 10px rgba(0,0,0,0.1)',
  },
  statLabel: {
    fontSize: tokens.fontSizeBase200,
    color: tokens.colorNeutralForeground3,
    marginTop: '4px',
  },
  scrollIndicator: {
    position: 'absolute',
    bottom: '40px',
    left: '50%',
    transform: 'translateX(-50%)',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: '8px',
    color: tokens.colorNeutralForeground3,
    cursor: 'pointer',
    zIndex: 3,
    padding: '12px 20px',
    borderRadius: tokens.borderRadiusCircular,
    transition: 'background-color 0.3s ease',
    ':hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    },
  },
  scrollText: {
    fontSize: tokens.fontSizeBase200,
    textShadow: '0 1px 4px rgba(0,0,0,0.3)',
    fontWeight: tokens.fontWeightSemibold,
    letterSpacing: '0.5px',
  },
  scrollArrow: {
    fontSize: '24px',
  },
});

export function HomePage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const { mode } = useTheme();
  const [searchQuery, setSearchQuery] = useState('');
  const [postsCount, setPostsCount] = useState<number>(0);
  const [lastUpdate, setLastUpdate] = useState<string>('-');

  useEffect(() => {
    const fetchPostsCount = async () => {
      try {
        const response = await postsApi.getPosts({
          page: 1,
          per_page: 1,
          status: 'published',
        });
        setPostsCount(response.pagination?.total || 0);

        if (response.data && response.data.length > 0) {
          const latestPost = response.data[0];
          const updateDate = new Date(
            latestPost.updated_at ?? latestPost.published_at ?? latestPost.created_at
          );
          const now = new Date();
          const diffMs = now.getTime() - updateDate.getTime();
          const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

          if (diffDays === 0) {
            const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
            if (diffHours === 0) {
              const diffMins = Math.floor(diffMs / (1000 * 60));
              setLastUpdate(diffMins <= 1 ? '刚刚' : `${diffMins}分钟前`);
            } else {
              setLastUpdate(`${diffHours}小时前`);
            }
          } else if (diffDays === 1) {
            setLastUpdate('昨天');
          } else if (diffDays < 7) {
            setLastUpdate(`${diffDays}天前`);
          } else if (diffDays < 30) {
            const weeks = Math.floor(diffDays / 7);
            setLastUpdate(`${weeks}周前`);
          } else if (diffDays < 365) {
            const months = Math.floor(diffDays / 30);
            setLastUpdate(`${months}个月前`);
          } else {
            const years = Math.floor(diffDays / 365);
            setLastUpdate(`${years}年前`);
          }
        }
      } catch (error) {
        console.error('Failed to fetch posts count:', error);
      }
    };

    void fetchPostsCount();
  }, []);

  const handleSearch = () => {
    if (searchQuery.trim()) {
      navigate(`/posts?q=${encodeURIComponent(searchQuery)}`);
    } else {
      navigate('/posts');
    }
  };

  const handleScrollToPosts = () => {
    navigate('/posts');
  };

  return (
    <div className={styles.pageContainer}>
      {/* Hero Section */}
      <section className={styles.heroSection}>
        <div className={styles.heroOverlay} />

        {/* Hero内容 */}
        <div className={styles.heroContent}>
          {/* 搜索框 - 右上角 */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.3, ease: 'easeOut' }}
            className={styles.searchContainer}
          >
            <Input
              placeholder="搜索文章..."
              value={searchQuery}
              onChange={(_, data) => setSearchQuery(data.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              className={mergeClasses(
                styles.searchInput,
                mode === 'dark' && styles.searchInputDark
              )}
              contentBefore={<SearchRegular />}
              size="large"
            />
            <Button
              appearance="primary"
              onClick={handleSearch}
              className={styles.searchButton}
              size="large"
            >
              搜索
            </Button>
          </motion.div>

          {/* 欢迎语 - 左下角 */}
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 1, delay: 0.5, ease: 'easeOut' }}
            className={styles.welcomeContainer}
          >
            <h1 className={styles.welcomeTitle}>
              欢迎来到
              <br />
              Peng Blog
            </h1>
            <p className={styles.welcomeSubtitle}>探索技术文章、教程和见解，记录学习，分享成长</p>
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.8, delay: 0.8, ease: 'easeOut' }}
              className={styles.statsRow}
            >
              <div className={styles.statItem}>
                <span className={styles.statNumber}>{postsCount}+</span>
                <span className={styles.statLabel}>文章</span>
              </div>
              <div className={styles.statItem}>
                <span className={styles.statNumber}>∞</span>
                <span className={styles.statLabel}>灵感</span>
              </div>
              <div className={styles.statItem}>
                <span className={styles.statNumber}>{lastUpdate}</span>
                <span className={styles.statLabel}>更新</span>
              </div>
            </motion.div>
          </motion.div>
        </div>

        {/* 滚动指示器 */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.8, delay: 1.2, ease: 'easeOut' }}
          className={styles.scrollIndicator}
          onClick={handleScrollToPosts}
        >
          <span className={styles.scrollText}>浏览文章</span>
          <motion.div
            animate={{ y: [0, 8, 0] }}
            transition={{
              duration: 1.6,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          >
            <ChevronDownRegular className={mergeClasses('scroll-arrow', styles.scrollArrow)} />
          </motion.div>
        </motion.div>
      </section>
    </div>
  );
}
