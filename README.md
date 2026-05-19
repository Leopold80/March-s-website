# 林夏的个人网站

用 Rust + Axum 构建的个人网站，支持 Markdown 日志和媒体墙功能。

## ✨ 功能特性

- 📝 **Markdown 日志** - 在 `logs/` 目录放置 Markdown 文件即可发布日志
- 🖼️ **媒体墙** - 瀑布流布局展示照片和视频
- 🎨 **响应式设计** - 适配桌面、平板、手机
- 🚀 **高性能** - Rust 异步服务器，编译时模板嵌入

## 🚀 快速开始

### 环境要求

- Rust 1.70+ (推荐最新稳定版)

### 运行

```bash
# 克隆项目
git clone <repo-url>
cd marchs-website

# 开发模式运行
cargo run

# 发布模式构建（生产环境）
cargo build --release
```

服务器启动后访问：http://localhost:3000

## 📁 项目结构

```
marchs-website/
├── assets/           # HTML 模板
│   ├── index.html    # 首页
│   ├── logs.html     # 日志列表
│   ├── log_post.html # 日志阅读
│   └── media.html    # 媒体墙
├── logs/             # Markdown 日志文件
├── media/
│   ├── photos/       # 照片
│   └── videos/       # 视频
├── src/
│   ├── main.rs       # 入口
│   ├── server.rs     # 路由配置
│   ├── handlers/     # HTTP 处理器
│   ├── services/     # 业务逻辑
│   └── models/       # 数据结构
├── Cargo.toml
└── README.md
```

## 📝 使用指南

### 发布日志

1. 在 `logs/` 目录创建 `.md` 文件
2. 文件开头添加 frontmatter：

```markdown
---
title: 我的第一篇日志
date: 2024-01-15
---

这里是日志正文...
```

3. 刷新 `/logs` 页面即可看到

### 添加照片/视频

- 照片放入 `media/photos/`
- 视频放入 `media/videos/`
- 访问 `/media` 查看瀑布流展示

支持格式：
- 照片：JPG, JPEG, PNG, GIF, WEBP
- 视频：MP4, WEBM, MOV

## 🛠️ 技术栈

| 组件 | 版本 | 说明 |
|------|------|------|
| Axum | 0.8 | Web 框架 |
| Tokio | 1 | 异步运行时 |
| pulldown-cmark | 0.13 | Markdown 解析 |
| tower-http | 0.6 | 静态文件服务 |

## 📋 部署 TODO

- [ ] 使用花生壳内网穿透暴露服务到公网
- [ ] 将网站推送到生产环境

## 📄 License

MIT
