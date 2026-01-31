/**
 * 主布局组件 - Fluent UI 2 NavDrawer
 * 左侧导航栏 + 右侧内容区
 */

import { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import {
  Button,
  Avatar,
  Divider,
  tokens,
  Tooltip,
} from '@fluentui/react-components';
import {
  NavDrawer,
  NavDrawerHeader,
  NavDrawerBody,
  NavItem,
  NavSectionHeader,
} from '@fluentui/react-components';
import {
  HomeRegular,
  DocumentRegular,
  TagRegular,
  FolderRegular,
  SearchRegular,
  SettingsRegular,
  WeatherMoonRegular,
  WeatherSunnyFilled,
  SignOutRegular,
  ArrowEnterRegular,
  PersonRegular,
  PanelLeftContractRegular,
  PanelLeftExpandRegular,
} from '@fluentui/react-icons';
import { useTheme } from '../../contexts/ThemeContext';
import { authApi } from '../../api';

const styles = {
  root: {
    display: 'flex',
    height: '100vh',
    overflow: 'hidden',
    backgroundColor: tokens.colorNeutralBackground3,
  } as React.CSSProperties,

  contentArea: {
    flex: 1,
    overflow: 'auto',
    backgroundColor: tokens.colorNeutralBackground3,
  } as React.CSSProperties,
};

export function MainLayout({ children }: { children: React.ReactNode }) {
  const navigate = useNavigate();
  const location = useLocation();
  const { mode, toggleTheme } = useTheme();
  const isAuthenticated = authApi.isAuthenticated();
  const currentUser = authApi.getCurrentUser();

  const [isExpanded, setIsExpanded] = useState(true);

  const permissions = currentUser
    ? typeof currentUser.permissions === 'string'
      ? parseInt(currentUser.permissions, 10)
      : currentUser.permissions
    : 0;
  const hasAdminPermission = (permissions & 16) !== 0;

  const handleNavClick = (e: any) => {
    navigate(e.currentTarget.value);
  };

  const handleLogout = () => {
    authApi.clearAuth();
    window.location.href = '/';
  };

  return (
    <div style={styles.root}>
      <NavDrawer
        defaultSelectedValue={location.pathname}
        open={true}
        type="inline"
        density="medium"
        style={{
          width: isExpanded ? '260px' : '68px',
          transition: 'width 0.3s ease',
          minWidth: isExpanded ? '260px' : '68px',
        }}
      >
        <NavDrawerHeader>
          <Tooltip content={isExpanded ? '折叠导航' : '展开导航'} relationship="label">
            <Button
              appearance="subtle"
              icon={isExpanded ? <PanelLeftContractRegular /> : <PanelLeftExpandRegular />}
              onClick={() => setIsExpanded(!isExpanded)}
              style={{
                width: '100%',
                justifyContent: isExpanded ? 'flex-start' : 'center',
              }}
            >
              <span style={{
                opacity: isExpanded ? 1 : 0,
                maxWidth: isExpanded ? '200px' : '0px',
                overflow: 'hidden',
                transition: 'opacity 0.3s ease, max-width 0.3s ease',
                display: 'inline-block',
                whiteSpace: 'nowrap',
              }}>折叠</span>
            </Button>
          </Tooltip>
        </NavDrawerHeader>

        <NavDrawerBody>
          {/* Logo */}
          <div
            style={{
              padding: '16px 12px 20px 12px',
              fontSize: tokens.fontSizeBase600,
              fontWeight: tokens.fontWeightBold,
              color: tokens.colorNeutralForeground1,
              overflow: 'hidden',
              whiteSpace: 'nowrap',
              textAlign: isExpanded ? 'left' : 'center',
              minHeight: '48px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: isExpanded ? 'flex-start' : 'center',
            }}
          >
            {isExpanded ? 'Peng Blog' : 'PB'}
          </div>

          {/* 主导航 */}
          <NavItem value="/" onClick={handleNavClick}>
            <HomeRegular />
            <span style={{
              opacity: isExpanded ? 1 : 0,
              maxWidth: isExpanded ? '200px' : '0px',
              overflow: 'hidden',
              transition: 'opacity 0.3s ease, max-width 0.3s ease',
              display: 'inline-block',
              whiteSpace: 'nowrap',
            }}>主页</span>
          </NavItem>
          <NavItem value="/posts" onClick={handleNavClick}>
            <DocumentRegular />
            <span style={{
              opacity: isExpanded ? 1 : 0,
              maxWidth: isExpanded ? '200px' : '0px',
              overflow: 'hidden',
              transition: 'opacity 0.3s ease, max-width 0.3s ease',
              display: 'inline-block',
              whiteSpace: 'nowrap',
            }}>文章</span>
          </NavItem>
          <NavItem value="/categories" onClick={handleNavClick}>
            <FolderRegular />
            <span style={{
              opacity: isExpanded ? 1 : 0,
              maxWidth: isExpanded ? '200px' : '0px',
              overflow: 'hidden',
              transition: 'opacity 0.3s ease, max-width 0.3s ease',
              display: 'inline-block',
              whiteSpace: 'nowrap',
            }}>分类</span>
          </NavItem>
          <NavItem value="/tags" onClick={handleNavClick}>
            <TagRegular />
            <span style={{
              opacity: isExpanded ? 1 : 0,
              maxWidth: isExpanded ? '200px' : '0px',
              overflow: 'hidden',
              transition: 'opacity 0.3s ease, max-width 0.3s ease',
              display: 'inline-block',
              whiteSpace: 'nowrap',
            }}>标签</span>
          </NavItem>
          <NavItem value="/search" onClick={handleNavClick}>
            <SearchRegular />
            <span style={{
              opacity: isExpanded ? 1 : 0,
              maxWidth: isExpanded ? '200px' : '0px',
              overflow: 'hidden',
              transition: 'opacity 0.3s ease, max-width 0.3s ease',
              display: 'inline-block',
              whiteSpace: 'nowrap',
            }}>搜索</span>
          </NavItem>

          {/* 管理后台（仅管理员） */}
          {hasAdminPermission && (
            <>
              <NavSectionHeader style={{
                opacity: isExpanded ? 1 : 0,
                maxHeight: isExpanded ? '100px' : '0px',
                overflow: 'hidden',
                transition: 'opacity 0.3s ease, max-height 0.3s ease',
              }}>管理</NavSectionHeader>
              <NavItem value="/admin" onClick={handleNavClick}>
                <SettingsRegular />
                <span style={{
                  opacity: isExpanded ? 1 : 0,
                  maxWidth: isExpanded ? '200px' : '0px',
                  overflow: 'hidden',
                  transition: 'opacity 0.3s ease, max-width 0.3s ease',
                  display: 'inline-block',
                  whiteSpace: 'nowrap',
                }}>管理后台</span>
              </NavItem>
            </>
          )}

          <Divider style={{ margin: '16px 0' }} />

          {/* 用户信息 */}
          {isAuthenticated && currentUser ? (
            <>
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '12px',
                  padding: isExpanded ? '12px' : '8px',
                  backgroundColor: tokens.colorNeutralBackground2,
                  borderRadius: tokens.borderRadiusMedium,
                  marginBottom: '12px',
                  justifyContent: isExpanded ? 'flex-start' : 'center',
                  minHeight: '48px',
                }}
              >
                <Avatar 
                  name={currentUser.username} 
                  size={32} 
                  icon={<PersonRegular />} 
                  color="brand" 
                  style={{ 
                    minWidth: '32px',
                    minHeight: '32px'
                  }}
                />
                <div style={{ 
                  flex: 1, 
                  opacity: isExpanded ? 1 : 0,
                  width: isExpanded ? 'auto' : 0,
                  overflow: 'hidden',
                  transition: 'opacity 0.3s ease, width 0.3s ease',
                  display: 'inline-block',
                }}>
                  <div
                    style={{
                      fontSize: tokens.fontSizeBase300,
                      fontWeight: tokens.fontWeightSemibold,
                      color: tokens.colorNeutralForeground1,
                      whiteSpace: 'nowrap',
                      overflow: 'hidden',
                      textOverflow: 'ellipsis',
                    }}
                  >
                    {currentUser.username}
                  </div>
                  <div
                    style={{
                      fontSize: tokens.fontSizeBase200,
                      color: tokens.colorNeutralForeground3,
                    }}
                  >
                    {hasAdminPermission ? '管理员' : '用户'}
                  </div>
                </div>
              </div>

              <Tooltip content="登出" relationship="label" visible={!isExpanded ? undefined : false}>
                <Button
                  appearance="subtle"
                  icon={<SignOutRegular />}
                  onClick={handleLogout}
                  style={{
                    width: '100%',
                    justifyContent: isExpanded ? 'flex-start' : 'center',
                  }}
                >
                  <span style={{
                    opacity: isExpanded ? 1 : 0,
                    maxWidth: isExpanded ? '200px' : '0px',
                    overflow: 'hidden',
                    transition: 'opacity 0.3s ease, max-width 0.3s ease',
                    display: 'inline-block',
                    whiteSpace: 'nowrap',
                  }}>登出</span>
                </Button>
              </Tooltip>
            </>
          ) : (
            <Tooltip content="登录" relationship="label" visible={!isExpanded ? undefined : false}>
              <Button
                appearance="primary"
                icon={<ArrowEnterRegular />}
                onClick={() => navigate('/login')}
                style={{
                  width: '100%',
                  justifyContent: isExpanded ? 'flex-start' : 'center',
                }}
              >
                <span style={{
                  opacity: isExpanded ? 1 : 0,
                  maxWidth: isExpanded ? '200px' : '0px',
                  overflow: 'hidden',
                  transition: 'opacity 0.3s ease, max-width 0.3s ease',
                  display: 'inline-block',
                  whiteSpace: 'nowrap',
                }}>登录</span>
              </Button>
            </Tooltip>
          )}

          {/* 主题切换 */}
          <Tooltip
            content={mode === 'light' ? '深色模式' : '浅色模式'}
            relationship="label"
            visible={!isExpanded ? undefined : false}
          >
            <Button
              appearance="subtle"
              icon={mode === 'light' ? <WeatherMoonRegular /> : <WeatherSunnyFilled />}
              onClick={toggleTheme}
              style={{
                width: '100%',
                justifyContent: isExpanded ? 'flex-start' : 'center',
              }}
            >
              <span style={{
                opacity: isExpanded ? 1 : 0,
                maxWidth: isExpanded ? '200px' : '0px',
                overflow: 'hidden',
                transition: 'opacity 0.3s ease, max-width 0.3s ease',
                display: 'inline-block',
                whiteSpace: 'nowrap',
              }}>{mode === 'light' ? '深色模式' : '浅色模式'}</span>
            </Button>
          </Tooltip>
        </NavDrawerBody>
      </NavDrawer>

      {/* 内容区域 */}
      <main style={styles.contentArea}>{children}</main>
    </div>
  );
}
