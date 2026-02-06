/**
 * 主布局组件 - Fluent UI 2 NavDrawer
 * 左侧导航栏 + 右侧内容区
 */

import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import {
  Avatar,
  Divider,
  tokens,
  Tooltip,
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import {
  NavDrawer,
  NavDrawerHeader,
  NavDrawerBody,
  NavDrawerFooter,
  NavItem,
  NavSectionHeader,
} from '@fluentui/react-components';
import {
  HomeRegular,
  DocumentRegular,
  TagRegular,
  FolderRegular,
  SettingsRegular,
  WeatherMoonRegular,
  WeatherSunnyFilled,
  SignOutRegular,
  ArrowEnterRegular,
  PanelLeftContractRegular,
  PanelLeftExpandRegular,
  NavigationRegular,
} from '@fluentui/react-icons';
import { useTheme } from '../../contexts/ThemeContext';
import { useConfig } from '../../contexts';
import { authApi } from '../../api';
import { LiquidGlassBackground } from '../effects/LiquidGlassBackground';

const useStyles = makeStyles({
  root: {
    display: 'flex',
    height: '100vh',
    overflow: 'hidden',
    position: 'relative',
    backgroundColor: 'transparent',
  },
  contentArea: {
    flex: '1',
    overflow: 'auto',
    backgroundColor: 'transparent',
    position: 'relative',
    zIndex: 1,
  },
  navDrawer: {
    transition: 'width 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
    backgroundColor: 'rgba(255, 255, 255, 0.6)', // Acrylic base
    backdropFilter: 'blur(20px)',
    borderRight: '1px solid rgba(0, 0, 0, 0.05)',
    zIndex: 100,
    display: 'flex',
    flexDirection: 'column',
  },
  navDrawerDark: {
    backgroundColor: 'rgba(0, 0, 0, 0.6)', // Dark Acrylic
    borderRight: '1px solid rgba(255, 255, 255, 0.1)',
  },
  globalBackground: {
    position: 'fixed',
    top: 0,
    left: 0,
    width: '100vw',
    height: '100vh',
    zIndex: 0,
    backgroundSize: 'cover',
    backgroundPosition: 'center',
    backgroundRepeat: 'no-repeat',
    transition: 'background-image 0.5s ease-in-out, filter 0.5s ease-in-out',
    pointerEvents: 'none',
  },
  globalBackgroundDark: {
    filter: 'brightness(0.5) contrast(1.1)',
  },
  navDrawerExpanded: {
    width: '260px',
  },
  navDrawerCollapsed: {
    width: '80px',
  },
  // Custom styles for NavItem
  navItem: {
    display: 'flex',
    alignItems: 'center',
    cursor: 'pointer',
    backgroundColor: 'transparent',
    width: '100%', // Ensure full width
    height: '48px', // Even taller for better touch
    padding: 0, // Reset default padding to ensure precise control
    fontSize: tokens.fontSizeBase300,
    position: 'relative', // For absolute positioning of indicator
    boxSizing: 'border-box',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    },
    '&[aria-selected="true"]': {
      backgroundColor: tokens.colorNeutralBackground1Selected,
      fontWeight: tokens.fontWeightSemibold,
      color: tokens.colorNeutralForeground1Selected,
    },
    // Hide default indicator mechanisms from Fluent UI just in case
    '&::after': { display: 'none' },
    '&::before': { display: 'none' },
    marginBottom: '2px',
  },
  activeIndicator: {
    position: 'absolute',
    left: '0', // Stick to the very edge
    top: '50%',
    transform: 'translateY(-50%)',
    width: '4px',
    height: '24px', // Pill height
    borderRadius: '0 4px 4px 0',
    backgroundColor: tokens.colorBrandForeground1,
    zIndex: 1,
  },
  navItemIcon: {
    fontSize: '24px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    minWidth: '24px', // Tighten min-width to avoid claiming too much space
    height: '24px',
    zIndex: 2, // Ensure icon is above indicator
    transition:
      'width 0.3s cubic-bezier(0.4, 0, 0.2, 1), margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
  },
  navItemContent: {
    marginLeft: '12px',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    transition: 'opacity 0.2s ease',
    opacity: 1,
    fontSize: tokens.fontSizeBase300,
  },
  navItemContentCollapsed: {
    opacity: 0,
    width: 0,
    display: 'none',
  },
  logo: {
    padding: '16px 0',
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightBold,
    color: tokens.colorNeutralForeground1,
    overflow: 'hidden',
    whiteSpace: 'nowrap',
    minHeight: '48px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  },
  logoExpanded: {
    justifyContent: 'flex-start',
    paddingLeft: '20px',
  },
  footer: {
    display: 'flex',
    flexDirection: 'column',
    gap: '2px', // Tighter gap like nav items
    padding: '16px 8px',
  },
  // Unified style for footer items to match NavItem
  mobileToggle: {
    position: 'absolute',
    top: '16px',
    left: '16px',
    zIndex: 100,
    backgroundColor: tokens.colorNeutralBackground1,
    border: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    padding: '8px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    cursor: 'pointer',
    boxShadow: tokens.shadow4,
  },
  footerItem: {
    display: 'flex',
    alignItems: 'center',
    width: '100%',
    height: '48px', // Match navItem height
    border: 'none',
    background: 'transparent',
    borderRadius: tokens.borderRadiusMedium,
    color: tokens.colorNeutralForeground1,
    cursor: 'pointer',
    position: 'relative', // For indicator
    marginBottom: '2px', // Match navItem margin
    // Remove static padding to allow precise control via inline styles
    padding: 0,
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    },
  },
  contentAcrylic: {
    backgroundColor: 'rgba(255, 255, 255, 0.7)',
    backdropFilter: 'blur(20px)',
    margin: '16px',
    borderRadius: '20px',
    border: '1px solid rgba(255, 255, 255, 0.8)',
    boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.15)',
    '@media (max-width: 768px)': {
      margin: '4px',
      paddingTop: '60px',
    },
  },
  contentAcrylicDark: {
    backgroundColor: 'rgba(0, 0, 0, 0.6)',
    backdropFilter: 'blur(20px)',
    margin: '16px',
    borderRadius: '20px',
    border: '1px solid rgba(255, 255, 255, 0.2)',
    boxShadow: '0 8px 32px 0 rgba(0, 0, 0, 0.3)',
    '@media (max-width: 768px)': {
      margin: '4px',
      paddingTop: '60px',
    },
  },
});

export function MainLayout({ children }: { children: React.ReactNode }) {
  const styles = useStyles();
  const navigate = useNavigate();
  const location = useLocation();
  const { mode, toggleTheme } = useTheme();
  const { publicConfig } = useConfig();
  const isAuthenticated = authApi.isAuthenticated();
  const currentUser = authApi.getCurrentUser();

  const [isExpanded, setIsExpanded] = useState(true);
  const [isOpen, setIsOpen] = useState(false);
  const [isMobile, setIsMobile] = useState(false);
  const [bingImage, setBingImage] = useState<string>('');

  useEffect(() => {
    const handleResize = () => {
      setIsMobile(window.innerWidth < 768);
      if (window.innerWidth >= 768) {
        setIsOpen(true);
      } else {
        setIsOpen(false);
      }
    };

    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  useEffect(() => {
    const fetchBingImage = async () => {
      try {
        const response = await fetch('/api/bing/daily-image');

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const result = (await response.json()) as { data?: { url?: string } };

        if (result?.data?.url) {
          setBingImage(result.data.url);
        } else {
          throw new Error('Invalid API response format');
        }
      } catch (error) {
        console.error('Failed to fetch Bing image:', error);
        setBingImage('https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=1920&q=80');
      }
    };
    void fetchBingImage();
  }, []);

  // Helper to determine active value, handling sub-routes
  const getSelectedValue = (pathname: string) => {
    if (pathname.startsWith('/posts') || pathname.startsWith('/post/')) return '/posts';
    return pathname;
  };

  const selectedValue = getSelectedValue(location.pathname);

  const permissions = currentUser
    ? typeof currentUser.permissions === 'string'
      ? parseInt(currentUser.permissions, 10)
      : currentUser.permissions
    : 0;
  const hasAdminPermission = (permissions & 16) !== 0;

  const handleNavClick = (_e: unknown, data: { value: string }) => {
    navigate(data.value);
    if (isMobile) {
      setIsOpen(false);
    }
  };

  const handleLogout = () => {
    authApi.clearAuth();
    window.location.href = '/';
  };

  const renderNavItem = (label: string, icon: JSX.Element, path: string) => {
    const isSelected = selectedValue === path;
    return (
      <NavItem
        key={path}
        value={path}
        onClick={() => navigate(path)}
        className={styles.navItem}
        aria-selected={isSelected}
        style={{
          justifyContent: 'flex-start',
        }}
      >
        {isSelected && <div className={styles.activeIndicator} />}
        <div
          className={styles.navItemIcon}
          style={{
            width: isExpanded || isMobile ? '24px' : '80px',
            marginLeft: isExpanded || isMobile ? '16px' : '0',
          }}
        >
          {icon}
        </div>
        <span
          className={
            isExpanded || isMobile ? styles.navItemContent : styles.navItemContentCollapsed
          }
        >
          {label}
        </span>
      </NavItem>
    );
  };

  return (
    <div className={styles.root}>
      <div
        className={mergeClasses(
          styles.globalBackground,
          mode === 'dark' && styles.globalBackgroundDark
        )}
        style={{ backgroundImage: bingImage ? `url(${bingImage})` : undefined }}
      />

      <LiquidGlassBackground />

      <NavDrawer
        selectedValue={selectedValue}
        onNavItemSelect={handleNavClick}
        open={isMobile ? isOpen : true}
        type={isMobile ? 'overlay' : 'inline'}
        className={mergeClasses(
          styles.navDrawer,
          mode === 'dark' && styles.navDrawerDark,
          isMobile
            ? styles.navDrawerExpanded
            : isExpanded
              ? styles.navDrawerExpanded
              : styles.navDrawerCollapsed
        )}
      >
        <NavDrawerHeader>
          <div
            className={mergeClasses(
              styles.logo,
              isExpanded || isMobile ? styles.logoExpanded : undefined
            )}
          >
            {isExpanded || isMobile ? 'Peng Blog' : 'PB'}
          </div>
        </NavDrawerHeader>

        <NavDrawerBody>
          {renderNavItem('主页', <HomeRegular />, '/')}
          {renderNavItem('文章', <DocumentRegular />, '/posts')}
          {renderNavItem('分类', <FolderRegular />, '/categories')}
          {renderNavItem('标签', <TagRegular />, '/tags')}

          {hasAdminPermission && (
            <>
              <Divider style={{ margin: '8px 0', opacity: isExpanded ? 1 : 0 }} />
              {isExpanded && <NavSectionHeader>管理</NavSectionHeader>}
              {renderNavItem('管理后台', <SettingsRegular />, '/admin')}
            </>
          )}
        </NavDrawerBody>

        <NavDrawerFooter className={styles.footer}>
          <Divider />

          {/* Theme Toggle */}
          <Tooltip
            content={mode === 'light' ? '深色模式' : '浅色模式'}
            relationship="label"
            positioning="after"
          >
            <button
              className={styles.footerItem}
              onClick={toggleTheme}
              style={{
                justifyContent: 'flex-start',
                paddingRight: isExpanded || isMobile ? '10px' : '0',
              }}
            >
              <div
                className={styles.navItemIcon}
                style={{
                  width: isExpanded || isMobile ? '24px' : '80px',
                  marginLeft: isExpanded || isMobile ? '16px' : '0',
                }}
              >
                {mode === 'light' ? <WeatherMoonRegular /> : <WeatherSunnyFilled />}
              </div>
              <span
                className={
                  isExpanded || isMobile ? styles.navItemContent : styles.navItemContentCollapsed
                }
              >
                {mode === 'light' ? '深色模式' : '浅色模式'}
              </span>
            </button>
          </Tooltip>

          {/* User Profile / Login */}
          {isAuthenticated && currentUser ? (
            <>
              <div
                className={styles.footerItem}
                style={{
                  justifyContent: 'flex-start',
                  paddingRight: isExpanded || isMobile ? '10px' : '0',
                  cursor: 'default',
                }}
              >
                <div
                  className={styles.navItemIcon}
                  style={{
                    width: isExpanded || isMobile ? '24px' : '80px',
                    marginLeft: isExpanded || isMobile ? '16px' : '0',
                  }}
                >
                  {currentUser.avatar_url ? (
                    <Avatar
                      name={currentUser.username}
                      size={32}
                      color="brand"
                      image={{ src: currentUser.avatar_url }}
                    />
                  ) : (
                    <Avatar name={currentUser.username} size={32} color="brand" />
                  )}
                </div>
                <span
                  className={
                    isExpanded || isMobile ? styles.navItemContent : styles.navItemContentCollapsed
                  }
                >
                  <div style={{ lineHeight: '1.2', fontWeight: tokens.fontWeightSemibold }}>
                    {currentUser.username}
                  </div>
                  <div
                    style={{
                      lineHeight: '1.2',
                      fontSize: tokens.fontSizeBase200,
                      color: tokens.colorNeutralForeground3,
                    }}
                  >
                    {hasAdminPermission ? '管理员' : '用户'}
                  </div>
                </span>
              </div>

              <Tooltip content="登出" relationship="label" positioning="after">
                <button
                  className={styles.footerItem}
                  onClick={handleLogout}
                  style={{
                    justifyContent: 'flex-start',
                    paddingRight: isExpanded || isMobile ? '10px' : '0',
                  }}
                >
                  <div
                    className={styles.navItemIcon}
                    style={{
                      width: isExpanded || isMobile ? '24px' : '80px',
                      marginLeft: isExpanded || isMobile ? '16px' : '0',
                    }}
                  >
                    <SignOutRegular />
                  </div>
                  <span
                    className={
                      isExpanded || isMobile
                        ? styles.navItemContent
                        : styles.navItemContentCollapsed
                    }
                  >
                    登出
                  </span>
                </button>
              </Tooltip>
            </>
          ) : publicConfig?.allow_registration ? (
            <Tooltip content="登录" relationship="label" positioning="after">
              <button
                className={styles.footerItem}
                onClick={() => navigate('/login')}
                style={{
                  justifyContent: 'flex-start',
                  paddingRight: isExpanded || isMobile ? '10px' : '0',
                }}
              >
                <div
                  className={styles.navItemIcon}
                  style={{
                    width: isExpanded || isMobile ? '24px' : '80px',
                    marginLeft: isExpanded || isMobile ? '16px' : '0',
                  }}
                >
                  <ArrowEnterRegular />
                </div>
                <span
                  className={
                    isExpanded || isMobile ? styles.navItemContent : styles.navItemContentCollapsed
                  }
                >
                  登录
                </span>
              </button>
            </Tooltip>
          ) : null}

          {/* Collapse Toggle */}
          {!isMobile && (
            <div style={{ marginTop: 'auto', paddingTop: 8 }}>
              <Tooltip
                content={isExpanded ? '折叠' : '展开'}
                relationship="label"
                positioning="after"
              >
                <button
                  className={styles.footerItem}
                  onClick={() => setIsExpanded(!isExpanded)}
                  style={{
                    justifyContent: 'flex-start',
                    paddingRight: isExpanded ? '10px' : '0',
                  }}
                >
                  <div
                    className={styles.navItemIcon}
                    style={{
                      width: isExpanded ? '24px' : '80px',
                      marginLeft: isExpanded ? '16px' : '0',
                    }}
                  >
                    {isExpanded ? <PanelLeftContractRegular /> : <PanelLeftExpandRegular />}
                  </div>
                  <span
                    className={isExpanded ? styles.navItemContent : styles.navItemContentCollapsed}
                  >
                    折叠导航
                  </span>
                </button>
              </Tooltip>
            </div>
          )}
          {isMobile && (
            <div style={{ marginTop: 'auto', paddingTop: 8 }}>
              <button
                className={styles.footerItem}
                onClick={() => setIsOpen(false)}
                style={{
                  justifyContent: 'flex-start',
                  paddingRight: '10px',
                }}
              >
                <div
                  className={styles.navItemIcon}
                  style={{
                    width: '24px',
                    marginLeft: '16px',
                  }}
                >
                  <PanelLeftContractRegular />
                </div>
                <span className={styles.navItemContent}>关闭菜单</span>
              </button>
            </div>
          )}
        </NavDrawerFooter>
      </NavDrawer>

      {/* 内容区域 */}
      <main
        className={mergeClasses(
          styles.contentArea,
          location.pathname !== '/' &&
            (mode === 'dark' ? styles.contentAcrylicDark : styles.contentAcrylic)
        )}
      >
        {isMobile && !isOpen && (
          <button className={styles.mobileToggle} onClick={() => setIsOpen(true)}>
            <NavigationRegular fontSize={20} />
          </button>
        )}
        {children}
      </main>
    </div>
  );
}
