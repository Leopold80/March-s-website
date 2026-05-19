# 林夏的个人网站 (Lin Xia's Personal Website)

用 Rust + Axum 构建的个人网站，支持 Markdown 日志和媒体墙功能。

## 项目结构

```
marchs-website/
├── assets/              # HTML 模板文件
│   ├── index.html       # 首页模板
│   ├── logs.html        # 日志列表页模板
│   ├── log_post.html    # 日志阅读页模板
│   └── media.html       # 媒体墙模板
├── logs/                # Markdown 日志文件目录
├── media/               # 媒体文件目录
│   ├── photos/          # 照片 (JPG, PNG, GIF, WEBP)
│   └── videos/          # 视频 (MP4, WEBM, MOV)
├── src/
│   ├── main.rs          # 程序入口
│   ├── server.rs        # 服务器配置和路由
│   └── pages.rs         # 页面处理器
├── Cargo.toml           # Rust 依赖配置
└── QWEN.md              # 项目说明文档
```

## 技术栈

- **后端**: Rust + Axum 0.8
- **异步运行时**: Tokio
- **Markdown 解析**: pulldown-cmark 0.13
- **静态文件服务**: tower-http 0.6

## 构建和运行

```bash
# 开发模式运行
cargo run

# 发布模式构建
cargo build --release

# 运行测试（如有）
cargo test
```

服务器默认监听 `http://0.0.0.0:3000`

## 页面路由

| 路径 | 说明 |
|------|------|
| `/` | 首页 |
| `/logs` | 日志列表页 |
| `/log/{slug}` | 单篇日志阅读页 |
| `/media` | 媒体墙（瀑布流布局） |
| `/media/photos/{filename}` | 照片静态资源 |
| `/media/videos/{filename}` | 视频静态资源 |

## 添加日志

在 `logs/` 目录下创建 `.md` 文件，格式如下：

```markdown
---
title: 日志标题
date: 2024-01-15
---

日志正文内容，支持 Markdown 语法：

- 列表
- **粗体**
- `代码`
- 代码块
- 表格等
```

文件名格式建议：`YYYY-MM-DD-slug.md`（如 `2024-01-15-hello.md`）

## 添加媒体

- **照片**: 放入 `media/photos/` 目录（支持 JPG, JPEG, PNG, GIF, WEBP）
- **视频**: 放入 `media/videos/` 目录（支持 MP4, WEBM, MOV）

媒体墙会自动扫描并显示所有媒体文件，按瀑布流布局排列。

## 代码设计说明

### 模块划分

- `main.rs`: 程序入口，启动 Tokio 异步运行时
- `server.rs`: 路由配置和服务器启动逻辑
- `pages.rs`: 页面处理器（路由处理函数）

### 设计理念

代码注释中使用"无人机飞控"类比来解释 Web 概念，便于有嵌入式/算法背景的开发者理解：

- Router → 指令分发器
- HTTP 请求 → 遥测请求
- 异步处理 → 多任务并发处理

### 静态资源处理

- HTML 模板使用 `include_str!()` 编译时嵌入（零运行时开销）
- 媒体文件使用 `tower-http::ServeDir` 运行时提供（支持大文件）

## 注意事项

1. 修改 HTML 模板后需要重新编译
2. 修改 `logs/` 或 `media/` 目录内容无需重新编译，刷新页面即可
3. MOV 视频格式在某些浏览器可能不兼容，建议优先使用 MP4 格式
