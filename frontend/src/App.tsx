import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Home from './pages/Home';
import LoginForm from './components/LoginForm';
import './App.css';

// 创建路由保护组件（可选，用于需要认证的路由）
const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  // 这里可以添加认证检查逻辑
  const isAuthenticated = localStorage.getItem('token') !== null;
  
  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }
  
  return <>{children}</>;
};

const App: React.FC = () => {
  return (
    <Router>
      <div className="app">
        <Routes>
          {/* 主页路由 */}
          <Route path="/" element={<Home />} />
          
          {/* 登录路由 */}
          <Route 
            path="/login" 
            element={
              <div className="login-page">
                <LoginForm 
                  onLoginSuccess={() => window.location.href = '/'}
                />
              </div>
            } 
          />
          
          {/* 注册路由 - 可以添加注册表单组件 */}
          <Route 
            path="/register" 
            element={
              <div className="register-page">
                <h2>注册</h2>
                <p>注册功能开发中...</p>
                <a href="/login">返回登录</a>
              </div>
            } 
          />
          
          {/* 受保护的路由示例 */}
          <Route 
            path="/admin" 
            element={
              <ProtectedRoute>
                <div className="admin-page">
                  <h2>管理后台</h2>
                  <p>管理功能开发中...</p>
                </div>
              </ProtectedRoute>
            } 
          />
          
          {/* 关于页面 */}
          <Route 
            path="/about" 
            element={
              <div className="about-page">
                <h2>关于 Peng Blog</h2>
                <p>Peng Blog 是一个现代化的博客平台，支持文章发布、评论、文件上传等功能。</p>
              </div>
            } 
          />
          
          {/* 默认重定向到首页 */}
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </div>
    </Router>
  );
};

export default App;