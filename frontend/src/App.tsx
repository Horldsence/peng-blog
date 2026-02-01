import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { ThemeProvider } from './contexts/ThemeContext';
import { ToastProvider } from './components/ui/Toast';
import { MainLayout } from './components/layout/MainLayout';
import { HomePage } from './pages/HomePage';
import { PostsPage } from './pages/PostsPage';
import { PostDetailPage } from './pages/PostDetailPage';
import { TagsPage } from './pages/TagsPage';
import { CategoriesPage } from './pages/CategoriesPage';
import { SearchPage } from './pages/SearchPage';
import { LoginForm } from './components/ui/LoginForm';
import { RegisterPage } from './pages/RegisterPage';
import { PostEditorPage } from './pages/PostEditorPage';
import { AdminPage } from './pages/AdminPage';
import './App.css';

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const isAuthenticated = localStorage.getItem('token') !== null;

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}

function App() {
  return (
    <ThemeProvider>
      <Router>
        <ToastProvider>
          <Routes>
            {/* 主应用路由（带布局） */}
            <Route
              path="/"
              element={
                <MainLayout>
                  <HomePage />
                </MainLayout>
              }
            />
            <Route
              path="/posts"
              element={
                <MainLayout>
                  <PostsPage />
                </MainLayout>
              }
            />
            <Route
              path="/post/:id"
              element={
                <MainLayout>
                  <PostDetailPage />
                </MainLayout>
              }
            />
            <Route
              path="/tags"
              element={
                <MainLayout>
                  <TagsPage />
                </MainLayout>
              }
            />
            <Route
              path="/categories"
              element={
                <MainLayout>
                  <CategoriesPage />
                </MainLayout>
              }
            />
            <Route
              path="/search"
              element={
                <MainLayout>
                  <SearchPage />
                </MainLayout>
              }
            />

            {/* 认证路由（不带布局） */}
            <Route
              path="/login"
              element={<LoginForm onLoginSuccess={() => (window.location.href = '/')} />}
            />
            <Route path="/register" element={<RegisterPage />} />

            {/* 管理员路由 */}
            <Route
              path="/admin/posts/new"
              element={
                <ProtectedRoute>
                  <MainLayout>
                    <PostEditorPage />
                  </MainLayout>
                </ProtectedRoute>
              }
            />
            <Route
              path="/admin/posts/edit/:id"
              element={
                <ProtectedRoute>
                  <MainLayout>
                    <PostEditorPage />
                  </MainLayout>
                </ProtectedRoute>
              }
            />
            <Route
              path="/admin"
              element={
                <ProtectedRoute>
                  <MainLayout>
                    <AdminPage />
                  </MainLayout>
                </ProtectedRoute>
              }
            />

            {/* 关于页面 */}
            <Route
              path="/about"
              element={
                <MainLayout>
                  <div style={{ padding: '32px' }}>
                    <h2>关于 Peng Blog</h2>
                    <p>基于 Rust + Fluent UI 2 构建的现代化博客系统</p>
                  </div>
                </MainLayout>
              }
            />

            {/* 404 重定向 */}
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </ToastProvider>
      </Router>
    </ThemeProvider>
  );
}

export default App;
