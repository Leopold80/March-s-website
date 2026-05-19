#!/usr/bin/env python3
"""
林夏的个人网站 - 部署脚本

用法:
    python deploy.py              # 首次部署
    python deploy.py --update     # 更新（保留数据）
"""

import os
import sys
import shutil
import subprocess
import argparse
from pathlib import Path

# 配置
CURRENT_USER = os.environ.get('USER', 'pi')
DEPLOY_DIR = Path.home() / "marchs-website"
SERVICE_NAME = "marchs-website"
PORT = 3000

def print_step(msg: str):
    print(f"\n{'='*60}")
    print(f"  {msg}")
    print(f"{'='*60}\n")

def check_prerequisites():
    """检查环境依赖"""
    print_step("检查环境依赖")
    
    # 检查 Rust
    result = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
    if result.returncode != 0:
        print("❌ 未检测到 Rust，请先安装：https://rustup.rs/")
        sys.exit(1)
    print(f"✓ Rust: {result.stdout.strip()}")
    
    # 检查 cargo
    result = subprocess.run(["cargo", "--version"], capture_output=True, text=True)
    if result.returncode != 0:
        print("❌ 未检测到 cargo")
        sys.exit(1)
    print(f"✓ Cargo: {result.stdout.strip()}")
    
    # 检查 systemd
    if not Path("/usr/bin/systemctl").exists():
        print("⚠️  未检测到 systemd，服务自动启动可能不可用")
    else:
        print("✓ systemd: 可用")

def build_release():
    """编译 release 版本"""
    print_step("编译 release 版本")
    result = subprocess.run(
        ["cargo", "build", "--release"],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        print("❌ 编译失败:")
        print(result.stderr)
        sys.exit(1)
    print("✓ 编译成功")

def create_deploy_directory():
    """创建部署目录结构"""
    print_step("创建部署目录")
    
    dirs_to_create = [
        DEPLOY_DIR / "bin",
        DEPLOY_DIR / "assets",
        DEPLOY_DIR / "md_notes",
        DEPLOY_DIR / "media" / "photos",
        DEPLOY_DIR / "media" / "videos",
    ]
    
    for d in dirs_to_create:
        if d.exists():
            print(f"✓ {d.relative_to(DEPLOY_DIR)} (已存在，保留)")
        else:
            d.mkdir(parents=True, exist_ok=True)
            print(f"✓ {d.relative_to(DEPLOY_DIR)} (新建)")

def copy_files():
    """复制文件"""
    print_step("复制文件")
    
    # 复制二进制文件
    src_bin = Path("target/release/marchs-website")
    dst_bin = DEPLOY_DIR / "bin" / "marchs-website"
    if src_bin.exists():
        shutil.copy2(src_bin, dst_bin)
        print(f"✓ 二进制文件 → {dst_bin}")
    else:
        print(f"❌ 未找到编译产物：{src_bin}")
        sys.exit(1)
    
    # 复制 assets（覆盖旧的）
    src_assets = Path("assets")
    dst_assets = DEPLOY_DIR / "assets"
    if src_assets.exists():
        if dst_assets.exists():
            shutil.rmtree(dst_assets)
        shutil.copytree(src_assets, dst_assets)
        print(f"✓ assets → {dst_assets}")
    
    # 复制 systemd 服务文件（替换占位符）
    src_service = Path("deploy/marchs-website.service")
    if src_service.exists():
        systemd_user_dir = Path.home() / ".config" / "systemd" / "user"
        systemd_user_dir.mkdir(parents=True, exist_ok=True)
        dst_service = systemd_user_dir / f"{SERVICE_NAME}.service"

        # 读取模板并替换占位符
        content = src_service.read_text()
        content = content.replace("/home/pi/marchs-website", str(DEPLOY_DIR))
        content = content.replace("User=pi", f"User={CURRENT_USER}")
        dst_service.write_text(content)
        print(f"✓ systemd 服务 → {dst_service}")
    else:
        print("⚠️  未找到 systemd 服务模板")

def generate_config():
    """生成配置文件"""
    print_step("生成配置文件")
    
    config_path = DEPLOY_DIR / "config.toml"
    if not config_path.exists():
        config_content = f"""# 林夏的个人网站 - 配置文件

[server]
port = {PORT}
host = "0.0.0.0"

[media]
photos_dir = "media/photos"
videos_dir = "media/videos"

[logs]
notes_dir = "md_notes"
"""
        config_path.write_text(config_content)
        print(f"✓ 配置文件 → {config_path}")
    else:
        print(f"✓ 配置文件已存在，跳过")

def install_systemd_service():
    """安装并启用 systemd 服务"""
    print_step("安装 systemd 服务")

    # 重载 systemd 配置（必须先 reload 才能识别新服务）
    subprocess.run(["systemctl", "--user", "daemon-reload"], check=True)
    print("✓ systemd 配置已重载")

    # 启用服务
    result = subprocess.run(
        ["systemctl", "--user", "enable", SERVICE_NAME],
        capture_output=True,
        text=True
    )
    if result.returncode == 0:
        print(f"✓ 服务已启用（开机自启）")
    else:
        print(f"⚠️  启用服务失败：{result.stderr}")

    # 启动服务
    result = subprocess.run(
        ["systemctl", "--user", "start", SERVICE_NAME],
        capture_output=True,
        text=True
    )
    if result.returncode == 0:
        print(f"✓ 服务已启动")
    else:
        print(f"⚠️  启动服务失败：{result.stderr}")

def verify_service():
    """验证服务是否正常运行"""
    print_step("验证服务状态")
    
    result = subprocess.run(
        ["systemctl", "--user", "is-active", SERVICE_NAME],
        capture_output=True,
        text=True
    )
    if result.stdout.strip() == "active":
        print(f"✓ 服务运行正常")
    else:
        print(f"⚠️  服务状态：{result.stdout.strip()}")

def print_summary():
    """打印部署摘要"""
    print_step("✅ 部署完成！")
    
    print(f"""
部署位置：{DEPLOY_DIR}
服务端口：{PORT}

访问地址:
  - 本地：http://localhost:{PORT}
  - 局域网：http://{get_local_ip()}:{PORT}

管理命令:
  systemctl --user status {SERVICE_NAME}    # 查看状态
  systemctl --user restart {SERVICE_NAME}   # 重启
  systemctl --user stop {SERVICE_NAME}      # 停止
  systemctl --user disable {SERVICE_NAME}   # 禁用开机自启
  
日志查看:
  journalctl --user -u {SERVICE_NAME} -f    # 实时日志
  journalctl --user -u {SERVICE_NAME} -n 50 # 最近 50 条

后续更新:
  1. git pull 拉取最新代码
  2. python deploy.py --update 重新部署
  
数据目录（升级时保留）:
  - {DEPLOY_DIR}/md_notes/   # 笔记
  - {DEPLOY_DIR}/media/      # 照片和视频
""")

def get_local_ip():
    """获取本机 IP"""
    import socket
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(("8.8.8.8", 80))
        ip = s.getsockname()[0]
        s.close()
        return ip
    except:
        return "127.0.0.1"

def confirm_deployment():
    """确认部署配置"""
    print(f"\n部署配置:")
    print(f"  当前用户：{CURRENT_USER}")
    print(f"  部署目录：{DEPLOY_DIR}")
    print(f"  服务端口：{PORT}")
    print()
    
    response = input("是否继续部署？[Y/n]: ").strip().lower()
    if response in ('', 'y', 'yes'):
        return True
    elif response in ('n', 'no'):
        print("部署已取消")
        return False
    else:
        return confirm_deployment()

def main():
    parser = argparse.ArgumentParser(description="林夏的个人网站 - 部署脚本")
    parser.add_argument("--update", action="store_true", help="更新现有部署（保留数据）")
    args = parser.parse_args()

    if args.update and not DEPLOY_DIR.exists():
        print("⚠️  未检测到现有部署，执行首次部署...")

    if not confirm_deployment():
        sys.exit(0)

    print(f"""
╔═══════════════════════════════════════════════════════════╗
║           林夏的个人网站 - 部署脚本                        ║
╠═══════════════════════════════════════════════════════════╣
║  部署位置：{str(DEPLOY_DIR):<45} ║
║  更新模式：{'是' if args.update else '否':<45} ║
╚═══════════════════════════════════════════════════════════╝
""")
    
    # 执行部署流程
    check_prerequisites()
    build_release()
    create_deploy_directory()
    copy_files()
    generate_config()
    install_systemd_service()
    verify_service()
    print_summary()

if __name__ == "__main__":
    main()
