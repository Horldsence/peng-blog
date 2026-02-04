import React, { useState, useEffect, ReactNode } from 'react';
import { configApi } from '../api';
import { ConfigContext } from './configCore';
import type { PublicConfig } from '../types';

export const ConfigProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [publicConfig, setPublicConfig] = useState<PublicConfig | null>(null);
  const [loading, setLoading] = useState(true);

  const fetchConfig = async () => {
    try {
      const response = await configApi.getPublicConfig();
      setPublicConfig(response.data);
    } catch (error) {
      console.error('Failed to fetch public config:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void fetchConfig();
  }, []);

  return (
    <ConfigContext.Provider
      value={{
        publicConfig,
        loading,
        refreshConfig: fetchConfig,
      }}
    >
      {children}
    </ConfigContext.Provider>
  );
};
