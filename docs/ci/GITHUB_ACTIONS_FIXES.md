# GitHub Actions CI/CD 修复文档

## 概述

本文档说明了 Peng Blog 项目 GitHub Actions CI/CD 配置的问题、修复方案和最佳实践。

## 问题诊断

### 原始问题

1. **CI 构建问题**：
   - `build` job 只运行 `cargo build --release`
   - **没有安装 Node.js 和 npm**
   - `app/build.rs` 尝试构建前端时找不到 npm，跳过前端构建
   - 最终二进制文件**不包含前端静态文件**
   - 构建的产物无法直接运行

2. **重复构建**：
   - `frontend-build` job 单独构建前端
   - 但构建产物不会传递给 `build` job
   - 导致前端被构建两次（如果 Node.js 存在的话）

3. **发布流程问题**：
   - `release.yml` 中构建命令为 `cargo build --release -p cli`
   - 只构建了 `cli` crate，没有构建 `app` crate
   - **没有安装 Node.js**，前端不会被嵌入
   - 发布的二进制文件不包含前端

### 影响范围

- ❌ CI 构建的二进制文件缺少前端
- ❌ Release 发布的二进制文件无法直接使用
- ❌ 用户下载后需要手动构建前端
- ❌ 违背了"单文件部署"的设计目标

## 修复方案

### 1. CI 配置修复 (`ci.yml`)

#### 修改 1: 在 build job 中添加 Node.js

```yaml
- name: Setup Node.js
  uses: actions/setup-node@v4
  with:
    node-version: "20"
    cache: "npm"
    cache-dependency-path:
```
