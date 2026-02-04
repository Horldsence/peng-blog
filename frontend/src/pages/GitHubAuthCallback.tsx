import { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { authApi } from '../api';

type AuthStatus = 'loading' | 'success' | 'error';

export function GitHubAuthCallback() {
  const [searchParams] = useSearchParams();
  const [status, setStatus] = useState<AuthStatus>('loading');
  const [errorMessage, setErrorMessage] = useState<string>('');

  useEffect(() => {
    const token = searchParams.get('token');
    const error = searchParams.get('error');
    const description = searchParams.get('description');

    if (error) {
      setStatus('error');
      setErrorMessage(description ?? 'GitHub 授权失败，请重试');
      const returnUrl = sessionStorage.getItem('github_oauth_return') ?? '/login';
      sessionStorage.removeItem('github_oauth_return');

      setTimeout(() => {
        window.location.href = returnUrl;
      }, 3000);
      return;
    }

    if (token) {
      try {
        authApi.saveGitHubAuth(token);
        setStatus('success');

        const returnUrl = sessionStorage.getItem('github_oauth_return') ?? '/';
        sessionStorage.removeItem('github_oauth_return');

        setTimeout(() => {
          window.location.href = returnUrl;
        }, 500);
      } catch (error) {
        console.error('Failed to store token:', error);
        setStatus('error');
        setErrorMessage('保存登录信息失败，请重试');
      }
    } else {
      setStatus('error');
      setErrorMessage('未收到授权信息，请重试');
    }
  }, [searchParams]);

  if (status === 'error') {
    return (
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          minHeight: '100vh',
          gap: '16px',
          padding: '20px',
        }}
      >
        <h2 style={{ color: '#d13438' }}>GitHub 授权失败</h2>
        <p style={{ textAlign: 'center', maxWidth: '500px' }}>{errorMessage}</p>
        <p style={{ fontSize: '14px', color: '#666' }}>3秒后自动跳转...</p>
      </div>
    );
  }

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: '100vh',
        gap: '16px',
      }}
    >
      <h2>GitHub 登录成功</h2>
      <p>正在跳转...</p>
    </div>
  );
}
