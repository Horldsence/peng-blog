import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Home from './pages/Home';
import LoginForm from './components/LoginForm';
import Register from './pages/Register';
import PostDetail from './pages/PostDetail';
import PostEditor from './pages/PostEditor';
import Admin from './pages/Admin';
import './App.css';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
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
          <Route path="/" element={<Home />} />

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

          <Route path="/register" element={<Register />} />

          <Route path="/post/:id" element={<PostDetail />} />

          <Route
            path="/admin/posts/new"
            element={
              <ProtectedRoute>
                <PostEditor />
              </ProtectedRoute>
            }
          />

          <Route
            path="/admin/posts/edit/:id"
            element={
              <ProtectedRoute>
                <PostEditor />
              </ProtectedRoute>
            }
          />

          <Route
            path="/admin"
            element={
              <ProtectedRoute>
                <Admin />
              </ProtectedRoute>
            }
          />

          <Route
            path="/about"
            element={
              <div className="about-page">
                <h2>关于 Peng Blog</h2>
                <p>Peng Blog 是一个现代化的博客平台，支持文章发布、评论、文件上传等功能。</p>
              </div>
            }
          />

          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </div>
    </Router>
  );
};

export default App;