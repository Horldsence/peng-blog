import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import {
  Card,
  CardHeader,
  Button,
  Input,
  Title2,
  Body1,
  tokens,
} from '@fluentui/react-components';
import {
  ArrowLeftRegular,
  PersonRegular,
  LockClosedRegular,
} from '@fluentui/react-icons';
import { authApi } from '../../api';
import type { UserLoginRequest } from '../../types';

interface LoginFormProps {
  onLoginSuccess?: () => void;
  onLoginError?: (error: any) => void;
}

export function LoginForm({ onLoginSuccess, onLoginError }: LoginFormProps) {
  const navigate = useNavigate();
  const [formData, setFormData] = useState<UserLoginRequest>({
    username: '',
    password: '',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!formData.username.trim() || !formData.password.trim()) {
      setError('用户名和密码不能为空');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const response = await authApi.login(formData);
      authApi.saveAuth(response);

      if (onLoginSuccess) {
        onLoginSuccess();
      }

      console.log('登录成功:', response.user);
    } catch (err: any) {
      const errorMessage = err.message || '登录失败，请检查用户名和密码';
      setError(errorMessage);

      if (onLoginError) {
        onLoginError(err);
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        minHeight: '100vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: 'var(--colorNeutralBackground2)',
        padding: '24px',
      }}
    >
      <Card
        style={{
          width: '100%',
          maxWidth: '420px',
          borderRadius: tokens.borderRadiusLarge,
          padding: '32px',
        }}
      >
        <CardHeader
          header={
            <div style={{ textAlign: 'center' }}>
              <Title2>登录</Title2>
              <Body1 style={{ color: 'var(--colorNeutralForeground2)', marginTop: '8px' }}>
                欢迎回到 Peng Blog
              </Body1>
            </div>
          }
        />

        {/* 错误提示 */}
        {error && (
          <div
            style={{
              padding: '12px 16px',
              marginBottom: '24px',
              backgroundColor: 'var(--colorStatusDangerBackground1)',
              border: '1px solid var(--colorStatusDangerBorder1)',
              borderRadius: tokens.borderRadiusMedium,
              color: 'var(--colorStatusDangerForeground1)',
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
            }}
          >
            <Body1>{error}</Body1>
            <Button
              appearance="transparent"
              size="small"
              onClick={() => setError('')}
            >
              ×
            </Button>
          </div>
        )}

        {/* 登录表单 */}
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
          <div>
            <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>
              用户名
            </Body1>
            <Input
              name="username"
              placeholder="请输入用户名"
              value={formData.username}
              onChange={(_, data) => {
                setFormData((prev: UserLoginRequest) => ({
                  ...prev,
                  username: data.value,
                }));
                if (error) setError('');
              }}
              contentBefore={<PersonRegular />}
              style={{ width: '100%' }}
              size="large"
              disabled={loading}
              autoComplete="username"
            />
          </div>

          <div>
            <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>
              密码
            </Body1>
            <Input
              type="password"
              name="password"
              placeholder="请输入密码"
              value={formData.password}
              onChange={(_, data) => {
                setFormData((prev: UserLoginRequest) => ({
                  ...prev,
                  password: data.value,
                }));
                if (error) setError('');
              }}
              contentBefore={<LockClosedRegular />}
              style={{ width: '100%' }}
              size="large"
              disabled={loading}
              autoComplete="current-password"
            />
          </div>

          <Button
            type="submit"
            appearance="primary"
            size="large"
            disabled={loading}
            style={{ marginTop: '8px' }}
          >
            {loading ? '登录中...' : '登录'}
          </Button>
        </form>

        {/* 注册链接 */}
        <div style={{ textAlign: 'center', marginTop: '24px' }}>
          <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
            还没有账户？{' '}
            <Link
              to="/register"
              style={{
                color: 'var(--colorBrandForeground1)',
                textDecoration: 'none',
                fontWeight: '600',
              }}
            >
              立即注册
            </Link>
          </Body1>
        </div>

        {/* 返回按钮 */}
        <div style={{ textAlign: 'center', marginTop: '16px' }}>
          <Button
            appearance="transparent"
            icon={<ArrowLeftRegular />}
            onClick={() => navigate('/')}
            size="small"
          >
            返回首页
          </Button>
        </div>
      </Card>
    </div>
  );
};
