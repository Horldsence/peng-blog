import React, { useState } from 'react';
import { authApi } from '../api';
import type { UserLoginRequest } from '../types';

interface LoginFormProps {
  onLoginSuccess?: () => void;
  onLoginError?: (error: any) => void;
}

const LoginForm: React.FC<LoginFormProps> = ({ onLoginSuccess, onLoginError }) => {
  const [formData, setFormData] = useState<UserLoginRequest>({
    username: '',
    password: '',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value,
    }));
    // 清除错误信息
    if (error) setError('');
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // 基本验证
    if (!formData.username.trim() || !formData.password.trim()) {
      setError('用户名和密码不能为空');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const response = await authApi.login(formData);
      
      // 保存登录信息
      authApi.saveAuth(response);
      
      // 调用成功回调
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
    <div className="login-form">
      <h2>登录</h2>
      
      {error && (
        <div className="error-message">
          {error}
        </div>
      )}
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="username">用户名</label>
          <input
            type="text"
            id="username"
            name="username"
            value={formData.username}
            onChange={handleChange}
            placeholder="请输入用户名"
            disabled={loading}
            autoComplete="username"
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="password">密码</label>
          <input
            type="password"
            id="password"
            name="password"
            value={formData.password}
            onChange={handleChange}
            placeholder="请输入密码"
            disabled={loading}
            autoComplete="current-password"
          />
        </div>
        
        <button type="submit" disabled={loading} className="submit-button">
          {loading ? '登录中...' : '登录'}
        </button>
      </form>
    </div>
  );
};

export default LoginForm;