import { useContext, createContext } from 'react';
import type { PublicConfig } from '../types';

export interface ConfigContextType {
  publicConfig: PublicConfig | null;
  loading: boolean;
  refreshConfig: () => Promise<void>;
}

export const ConfigContext = createContext<ConfigContextType | undefined>(undefined);

export const useConfig = (): ConfigContextType => {
  const context = useContext(ConfigContext);
  if (context === undefined) {
    throw new Error('useConfig must be used within a ConfigProvider');
  }
  return context;
};
