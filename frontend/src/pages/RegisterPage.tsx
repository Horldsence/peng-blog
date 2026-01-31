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
import { authApi } from '../api';
import type { UserCreateRequest } from '../types';

export function RegisterPage() {
  const navigate = useNavigate();
  const [formData, setFormData] = useState<UserCreateRequest>({
    username: '',
    password: '',
  });
  const [confirmPassword, setConfirmPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!formData.username.trim() || !formData.password.trim()) {
      setError('用户名和密码不能为空');
      return;
    }

    if (formData.username.length < 3) {
      setError('用户名至少需要3个字符');
      return;
    }

    if (formData.password.length < 8) {
      setError('密码至少需要8个字符');
      return;
    }

    if (formData.password !== confirmPassword) {
      setError('两次输入的密码不一致');
      return;
    }

    setLoading(true);
    setError('');

    try {
      await authApi.register(formData);
      alert('注册成功！请登录');
      navigate('/login');
    } catch (err: any) {
      const errorMessage = err.message || '注册失败';
      setError(errorMessage);
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
              <Title2>创建账户</Title2>
              <Body1 style={{ color: 'var(--colorNeutralForeground2)', marginTop: '8px' }}>
                加入 Peng Blog，开始你的写作之旅
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

        {/* 注册表单 */}
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
          <div>
            <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>
              用户名
            </Body1>
            <Input
              name="username"
              placeholder="至少3个字符"
              value={formData.username}
              onChange={(_, data) => {
                setFormData(prev => ({ ...prev, username: data.value }));
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
              placeholder="至少8个字符"
              value={formData.password}
              onChange={(_, data) => {
                setFormData(prev => ({ ...prev, password: data.value }));
                if (error) setError('');
              }}
              contentBefore={<LockClosedRegular />}
              style={{ width: '100%' }}
              size="large"
              disabled={loading}
              autoComplete="new-password"
            />
          </div>

          <div>
            <Body1 style={{ fontWeight: '600', marginBottom: '8px' }}>
              确认密码
            </Body1>
            <Input
              type="password"
              placeholder="再次输入密码"
              value={confirmPassword}
              onChange={(_, data) => setConfirmPassword(data.value)}
              contentBefore={<LockClosedRegular />}
              style={{ width: '100%' }}
              size="large"
              disabled={loading}
              autoComplete="new-password"
            />
          </div>

          <Button
            type="submit"
            appearance="primary"
            size="large"
            disabled={loading}
            style={{ marginTop: '8px' }}
          >
            {loading ? '注册中...' : '注册'}
          </Button>
        </form>

        {/* 登录链接 */}
        <div style={{ textAlign: 'center', marginTop: '24px' }}>
          <Body1 style={{ color: 'var(--colorNeutralForeground2)' }}>
            已有账户？{' '}
            <Link
              to="/login"
              style={{
                color: 'var(--colorBrandForeground1)',
                textDecoration: 'none',
                fontWeight: '600',
              }}
            >
              立即登录
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
