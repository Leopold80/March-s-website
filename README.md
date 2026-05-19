# 林夏的个人网站

用 Rust + Axum 构建的个人网站，支持 Markdown 日志和媒体墙功能。

## 📋 TODO

- [x] 基础网站功能（首页、日志、媒体墙）
- [x] Markdown 日志支持
- [x] 媒体墙瀑布流布局
- [x] HEIC 照片转 JPEG 支持
- [x] 使用花生壳 HTTPS 内网穿透实现外网访问
- [x] 创建部署级的服务器软件方案（systemd 服务、Docker 等）
- [x] 部署脚本和卸载脚本
- [ ] 图片缩略图优化
- [ ] 媒体墙分页

## ✨ 功能特性

- 📝 **Markdown 日志** - 在 `md_notes/` 目录放置 Markdown 文件即可发布日志
- 🖼️ **媒体墙** - 瀑布流布局展示照片和视频
- 🎨 **响应式设计** - 适配桌面、平板、手机
- 🚀 **高性能** - Rust 异步服务器，编译时模板嵌入
- 🌐 **内网穿透** - 已配置花生壳，支持外网访问

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
├── md_notes/         # Markdown 日志文件
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

1. 在 `md_notes/` 目录创建 `.md` 文件
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

## 📋 部署

### 🚀 一键部署（推荐）

适合树莓派/个人服务器，自动配置 systemd 服务：

```bash
# 首次部署
python deploy/deploy.py

# 后续更新（保留 md_notes 和 media 数据）
python deploy/deploy.py --update
```

部署后自动：
- ✅ 编译 release 版本
- ✅ 安装到 `~/marchs-website`
- ✅ 配置 systemd 服务（开机自启、崩溃重启）
- ✅ 保留用户数据目录

### 🗑️ 卸载

```bash
# 完全卸载（删除所有文件和数据）
python deploy/uninstall.py

# 保留数据卸载（只删除程序，保留日志和媒体）
python deploy/uninstall.py --keep-data
```

### 服务管理

```bash
# 查看状态
systemctl --user status marchs-website

# 重启服务
systemctl --user restart marchs-website

# 查看日志
journalctl --user -u marchs-website -f

# 停止服务
systemctl --user stop marchs-website
```

详细部署说明见 [deploy/README.md](deploy/README.md)

### 手动部署

```bash
# 编译
cargo build --release

# 运行
cargo run --release
```

### 云服务器部署

如需更稳定的生产环境，可部署到阿里云/腾讯云轻量应用服务器。

## 📄 License

MIT
