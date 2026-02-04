import { useEffect, useState } from 'react';
import { useSearchParams, Navigate } from 'react-router-dom';
import { authApi } from '../api';

type AuthStatus = 'loading' | 'success' | 'error';

export function GitHubAuthCallback() {
  const [searchParams] = useSearchParams();
  const [status, setStatus] = useState<AuthStatus>('loading');

  useEffect(() => {
    const token = searchParams.get('token');

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
      }
    } else {
      setStatus('error');
    }
  }, [searchParams]);

  if (status === 'error') {
    return <Navigate to="/login" replace />;
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
