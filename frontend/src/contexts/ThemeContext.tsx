/**
 * 主题 Context 和 Provider
 * 管理全局主题状态（深色/浅色模式）
 */

import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { Theme, FluentProvider } from '@fluentui/react-components';
import { getTheme, getStoredThemeMode, storeThemeMode, ThemeMode } from '../theme';

interface ThemeContextValue {
  mode: ThemeMode;
  toggleTheme: () => void;
  setTheme: (mode: ThemeMode) => void;
  theme: Theme;
}

const ThemeContext = createContext<ThemeContextValue | undefined>(undefined);

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const [mode, setMode] = useState<ThemeMode>(() => getStoredThemeMode());
  const theme = getTheme(mode);

  // 监听系统主题变化
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    const handleChange = (e: MediaQueryListEvent) => {
      // 只有在用户没有手动设置主题时才跟随系统
      const stored = localStorage.getItem('theme-mode');
      if (!stored) {
        setMode(e.matches ? 'dark' : 'light');
      }
    };

    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  // 应用主题 class 到 document
  useEffect(() => {
    document.documentElement.classList.remove('light', 'dark');
    document.documentElement.classList.add(mode);
  }, [mode]);

  const toggleTheme = () => {
    const newMode: ThemeMode = mode === 'light' ? 'dark' : 'light';
    setMode(newMode);
    storeThemeMode(newMode);
  };

  const setTheme = (newMode: ThemeMode) => {
    setMode(newMode);
    storeThemeMode(newMode);
  };

  return (
    <ThemeContext.Provider value={{ mode, toggleTheme, setTheme, theme }}>
      <FluentProvider theme={theme}>
        {children}
      </FluentProvider>
    </ThemeContext.Provider>
  );
}

/**
 * 使用主题 Context */
export function useTheme(): ThemeContextValue {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
}
