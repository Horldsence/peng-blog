/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, useCallback, ReactNode } from 'react';
import {
  useId,
  Toaster,
  useToastController,
  Toast as FluentToast,
  ToastTitle,
  ToastBody,
  ToastIntent,
} from '@fluentui/react-components';

type ToastType = 'success' | 'error' | 'info' | 'warning';

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
  const toasterId = useId('app-toaster');
  const { dispatchToast } = useToastController(toasterId);

  const showToast = useCallback(
    (title: string, message?: string, type: ToastType = 'info') => {
      dispatchToast(
        <FluentToast>
          <ToastTitle>{title}</ToastTitle>
          {message && <ToastBody>{message}</ToastBody>}
        </FluentToast>,
        { intent: type as ToastIntent }
      );
    },
    [dispatchToast]
  );

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
      <Toaster toasterId={toasterId} position="bottom-end" />
    </ToastContext.Provider>
  );
};
