# OpenFang 汉化版

> 由 **若枫贺昂** 使用 AI 工作流完成中文本地化

## 🎯 项目简介

OpenFang 是一个开源的智能体操作系统（Agent OS），本项目为其完整的中文本地化版本。

### 原项目地址
- 原项目: [RightNow-AI/openfang](https://github.com/RightNow-AI/openfang)

## ✨ 汉化内容

### 已汉化的页面
- ✅ 侧边栏导航菜单
- ✅ 聊天界面 (Chat)
- ✅ 概览页面 (Overview)
- ✅ 统计页面 (Usage)
- ✅ 日志页面 (Logs)
- ✅ 会话管理 (Sessions)
- ✅ 审批系统 (Approvals)
- ✅ 工作流管理 (Workflows)
- ✅ 调度器 (Scheduler)
- ✅ 频道配置 (Channels)
- ✅ 技能管理 (Skills)
- ✅ 手部工具 (Hands)
- ✅ 设置页面 (Settings)
- ✅ 设置向导 (Wizard)
- ✅ 工作流构建器 (Workflow Builder)
- ✅ 智能体管理 (Agents)

### 汉化统计
- 翻译条目: 500+ 处
- 涉及文件: 17 个前端文件
- 修复变量名: 50+ 处

## 🚀 快速开始

### 方式一：直接运行编译好的程序

```bash
# 下载 release 目录下的可执行文件
./openfang.exe init    # 初始化配置
./openfang.exe start   # 启动服务
```

### 方式二：从源码编译

```bash
# 克隆仓库
git clone https://github.com/rfdiosuao/openfang-cn.git
cd openfang-cn

# 编译项目 (需要 Rust 环境)
cargo build --release

# 运行
./target/release/openfang.exe init
./target/release/openfang.exe start
```

启动后访问 http://127.0.0.1:4200 即可看到汉化后的界面。

## 📦 可执行文件位置

编译后的可执行文件位于：
- CLI版本: `target/release/openfang.exe`
- 桌面版: `target/release/openfang-desktop.exe`

## 🛠️ 技术栈

- **后端**: Rust + Tauri 2.0
- **前端**: Alpine.js + Tailwind CSS
- **数据库**: SQLite

## 📝 汉化说明

本项目使用 AI 工作流进行自动化汉化，遵循以下原则：
1. 只翻译用户可见的字符串
2. 保持 JavaScript 变量名、函数名不变
3. 翻译通俗自然，符合中文习惯
4. 保持代码结构完整

## 👤 贡献者

- **汉化工作**: 若枫贺昂 (使用 AI 工作流)
- **原项目**: [RightNow-AI](https://github.com/RightNow-AI)

## 📄 许可证

本项目遵循原项目的开源许可证。

---

**注意**: 本项目为 OpenFang 的中文本地化版本，核心功能与原项目保持一致。
