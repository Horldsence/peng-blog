/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';

type ToastType = 'success' | 'error' | 'info' | 'warning';

interface ToastMessage {
  id: string;
  title: string;
  message?: string;
  type: ToastType;
}

interface ToastContextType {
  showToast: (title: string, message?: string, type?: ToastType) => void;
  showSuccess: (message: string) => void;
  showError: (message: string) => void;
  showInfo: (message: string) => void;
  showWarning: (message: string) => void;
}

const ToastContext = createContext<ToastContextType | undefined>(undefined);

export const useToast = () => {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within ToastProvider');
  }
  return context;
};

export const ToastProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [toasts, setToasts] = useState<ToastMessage[]>([]);

  const getToastStyle = useCallback((type: ToastType) => {
    const styles = {
      success: { backgroundColor: '#dff6dd', color: '#0f5132', border: '1px solid #badbcc' },
      error: { backgroundColor: '#f8d7da', color: '#842029', border: '1px solid #f5c2c7' },
      info: { backgroundColor: '#d1ecf1', color: '#055160', border: '1px solid #bee5eb' },
      warning: { backgroundColor: '#fff3cd', color: '#664d03', border: '1px solid #ffe69c' },
    };
    return styles[type];
  }, []);

  const showToast = useCallback((title: string, message?: string, type: ToastType = 'info') => {
    const id = Math.random().toString(36).substring(7);
    const newToast: ToastMessage = { id, title, message, type };
    setToasts((prev) => [...prev, newToast]);

    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 5000);
  }, []);

  const showSuccess = useCallback(
    (message: string) => {
      showToast('成功', message, 'success');
    },
    [showToast]
  );

  const showError = useCallback(
    (message: string) => {
      showToast('错误', message, 'error');
    },
    [showToast]
  );

  const showInfo = useCallback(
    (message: string) => {
      showToast('提示', message, 'info');
    },
    [showToast]
  );

  const showWarning = useCallback(
    (message: string) => {
      showToast('警告', message, 'warning');
    },
    [showToast]
  );

  return (
    <ToastContext.Provider value={{ showToast, showSuccess, showError, showInfo, showWarning }}>
      {children}
      <div
        style={{
          position: 'fixed',
          bottom: '24px',
          right: '24px',
          zIndex: 9999,
          display: 'flex',
          flexDirection: 'column',
          gap: '12px',
        }}
      >
        {toasts.map((toast) => (
          <div
            key={toast.id}
            style={{
              ...getToastStyle(toast.type),
              padding: '16px',
              borderRadius: '8px',
              minWidth: '300px',
              animation: 'slideInRight 0.3s ease-out',
              boxShadow: '0 4px 12px rgba(0,0,0,0.15)',
            }}
          >
            <div style={{ fontWeight: '600', marginBottom: toast.message ? '8px' : '0' }}>
              {toast.title}
            </div>
            {toast.message && <div style={{ fontSize: '14px' }}>{toast.message}</div>}
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
};
