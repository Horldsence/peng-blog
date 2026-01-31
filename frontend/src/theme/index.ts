/**
 * Fluent UI 2 主题配置
 * 支持深色/浅色模式切换
 */

import {
  webLightTheme,
  webDarkTheme,
  Theme,
} from '@fluentui/react-components';

/**
 * 自定义浅色主题 */
export const lightTheme: Theme = {
  ...webLightTheme,
};

/**
 * 自定义深色主题 */
export const darkTheme: Theme = {
  ...webDarkTheme,
};

/**
 * 主题类型 */
export type ThemeMode = 'light' | 'dark';

/**
 * 获取当前主题 */
export const getTheme = (mode: ThemeMode): Theme => {
  return mode === 'dark' ? darkTheme : lightTheme;
};

/**
 * 从 localStorage 获取主题模式 */
export const getStoredThemeMode = (): ThemeMode => {
  const stored = localStorage.getItem('theme-mode');
  if (stored === 'light' || stored === 'dark') {
    return stored;
  }

  // 检测系统主题偏好
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }

  return 'light';
};

/**
 * 保存主题模式到 localStorage */
export const storeThemeMode = (mode: ThemeMode): void => {
  localStorage.setItem('theme-mode', mode);
};
