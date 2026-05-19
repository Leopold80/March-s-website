#!/usr/bin/env python3
"""
林夏的个人网站 - 卸载脚本

用法:
    python uninstall.py              # 完全卸载
    python uninstall.py --keep-data  # 保留数据目录
"""

import os
import sys
import shutil
import subprocess
from pathlib import Path

# 配置
CURRENT_USER = os.environ.get('USER', 'pi')
DEPLOY_DIR = Path.home() / "marchs-website"
SERVICE_NAME = "marchs-website"

def print_step(msg: str):
    print(f"\n{'='*60}")
    print(f"  {msg}")
    print(f"{'='*60}\n")

def confirm_uninstall(keep_data: bool):
    """确认卸载"""
    mode = "保留数据" if keep_data else "完全卸载"
    print(f"""
╔═══════════════════════════════════════════════════════════╗
║           林夏的个人网站 - 卸载脚本                        ║
╠═══════════════════════════════════════════════════════════╣
║  当前用户：{CURRENT_USER:<45} ║
║  部署目录：{str(DEPLOY_DIR):<45} ║
║  卸载模式：{mode:<45} ║
╚═══════════════════════════════════════════════════════════╝

{'⚠️  警告：此操作将删除以下服务配置和文件！' if not keep_data else '⚠️  警告：此操作将删除服务，但保留数据目录'}

将被删除:
  - systemd 服务配置
  - bin/marchs-website (二进制文件)
  - assets/ (HTML 模板)
  - config.toml (配置文件)
  {'  - md_notes/ (日志数据)' if not keep_data else '  ✓ md_notes/ (保留)'}
  {'  - media/ (照片和视频)' if not keep_data else '  ✓ media/ (保留)'}
""")
    
    response = input("确认继续？[y/N]: ").strip().lower()
    if response in ('y', 'yes'):
        return True
    else:
        print("卸载已取消")
        return False

def stop_service():
    """停止服务"""
    print_step("停止服务")
    
    # 检查服务是否存在
    result = subprocess.run(
        ["systemctl", "--user", "is-active", SERVICE_NAME],
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        subprocess.run(["systemctl", "--user", "stop", SERVICE_NAME], check=False)
        print("✓ 服务已停止")
    else:
        print("ℹ 服务未运行")

def disable_service():
    """禁用服务"""
    print_step("禁用服务")
    
    result = subprocess.run(
        ["systemctl", "--user", "is-enabled", SERVICE_NAME],
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        subprocess.run(["systemctl", "--user", "disable", SERVICE_NAME], check=False)
        print("✓ 服务已禁用（开机不自启）")
    else:
        print("ℹ 服务未启用")

def remove_service_file():
    """删除 systemd 服务文件"""
    print_step("删除 systemd 服务文件")
    
    systemd_user_dir = Path.home() / ".config" / "systemd" / "user"
    service_file = systemd_user_dir / f"{SERVICE_NAME}.service"
    
    if service_file.exists():
        service_file.unlink()
        print(f"✓ 已删除：{service_file}")
        
        # 重载 systemd 配置
        subprocess.run(["systemctl", "--user", "daemon-reload"], check=False)
        print("✓ systemd 配置已重载")
    else:
        print("ℹ 服务文件不存在")

def remove_deploy_directory(keep_data: bool):
    """删除部署目录"""
    print_step("删除部署目录")
    
    if not DEPLOY_DIR.exists():
        print("ℹ 部署目录不存在")
        return
    
    if keep_data:
        # 保留数据目录，只删除其他文件
        files_to_delete = [
            DEPLOY_DIR / "bin",
            DEPLOY_DIR / "assets",
            DEPLOY_DIR / "config.toml",
        ]
        
        for f in files_to_delete:
            if f.exists():
                if f.is_dir():
                    shutil.rmtree(f)
                else:
                    f.unlink()
                print(f"✓ 已删除：{f.relative_to(DEPLOY_DIR)}")
        
        print("✓ 数据目录已保留:")
        print(f"    - {DEPLOY_DIR / 'md_notes'}")
        print(f"    - {DEPLOY_DIR / 'media'}")
    else:
        # 完全删除
        shutil.rmtree(DEPLOY_DIR)
        print(f"✓ 已删除整个部署目录：{DEPLOY_DIR}")

def verify_uninstall():
    """验证卸载是否完成"""
    print_step("验证卸载结果")
    
    # 检查服务是否还存在
    result = subprocess.run(
        ["systemctl", "--user", "list-unit-files", SERVICE_NAME],
        capture_output=True,
        text=True
    )
    
    if SERVICE_NAME in result.stdout:
        print("⚠️  服务配置可能未完全删除")
    else:
        print("✓ 服务配置已删除")
    
    # 检查部署目录
    if DEPLOY_DIR.exists():
        print(f"⚠️  部署目录仍存在：{DEPLOY_DIR}")
        print("   (使用了 --keep-data 选项)")
    else:
        print("✓ 部署目录已删除")

def print_summary(keep_data: bool):
    """打印卸载摘要"""
    print_step("卸载完成！")
    
    mode = "保留数据" if keep_data else "完全卸载"
    print(f"""
卸载模式：{mode}

已删除:
  ✓ systemd 服务配置
  ✓ 二进制文件
  ✓ assets 目录
  ✓ config.toml
  {'' if keep_data else '✓ md_notes 目录'}
  {'' if keep_data else '✓ media 目录'}

{'数据目录已保留，可重新部署:' if keep_data else ''}
  {f'  python deploy.py' if keep_data else ''}
""")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="林夏的个人网站 - 卸载脚本")
    parser.add_argument(
        "--keep-data",
        action="store_true",
        help="保留数据目录（md_notes 和 media）"
    )
    args = parser.parse_args()
    
    if not confirm_uninstall(args.keep_data):
        sys.exit(0)
    
    # 执行卸载流程
    stop_service()
    disable_service()
    remove_service_file()
    remove_deploy_directory(args.keep_data)
    verify_uninstall()
    print_summary(args.keep_data)

if __name__ == "__main__":
    main()
