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
  makeStyles,
  mergeClasses,
} from '@fluentui/react-components';
import {
  ArrowLeftRegular,
  PersonRegular,
  LockClosedRegular,
} from '@fluentui/react-icons';
import { authApi } from '../../api';
import type { UserLoginRequest } from '../../types';

const useStyles = makeStyles({
  container: {
    minHeight: '100vh',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: tokens.colorNeutralBackground2,
    padding: '24px',
  },
  card: {
    width: '100%',
    maxWidth: '420px',
    borderRadius: tokens.borderRadiusLarge,
    padding: '32px',
  },
  headerContent: {
    textAlign: 'center',
  },
  headerSubtitle: {
    color: tokens.colorNeutralForeground2,
    marginTop: '8px',
  },
  errorBox: {
    padding: '12px 16px',
    marginBottom: '24px',
    backgroundColor: tokens.colorStatusDangerBackground1,
    border: `1px solid ${tokens.colorStatusDangerBorder1}`,
    borderRadius: tokens.borderRadiusMedium,
    color: tokens.colorStatusDangerForeground1,
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  form: {
    display: 'flex',
    flexDirection: 'column',
    gap: '20px',
  },
  fieldLabel: {
    fontWeight: tokens.fontWeightSemibold,
    marginBottom: '8px',
    display: 'block',
  },
  input: {
    width: '100%',
  },
  submitButton: {
    marginTop: '8px',
  },
  footer: {
    textAlign: 'center',
    marginTop: '24px',
  },
  footerText: {
    color: tokens.colorNeutralForeground2,
  },
  link: {
    color: tokens.colorBrandForeground1,
    textDecoration: 'none',
    fontWeight: tokens.fontWeightSemibold,
    ':hover': {
      textDecoration: 'underline',
    },
  },
  backButtonContainer: {
    textAlign: 'center',
    marginTop: '16px',
  },
});

interface LoginFormProps {
  onLoginSuccess?: () => void;
  onLoginError?: (error: any) => void;
}

export function LoginForm({ onLoginSuccess, onLoginError }: LoginFormProps) {
  const styles = useStyles();
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
    <div className={styles.container}>
      <Card className={styles.card}>
        <CardHeader
          header={
            <div className={styles.headerContent}>
              <Title2>登录</Title2>
              <Body1 className={styles.headerSubtitle}>
                欢迎回到 Peng Blog
              </Body1>
            </div>
          }
        />

        {/* 错误提示 */}
        {error && (
          <div className={styles.errorBox}>
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
        <form onSubmit={handleSubmit} className={styles.form}>
          <div>
            <label className={styles.fieldLabel}>
              用户名
            </label>
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
              className={styles.input}
              size="large"
              disabled={loading}
              autoComplete="username"
            />
          </div>

          <div>
            <label className={styles.fieldLabel}>
              密码
            </label>
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
              className={styles.input}
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
            className={styles.submitButton}
          >
            {loading ? '登录中...' : '登录'}
          </Button>
        </form>

        {/* 注册链接 */}
        <div className={styles.footer}>
          <Body1 className={styles.footerText}>
            还没有账户？{' '}
            <Link
              to="/register"
              className={styles.link}
            >
              立即注册
            </Link>
          </Body1>
        </div>

        {/* 返回按钮 */}
        <div className={styles.backButtonContainer}>
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
