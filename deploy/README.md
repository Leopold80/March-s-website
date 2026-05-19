# 部署脚本原理说明

## 目录结构

```
deploy/
├── deploy.py                  # Python 部署脚本
├── marchs-website.service     # systemd 服务模板
└── README.md                  # 本文件
```

---

## 部署原理

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                     部署流程概览                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. 编译阶段 (cargo build --release)                        │
│     源代码 → Rust 编译器 → 二进制文件                         │
│                                                             │
│  2. 部署阶段 (deploy.py)                                    │
│     二进制文件 + assets → 部署目录 (~/marchs-website)        │
│                                                             │
│  3. 服务注册 (systemd)                                      │
│     marchs-website.service → ~/.config/systemd/user/       │
│                                                             │
│  4. 服务启动                                                │
│     systemd → 自动运行二进制文件 → HTTP 服务                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 为什么需要部署脚本？

直接运行 `cargo run` 的问题：
- 终端关闭后服务停止
- 无法开机自启
- 崩溃后不会自动重启
- 日志分散，难以查看

部署脚本解决的问题：
- **二进制分离**: 编译产物与源码分离，便于管理
- **服务化**: 使用 systemd 管理进程生命周期
- **持久化**: 开机自启、崩溃重启
- **日志集中**: 通过 `journalctl` 统一查看日志

---

## 核心组件详解

### 1. deploy.py - 部署脚本

#### 功能模块

```python
# 模块调用顺序
main()
├── check_prerequisites()      # 检查 Rust、cargo、systemd
├── build_release()            # cargo build --release
├── create_deploy_directory()  # 创建目录结构
├── copy_files()               # 复制二进制文件和 assets
├── generate_config()          # 生成 config.toml
├── install_systemd_service()  # 注册 systemd 服务
├── verify_service()           # 验证服务状态
└── print_summary()            # 打印部署摘要
```

#### 目录结构创建

```python
dirs_to_create = [
    DEPLOY_DIR / "bin",           # 存放编译后的二进制文件
    DEPLOY_DIR / "assets",        # HTML 模板
    DEPLOY_DIR / "md_notes",      # Markdown 日志（数据目录，升级保留）
    DEPLOY_DIR / "media/photos",  # 照片（数据目录，升级保留）
    DEPLOY_DIR / "media/videos",  # 视频（数据目录，升级保留）
]
```

#### 关键设计

**1. 二进制与源码分离**

```
源码目录：/path/to/source/
├── src/
├── Cargo.toml
└── target/release/marchs-website  ← 编译产物

部署目录：~/marchs-website/
├── bin/marchs-website             ← 复制后的二进制（独立运行）
├── assets/
├── md_notes/                      ← 数据目录（升级时保留）
└── media/                         ← 数据目录（升级时保留）
```

**2. systemd 服务模板替换**

部署脚本会读取 `marchs-website.service` 模板，并替换占位符：

```python
# 模板中的占位符
WorkingDirectory=/home/pi/marchs-website  →  替换为实际部署路径
User=pi                                    →  替换为当前用户名
```

**3. 升级保护数据**

```python
# 数据目录在已存在时不会删除
if d.exists():
    print(f"✓ {d.relative_to(DEPLOY_DIR)} (已存在，保留)")
else:
    d.mkdir(parents=True, exist_ok=True)
    print(f"✓ {d.relative_to(DEPLOY_DIR)} (新建)")
```

---

### 2. marchs-website.service - systemd 服务

#### 服务配置解析

```ini
[Unit]
Description=林夏的个人网站 (Lin Xia's Personal Website)
After=network.target
# ↑ 在网络就绪后启动，确保能绑定端口
```

```ini
[Service]
Type=simple
# ↑ 简单服务，直接执行 ExecStart

User=pi
# ↑ 以普通用户身份运行（非 root，安全）

WorkingDirectory=/home/pi/marchs-website
# ↑ 工作目录，二进制文件从此处读取 assets 和 md_notes

ExecStart=/home/pi/marchs-website/bin/marchs-website
# ↑ 实际执行的二进制文件路径

Restart=always
# ↑ 崩溃后自动重启（关键！）

RestartSec=5
# ↑ 重启前等待 5 秒，避免崩溃循环占用过多资源

StandardOutput=journal
StandardError=journal
# ↑ 日志输出到 systemd journal，可用 journalctl 查看
```

```ini
[Install]
WantedBy=default.target
# ↑ 用户级服务的目标（相当于系统级的 multi-user.target）
```

#### systemd 层级说明

```
系统级服务 (/etc/systemd/system/)
├── 需要 root 权限
└── 开机自启（用户登录前）

用户级服务 (~/.config/systemd/user/)
├── 无需 root 权限
└── 用户登录后自启
```

本项目使用**用户级服务**，原因：
- 无需 root 权限，更安全
- 适合个人设备部署
- 日志隔离（`journalctl --user`）

---

## 使用指南

### 首次部署

```bash
# 在源码目录执行
cd ~/marchs-website
python deploy.py
```

执行流程：
1. 检查 Rust 环境
2. 编译 release 版本（`cargo build --release`）
3. 创建部署目录 `~/marchs-website/`
4. 复制二进制文件和 assets
5. 生成 `config.toml` 配置文件
6. 注册 systemd 服务
7. 启动服务

### 更新部署

```bash
# 拉取最新代码后
git pull
python deploy.py --update
```

`--update` 模式的行为：
- 覆盖 `bin/marchs-website`（新版本二进制）
- 覆盖 `assets/`（新模板）
- **保留** `md_notes/`（用户数据）
- **保留** `media/`（用户上传的媒体）
- 自动重启服务

### 服务管理

```bash
# 查看状态
systemctl --user status marchs-website

# 重启服务
systemctl --user restart marchs-website

# 停止服务
systemctl --user stop marchs-website

# 启动服务
systemctl --user start marchs-website

# 禁用开机自启
systemctl --user disable marchs-website

# 启用开机自启
systemctl --user enable marchs-website
```

### 日志查看

```bash
# 实时日志（类似 tail -f）
journalctl --user -u marchs-website -f

# 最近 50 条
journalctl --user -u marchs-website -n 50

# 今天以来的日志
journalctl --user -u marchs-website --since today

# 特定时间段
journalctl --user -u marchs-website --since "2024-01-01" --until "2024-01-02"
```

---

## 故障排查

### 服务无法启动

```bash
# 查看详细错误
systemctl --user status marchs-website

# 检查端口是否被占用
lsof -i :3000

# 手动运行二进制文件调试
~/marchs-website/bin/marchs-website
```

### 编译失败

```bash
# 清理编译缓存
cargo clean

# 重新编译
cargo build --release
```

### systemd 服务未生效

```bash
# 重载 systemd 配置
systemctl --user daemon-reload

# 检查服务文件是否存在
ls -la ~/.config/systemd/user/marchs-website.service
```

---

## 部署流程图

```
┌─────────────┐
│  git pull   │  更新源码
└──────┬──────┘
       │
       ▼
┌─────────────────────────────┐
│  python deploy.py --update  │
└──────┬──────────────────────┘
       │
       ├──→ 1. cargo build --release
       │      target/release/marchs-website
       │
       ├──→ 2. 复制二进制到 ~/marchs-website/bin/
       │
       ├──→ 3. 覆盖 assets/
       │
       ├──→ 4. 保留 md_notes/ 和 media/
       │
       ├──→ 5. systemctl --user daemon-reload
       │
       ├──→ 6. systemctl --user restart marchs-website
       │
       └──→ ✅ 部署完成
```

---

## 常见问题

**Q: 为什么不直接用 `cargo run`？**

A: `cargo run` 适合开发调试，但生产环境需要：
- 进程守护（崩溃重启）
- 日志管理
- 开机自启
- 二进制与源码分离（便于版本管理）

**Q: 部署后原来的源码目录还能用吗？**

A: 可以。部署目录和源码目录是独立的：
- 源码目录：编译、开发
- 部署目录：运行服务

**Q: 如何回退到旧版本？**

A: 使用 git 回退后重新部署：
```bash
git checkout <commit-hash>
python deploy.py --update
```

**Q: 数据目录具体包含哪些？**

A: 以下目录在 `--update` 时保留：
- `md_notes/` - Markdown 日志
- `media/photos/` - 照片
- `media/videos/` - 视频

**Q: 如何完全卸载？**

A: 
```bash
# 停止并禁用服务
systemctl --user stop marchs-website
systemctl --user disable marchs-website

# 删除服务文件
rm ~/.config/systemd/user/marchs-website.service

# 删除部署目录
rm -rf ~/marchs-website
```

---

## 技术总结

| 组件 | 作用 | 类比 |
|------|------|------|
| `cargo build --release` | 编译优化后的二进制 | 类似无人机飞控固件编译 |
| `deploy.py` | 自动化部署流程 | 类似固件烧录工具 |
| `systemd` | 进程管理和服务化 | 类似飞控中的看门狗 + 任务调度器 |
| `journalctl` | 日志查看 | 类似飞控黑匣子日志 |

---

## 参考链接

- [systemd 用户服务](https://wiki.archlinux.org/title/Systemd/User)
- [journalctl 使用指南](https://www.man7.org/linux/man-pages/man1/journalctl.1.html)
- [Rust 发布模式优化](https://doc.rust-lang.org/cargo/reference/profiles.html#release)
