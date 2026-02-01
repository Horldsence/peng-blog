import { useEffect, useRef, useState } from 'react';
import {
  Button,
  Input,
  tokens,
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import {
  SearchRegular,
  ChevronDownRegular,
} from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { postsApi } from '../api';
import gsap from 'gsap';

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
    justifyContent: 'center', // Center content vertically
  },
  // Removed heroBackground
  heroOverlay: {
    // Only needed if we want to darken the global background specifically for home,
    // otherwise relies on MainLayout's acrylic or just transparent.
    // Let's keep a slight gradient for text readability if the acrylic isn't enough
    // or if we want that 'hero' look.
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    // Lighter gradient since we have acrylic in MainLayout
    background: 'linear-gradient(135deg, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0.05) 50%, rgba(0,0,0,0.1) 100%)',
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
    padding: '48px 56px',
    boxSizing: 'border-box',
  },
  searchContainer: {
    display: 'flex',
    gap: '12px',
    alignItems: 'center',
    alignSelf: 'flex-end',
    opacity: 0,
    transform: 'translateY(20px)',
  },
  searchInput: {
    width: '300px',
    backgroundColor: 'rgba(255,255,255,0.8)', // Slightly more transparent
    borderRadius: tokens.borderRadiusCircular,
    border: '1px solid rgba(255,255,255,0.3)',
    backdropFilter: 'blur(10px)',
    boxShadow: '0 4px 20px rgba(0,0,0,0.1)',
    '& input': {
      backgroundColor: 'transparent',
    }
  },
  searchButton: {
    borderRadius: tokens.borderRadiusCircular,
    border: '1px solid rgba(255,255,255,0.3)',
    // Removed fixed backgroundColor to allow 'primary' appearance (theme color)
    boxShadow: '0 4px 20px rgba(0,0,0,0.15)',
    zIndex: 2,
  },
  welcomeContainer: {
    maxWidth: '550px',
    opacity: 0,
    transform: 'translateY(30px)',
    padding: '32px 0',
  },
  welcomeTitle: {
    fontSize: '64px',
    fontWeight: tokens.fontWeightBold,
    color: 'transparent',
    margin: '0 0 24px 0',
    lineHeight: '1.1',
    letterSpacing: '-0.02em',
    backgroundImage: 'linear-gradient(180deg, rgba(255,255,255,1) 0%, rgba(255,255,255,0.7) 100%)',
    WebkitBackgroundClip: 'text',
    backgroundClip: 'text',
    filter: 'drop-shadow(0 2px 10px rgba(0,0,0,0.3))',
  },
  welcomeSubtitle: {
    fontSize: tokens.fontSizeBase500,
    color: 'rgba(255,255,255,0.9)',
    margin: '0 0 24px 0',
    textShadow: '0 2px 10px rgba(0,0,0,0.3)',
    lineHeight: '1.7',
    maxWidth: '450px',
  },
  statsRow: {
    display: 'flex',
    gap: '32px',
    opacity: 0,
  },
  statItem: {
    display: 'flex',
    flexDirection: 'column',
  },
  statNumber: {
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightBold,
    color: '#ffffff',
    textShadow: '0 2px 10px rgba(0,0,0,0.3)',
  },
  statLabel: {
    fontSize: tokens.fontSizeBase200,
    color: 'rgba(255,255,255,0.7)',
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
    color: 'rgba(255,255,255,0.8)',
    cursor: 'pointer',
    zIndex: 3,
    opacity: 0,
    padding: '12px 20px',
    borderRadius: tokens.borderRadiusCircular,
    transition: 'background-color 0.3s ease',
    ':hover': {
      backgroundColor: 'rgba(255,255,255,0.1)',
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
  }
});

export function HomePage() {
  const styles = useStyles();
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState('');
  const [postsCount, setPostsCount] = useState<number>(0);

  const pageRef = useRef<HTMLDivElement>(null);
  const searchRef = useRef<HTMLDivElement>(null);
  const welcomeRef = useRef<HTMLDivElement>(null);
  const statsRef = useRef<HTMLDivElement>(null);
  const scrollIndicatorRef = useRef<HTMLDivElement>(null);

  // 获取文章数量统计
  useEffect(() => {
    const fetchPostsCount = async () => {
      try {
        const response = await postsApi.getPosts({
          page: 1,
          per_page: 1,
        });
        setPostsCount(response.pagination?.total || 0);
      } catch (error) {
        console.error('Failed to fetch posts count:', error);
      }
    };

    fetchPostsCount();
  }, []);

  // GSAP动画
  useEffect(() => {
    const ctx = gsap.context(() => {
      // 搜索框动画
      if (searchRef.current) {
        gsap.to(searchRef.current, {
          opacity: 1,
          y: 0,
          duration: 0.8,
          delay: 0.3,
          ease: 'power3.out',
        });
      }

      // 欢迎语动画
      if (welcomeRef.current) {
        gsap.to(welcomeRef.current, {
          opacity: 1,
          y: 0,
          duration: 1,
          delay: 0.5,
          ease: 'power3.out',
        });
      }

      // 统计数字动画
      if (statsRef.current) {
        gsap.to(statsRef.current, {
          opacity: 1,
          duration: 0.8,
          delay: 0.8,
          ease: 'power2.out',
        });
      }

      // 滚动指示器动画
      if (scrollIndicatorRef.current) {
        gsap.to(scrollIndicatorRef.current, {
          opacity: 1,
          duration: 0.8,
          delay: 1.2,
          ease: 'power2.out',
        });

        // 持续跳动动画
        gsap.to(scrollIndicatorRef.current.querySelector('.scroll-arrow'), {
          y: 8,
          duration: 0.8,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
        });
      }
    }, pageRef);

    return () => ctx.revert();
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
    <div ref={pageRef} className={styles.pageContainer}>
      {/* Hero Section */}
      <section className={styles.heroSection}>
        <div className={styles.heroOverlay} />

        {/* Hero内容 */}
        <div className={styles.heroContent}>
          {/* 搜索框 - 右上角 */}
          <div ref={searchRef} className={styles.searchContainer}>
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
              className={styles.searchButton}
              size="large"
            >
              搜索
            </Button>
          </div>

          {/* 欢迎语 - 左下角 */}
          <div ref={welcomeRef} className={styles.welcomeContainer}>
            <h1 className={styles.welcomeTitle}>
              欢迎来到<br />Peng Blog
            </h1>
            <p className={styles.welcomeSubtitle}>
              探索技术文章、教程和见解，记录学习，分享成长
            </p>
            <div ref={statsRef} className={styles.statsRow}>
              <div className={styles.statItem}>
                <span className={styles.statNumber}>{postsCount}+</span>
                <span className={styles.statLabel}>文章</span>
              </div>
              <div className={styles.statItem}>
                <span className={styles.statNumber}>∞</span>
                <span className={styles.statLabel}>灵感</span>
              </div>
              <div className={styles.statItem}>
                <span className={styles.statNumber}>24/7</span>
                <span className={styles.statLabel}>更新</span>
              </div>
            </div>
          </div>
        </div>

        {/* 滚动指示器 */}
        <div
          ref={scrollIndicatorRef}
          className={styles.scrollIndicator}
          onClick={handleScrollToPosts}
        >
          <span className={styles.scrollText}>浏览文章</span>
          <ChevronDownRegular className={mergeClasses("scroll-arrow", styles.scrollArrow)} />
        </div>
      </section>
    </div>
  );
}
