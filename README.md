<p align="center"><strong>简体中文</strong> · <a href="README.en.md">English</a></p>

<p align="center">
  <img src="public/assets/openfang-logo.png" width="112" alt="OpenFang logo">
</p>

<h1 align="center">OpenFang 中文版</h1>

<p align="center">面向中文用户维护的开源 Agent OS 本地化版本</p>

<p align="center">
  <a href="https://github.com/rfdiosuao/openfang-cn/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/rfdiosuao/openfang-cn/ci.yml?branch=main&style=flat-square&label=CI"></a>
  <a href="https://github.com/rfdiosuao/openfang-cn/releases"><img alt="Release" src="https://img.shields.io/github/v/release/rfdiosuao/openfang-cn?style=flat-square"></a>
  <a href="https://github.com/rfdiosuao/openfang-cn/stargazers"><img alt="Stars" src="https://img.shields.io/github/stars/rfdiosuao/openfang-cn?style=flat-square&logo=github"></a>
  <a href="LICENSE-MIT"><img alt="License" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square"></a>
  <a href="https://github.com/rfdiosuao/openfang-cn/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/rfdiosuao/openfang-cn?style=flat-square"></a>
</p>

<p align="center">
  <a href="#-快速开始">快速开始</a> ·
  <a href="#-汉化范围">汉化范围</a> ·
  <a href="docs/README.md">项目文档</a> ·
  <a href="CONTRIBUTING.md">参与贡献</a> ·
  <a href="https://github.com/RightNow-AI/openfang">上游项目</a>
</p>

OpenFang 是一个开源智能体操作系统。本仓库在保留核心能力的基础上，持续维护中文界面、中文表达和本地化体验，让中文用户更容易安装、理解和使用 OpenFang。

> [!IMPORTANT]
> 这是由社区维护的中文本地化仓库，并非 OpenFang 官方中文发行版。核心架构、功能演进与安全公告请同时参考上游项目 [RightNow-AI/openfang](https://github.com/RightNow-AI/openfang)。

## ✨ 为什么使用这个版本

- **中文界面**：覆盖主要控制台页面和常用操作路径。
- **保留上游能力**：不改变核心架构，尽量让本地化补丁保持可追踪。
- **完整源码**：后端、前端、SDK、示例和部署资料均保留在仓库中。
- **可参与维护**：可以通过 Issue 报告漏译、术语问题和版本差异。

## 🌏 汉化范围

当前已覆盖 500+ 条用户可见文本，主要包括：

| 类别 | 已覆盖内容 |
| --- | --- |
| 核心界面 | 概览、聊天、统计、日志、会话与设置 |
| Agent 管理 | Agents、Skills、Hands、Channels |
| 自动化 | Workflows、Scheduler、Approvals、Workflow Builder |
| 初始化体验 | Wizard、导航菜单和常用提示信息 |

本地化遵循四项原则：只翻译用户可见文本；不翻译代码标识符；术语在同一功能域保持一致；与上游差异尽量控制在可审查范围内。

## 🚀 快速开始

### 环境要求

- Rust stable（版本以 [`rust-toolchain.toml`](rust-toolchain.toml) 为准）
- Git
- 桌面构建所需的系统依赖；Linux 用户可参考 [安装文档](docs/getting-started.md)

### 从源码构建

```bash
git clone https://github.com/rfdiosuao/openfang-cn.git
cd openfang-cn
cargo build --release
```

初始化并启动：

```bash
./target/release/openfang init
./target/release/openfang start
```

Windows PowerShell：

```powershell
.\target\release\openfang.exe init
.\target\release\openfang.exe start
```

启动后访问 <http://127.0.0.1:4200>。生产部署前请阅读 [配置说明](docs/configuration.md)、[安全说明](SECURITY.md) 与 [生产检查表](docs/production-checklist.md)。

## 🧭 文档导航

| 目标 | 文档 |
| --- | --- |
| 第一次安装 | [Getting Started](docs/getting-started.md) |
| 配置模型与服务 | [Configuration](docs/configuration.md) |
| 了解系统结构 | [Architecture](docs/architecture.md) |
| 开发 Agent Skill | [Skill Development](docs/skill-development.md) |
| 排查常见问题 | [Troubleshooting](docs/troubleshooting.md) |
| 查看版本变化 | [CHANGELOG](CHANGELOG.md) |

## 🤝 参与贡献

欢迎参与三类贡献：

1. 在 [漏译 / 术语 Issue](https://github.com/rfdiosuao/openfang-cn/issues/new?template=translation.yml) 中报告具体页面、原文和建议译文。
2. 在 [Bug Issue](https://github.com/rfdiosuao/openfang-cn/issues/new?template=bug-report.yml) 中提供版本、平台和复现步骤。
3. 按照 [`CONTRIBUTING.md`](CONTRIBUTING.md) 提交代码、测试或可审查的本地化补丁。

提交前至少运行：

```bash
cargo fmt --check
cargo check --workspace
```

## 🔄 与上游的关系

- 上游仓库：[RightNow-AI/openfang](https://github.com/RightNow-AI/openfang)
- 中文版问题：[Issues](https://github.com/rfdiosuao/openfang-cn/issues)
- 中文版发布：[Releases](https://github.com/rfdiosuao/openfang-cn/releases)

发现属于核心功能的问题时，建议先确认上游是否已经修复；适合通用解决的问题也欢迎优先贡献到上游，再同步回中文版本。

## 📄 许可证

本项目沿用上游双许可证：可选择 [MIT License](LICENSE-MIT) 或 [Apache License 2.0](LICENSE-APACHE)。第三方组件仍遵循各自的许可证和商标规则。

---

<p align="center">让优秀的 Agent 基础设施更容易被中文用户理解、使用和共同维护。</p>
