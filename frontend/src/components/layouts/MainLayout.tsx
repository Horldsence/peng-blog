/**
 * 主布局组件
 * 左侧导航栏 + 右侧内容区
 */

import { useNavigate } from 'react-router-dom';
import {
  Divider,
  Caption1,
} from '@fluentui/react-components';
import {
  HomeRegular,
  DocumentRegular,
  TagRegular,
  FolderRegular,
  SearchRegular,
  SettingsRegular,
} from '@fluentui/react-icons';
import { useTheme } from '../../contexts/ThemeContext';

interface NavItem {
  key: string;
  name: string;
  url: string;
  icon: React.ReactNode;
}

const navItems: NavItem[] = [
  { key: 'home', name: '主页', url: '/', icon: <HomeRegular /> },
  { key: 'posts', name: '文章', url: '/posts', icon: <DocumentRegular /> },
  { key: 'tags', name: '标签', url: '/tags', icon: <TagRegular /> },
  { key: 'categories', name: '分类', url: '/categories', icon: <FolderRegular /> },
  { key: 'search', name: '搜索', url: '/search', icon: <SearchRegular /> },
];

const adminItems: NavItem[] = [
  { key: 'admin', name: '管理后台', url: '/admin', icon: <SettingsRegular /> },
];

export function MainLayout({ children }: { children: React.ReactNode }) {
  const navigate = useNavigate();
  const { mode, toggleTheme } = useTheme();

  return (
    <div style={{ display: 'flex', height: '100vh', overflow: 'hidden' }}>
      {/* 左侧导航栏 */}
      <div
        style={{
          width: '250px',
          backgroundColor: `var(--colorNeutralBackground1)`,
          borderRight: '1px solid var(--colorNeutralStroke1)',
          display: 'flex',
          flexDirection: 'column',
          padding: '16px',
        }}
      >
        {/* Logo */}
        <div style={{ marginBottom: '24px', padding: '0 8px' }}>
          <h2 style={{ margin: 0, fontSize: '20px', fontWeight: '600' }}>
            Peng Blog
          </h2>
          <Caption1 style={{ color: 'var(--colorNeutralForeground3)' }}>
            基于 Fluent UI 2
          </Caption1>
        </div>

        {/* 主导航 */}
        <div style={{ flex: 1, overflow: 'auto' }}>
          {navItems.map((item) => (
            <button
              key={item.key}
              onClick={() => navigate(item.url)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                padding: '12px',
                width: '100%',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer',
                backgroundColor: 'transparent',
                color: 'var(--colorNeutralForeground1)',
                fontSize: '14px',
                marginBottom: '4px',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'var(--colorNeutralBackground1Hover)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              {item.icon}
              <span>{item.name}</span>
            </button>
          ))}
        </div>

        <Divider />

        {/* 管理员菜单 */}
        <div style={{ marginTop: '8px' }}>
          {adminItems.map((item) => (
            <button
              key={item.key}
              onClick={() => navigate(item.url)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                padding: '12px',
                width: '100%',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer',
                backgroundColor: 'transparent',
                color: 'var(--colorNeutralForeground1)',
                fontSize: '14px',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'var(--colorNeutralBackground1Hover)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              {item.icon}
              <span>{item.name}</span>
            </button>
          ))}
        </div>

        <Divider />

        {/* 主题切换 */}
        <div style={{ padding: '8px' }}>
          <button
            onClick={toggleTheme}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '8px 12px',
              width: '100%',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              backgroundColor: 'var(--colorNeutralBackground1)',
              color: 'var(--colorNeutralForeground1)',
              fontSize: '14px',
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = 'var(--colorNeutralBackground1Hover)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = 'var(--colorNeutralBackground1)';
            }}
          >
            {mode === 'light' ? '🌙 深色模式' : '☀️ 浅色模式'}
          </button>
        </div>
      </div>

      {/* 右侧内容区 */}
      <div
        style={{
          flex: 1,
          overflow: 'auto',
          backgroundColor: 'var(--colorNeutralBackground3)',
        }}
      >
        <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '32px' }}>
          {children}
        </div>
      </div>
    </div>
  );
}
