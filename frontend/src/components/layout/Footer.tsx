import React from 'react';
import { Github, Twitter, Heart } from 'lucide-react';

const Footer: React.FC = () => {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t bg-background">
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
          {/* Brand */}
          <div className="space-y-4">
            <div className="flex items-center space-x-2">
              <div className="h-8 w-8 rounded-lg bg-primary flex items-center justify-center">
                <span className="text-primary-foreground font-bold text-sm">PB</span>
              </div>
              <span className="text-lg font-bold">Peng Blog</span>
            </div>
            <p className="text-sm text-muted-foreground">
              一个现代化的博客平台，分享技术、记录生活。
            </p>
          </div>

          {/* Quick Links */}
          <div>
            <h3 className="font-semibold mb-4">快速链接</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li><a href="/" className="hover:text-foreground transition-colors">首页</a></li>
              <li><a href="/posts" className="hover:text-foreground transition-colors">文章</a></li>
              <li><a href="/about" className="hover:text-foreground transition-colors">关于</a></li>
              <li><a href="/admin" className="hover:text-foreground transition-colors">管理</a></li>
            </ul>
          </div>

          {/* Resources */}
          <div>
            <h3 className="font-semibold mb-4">资源</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li><a href="/docs" className="hover:text-foreground transition-colors">文档</a></li>
              <li><a href="/api" className="hover:text-foreground transition-colors">API</a></li>
              <li><a href="/tags" className="hover:text-foreground transition-colors">标签</a></li>
              <li><a href="/categories" className="hover:text-foreground transition-colors">分类</a></li>
            </ul>
          </div>

          {/* Legal */}
          <div>
            <h3 className="font-semibold mb-4">法律信息</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li><a href="/privacy" className="hover:text-foreground transition-colors">隐私政策</a></li>
              <li><a href="/terms" className="hover:text-foreground transition-colors">服务条款</a></li>
              <li><a href="/contact" className="hover:text-foreground transition-colors">联系我们</a></li>
            </ul>
          </div>
        </div>

        {/* Bottom bar */}
        <div className="mt-8 pt-8 border-t flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
          <p className="text-sm text-muted-foreground flex items-center space-x-2">
            <span>© {currentYear} Peng Blog. All rights reserved.</span>
            <span className="hidden md:inline">•</span>
            <span className="hidden md:inline flex items-center space-x-1">
              Made with <Heart className="h-4 w-4 fill-red-500 text-red-500" /> by Peng
            </span>
          </p>
          
          <div className="flex items-center space-x-4">
            <a
              href="https://github.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-muted-foreground hover:text-foreground transition-colors"
              aria-label="GitHub"
            >
              <Github className="h-5 w-5" />
            </a>
            <a
              href="https://twitter.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-muted-foreground hover:text-foreground transition-colors"
              aria-label="Twitter"
            >
              <Twitter className="h-5 w-5" />
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
