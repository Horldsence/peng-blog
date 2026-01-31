import { useEffect, useRef, useState } from 'react';
import {
  Button,
  Input,
  tokens,
} from '@fluentui/react-components';
import {
  SearchRegular,
  ChevronDownRegular,
} from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { postsApi } from '../api';
import gsap from 'gsap';

// 样式对象
const styles = {
  pageContainer: {
    margin: '-32px',
    position: 'relative',
  } as React.CSSProperties,

  heroSection: {
    position: 'relative',
    height: '100vh',
    width: '100%',
    overflow: 'hidden',
  } as React.CSSProperties,

  heroBackground: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    width: '100%',
    height: '100%',
    backgroundSize: 'cover',
    backgroundPosition: 'center center',
    backgroundRepeat: 'no-repeat',
    transform: 'scale(1.05)',
    willChange: 'transform, opacity',
  } as React.CSSProperties,

  heroOverlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    background: `linear-gradient(135deg, rgba(0,0,0,0.5) 0%, rgba(0,0,0,0.2) 50%, rgba(0,0,0,0.3) 100%)`,
    zIndex: 1,
  } as React.CSSProperties,

  heroContent: {
    position: 'relative',
    zIndex: 2,
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'space-between',
    padding: '48px 56px',
    boxSizing: 'border-box',
  } as React.CSSProperties,

  // 搜索区域 - 右上角
  searchContainer: {
    display: 'flex',
    gap: '12px',
    alignItems: 'center',
    alignSelf: 'flex-end',
    opacity: 0,
    transform: 'translateY(20px)',
  } as React.CSSProperties,

  searchInput: {
    width: '300px',
    backgroundColor: 'rgba(255,255,255,0.95)',
    borderRadius: tokens.borderRadiusCircular,
    border: 'none',
    boxShadow: '0 4px 20px rgba(0,0,0,0.15)',
  } as React.CSSProperties,

  searchButton: {
    borderRadius: tokens.borderRadiusCircular,
    backgroundColor: 'rgba(255,255,255,0.95)',
    color: tokens.colorNeutralForeground1,
    boxShadow: '0 4px 20px rgba(0,0,0,0.15)',
  } as React.CSSProperties,

  // 欢迎语 - 左下角
  welcomeContainer: {
    maxWidth: '550px',
    opacity: 0,
    transform: 'translateY(30px)',
  } as React.CSSProperties,

  welcomeTitle: {
    fontSize: '56px',
    fontWeight: tokens.fontWeightBold,
    color: '#ffffff',
    margin: '0 0 20px 0',
    textShadow: '0 4px 20px rgba(0,0,0,0.4)',
    lineHeight: '1.15',
    letterSpacing: '-0.02em',
  } as React.CSSProperties,

  welcomeSubtitle: {
    fontSize: tokens.fontSizeBase500,
    color: 'rgba(255,255,255,0.9)',
    margin: '0 0 24px 0',
    textShadow: '0 2px 10px rgba(0,0,0,0.3)',
    lineHeight: '1.7',
    maxWidth: '450px',
  } as React.CSSProperties,

  statsRow: {
    display: 'flex',
    gap: '32px',
    opacity: 0,
  } as React.CSSProperties,

  statItem: {
    display: 'flex',
    flexDirection: 'column',
  } as React.CSSProperties,

  statNumber: {
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightBold,
    color: '#ffffff',
    textShadow: '0 2px 10px rgba(0,0,0,0.3)',
  } as React.CSSProperties,

  statLabel: {
    fontSize: tokens.fontSizeBase200,
    color: 'rgba(255,255,255,0.7)',
    marginTop: '4px',
  } as React.CSSProperties,

  // 滚动指示器
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
  } as React.CSSProperties,

  scrollText: {
    fontSize: tokens.fontSizeBase200,
    textShadow: '0 1px 4px rgba(0,0,0,0.3)',
    fontWeight: tokens.fontWeightSemibold,
    letterSpacing: '0.5px',
  } as React.CSSProperties,
};

export function HomePage() {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState('');
  const [bingImage, setBingImage] = useState<string>('');
  const [imageLoaded, setImageLoaded] = useState(false);
  const [postsCount, setPostsCount] = useState<number>(0);

  const pageRef = useRef<HTMLDivElement>(null);
  const heroBgRef = useRef<HTMLDivElement>(null);
  const searchRef = useRef<HTMLDivElement>(null);
  const welcomeRef = useRef<HTMLDivElement>(null);
  const statsRef = useRef<HTMLDivElement>(null);
  const scrollIndicatorRef = useRef<HTMLDivElement>(null);

  // 获取Bing每日一图
  useEffect(() => {
    const fetchBingImage = async () => {
      try {
        const response = await fetch('https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1');
        const data = await response.json();
        if (data.images && data.images.length > 0) {
          const imageUrl = `https://www.bing.com${data.images[0].url}`;
          const img = new Image();
          img.onload = () => {
            setBingImage(imageUrl);
            setImageLoaded(true);
          };
          img.src = imageUrl;
        }
      } catch (error) {
        console.error('Failed to fetch Bing image:', error);
        setBingImage('https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=1920&q=80');
        setImageLoaded(true);
      }
    };

    fetchBingImage();
  }, []);

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
    if (!imageLoaded) return;

    const ctx = gsap.context(() => {
      // Hero背景缩放动画 - 更加平滑
      if (heroBgRef.current) {
        gsap.fromTo(heroBgRef.current,
          { scale: 1.15, opacity: 0 },
          { scale: 1.05, opacity: 1, duration: 1.8, ease: 'power2.inOut' }
        );
      }

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
  }, [imageLoaded]);

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
    <div ref={pageRef} style={styles.pageContainer}>
      {/* Hero Section */}
      <section style={styles.heroSection}>
        {/* 背景图 */}
        <div
          ref={heroBgRef}
          style={{
            ...styles.heroBackground,
            backgroundImage: bingImage ? `url(${bingImage})` : undefined,
            opacity: 0,
          }}
        />
        <div style={styles.heroOverlay} />

        {/* Hero内容 */}
        <div style={styles.heroContent}>
          {/* 搜索框 - 右上角 */}
          <div ref={searchRef} style={styles.searchContainer}>
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
              style={styles.searchButton}
              size="large"
            >
              搜索
            </Button>
          </div>

          {/* 欢迎语 - 左下角 */}
          <div ref={welcomeRef} style={styles.welcomeContainer}>
            <h1 style={styles.welcomeTitle}>
              欢迎来到<br />Peng Blog
            </h1>
            <p style={styles.welcomeSubtitle}>
              探索技术文章、教程和见解，记录学习，分享成长
            </p>
            <div ref={statsRef} style={styles.statsRow}>
              <div style={styles.statItem}>
                <span style={styles.statNumber}>{postsCount}+</span>
                <span style={styles.statLabel}>文章</span>
              </div>
              <div style={styles.statItem}>
                <span style={styles.statNumber}>∞</span>
                <span style={styles.statLabel}>灵感</span>
              </div>
              <div style={styles.statItem}>
                <span style={styles.statNumber}>24/7</span>
                <span style={styles.statLabel}>更新</span>
              </div>
            </div>
          </div>
        </div>

        {/* 滚动指示器 */}
        <div
          ref={scrollIndicatorRef}
          style={styles.scrollIndicator}
          onClick={handleScrollToPosts}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = 'rgba(255,255,255,0.1)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = 'transparent';
          }}
        >
          <span style={styles.scrollText}>浏览文章</span>
          <ChevronDownRegular className="scroll-arrow" style={{ fontSize: '24px' }} />
        </div>
      </section>
    </div>
  );
}
