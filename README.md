<p align="center">
  <img src="public/assets/openfang-logo.png" width="160" alt="OpenFang 标志" />
</p>

<h1 align="center">OpenFang</h1>
<h3 align="center">智能代理操作系统</h3>

<p align="center">
  基于 Rust 构建的开源智能代理操作系统。137K 行代码。14 个 crate。1,767+ 个测试。零 Clippy 警告。<br/>
  <strong>单一二进制文件。经过实战验证。真正为你工作的智能代理。</strong>
</p>

<p align="center">
  <a href="https://openfang.sh/docs">文档</a> &bull;
  <a href="https://openfang.sh/docs/getting-started">快速开始</a> &bull;
  <a href="https://x.com/openfangg">Twitter / X</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/language-Rust-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="MIT" />
  <img src="https://img.shields.io/badge/version-0.3.30-green?style=flat-square" alt="v0.3.30" />
  <img src="https://img.shields.io/badge/tests-1,767%2B%20passing-brightgreen?style=flat-square" alt="测试" />
  <img src="https://img.shields.io/badge/clippy-0%20warnings-brightgreen?style=flat-square" alt="Clippy" />
  <a href="https://www.buymeacoffee.com/openfang" target="_blank"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=flat-square&logo=buy-me-a-coffee&logoColor=black" alt="请我喝杯咖啡" /></a>
</p>

---

> **v0.3.30 — 安全强化版本（2026年3月）**
>
> OpenFang 功能已完整，但仍处于 1.0 之前的版本。在小版本之间可能会遇到一些问题或破坏性更改。我们快速发布并快速修复。在 v1.0 之前，生产环境请固定到特定的 commit。[在此报告问题](https://github.com/RightNow-AI/openfang/issues)。

---

## 什么是 OpenFang？

OpenFang 是一个 **开源智能代理操作系统** —— 不是聊天机器人框架，不是围绕 LLM 的 Python 包装器，不是 "多代理编排器"。它是一个完整的自主代理操作系统，使用 Rust 从头构建。

传统的代理框架等待你输入指令。OpenFang 运行 **为你工作的自主代理** —— 按计划运行，24/7，构建知识图谱，监控目标，生成销售线索，管理你的社交媒体，并向你的仪表板报告结果。

整个系统编译成一个 **约 32MB 的单一二进制文件**。一次安装，一个命令，你的代理就会启动运行。

```bash
curl -fsSL https://openfang.sh/install | sh
openfang init
openfang start
# 仪表板运行在 http://localhost:4200
```

<details>
<summary><strong>Windows</strong></summary>

```powershell
irm https://openfang.sh/install.ps1 | iex
openfang init
openfang start
```

</details>

---

## Hands：真正做事的智能代理

<p align="center"><em>"传统代理等待你输入。Hands 为你工作。"</em></p>

**Hands** 是 OpenFang 的核心创新 —— 预构建的自主能力包，独立运行，按计划执行，无需你提示。这不是聊天机器人。这是一个会在早上 6 点醒来，研究你的竞争对手，构建知识图谱，对发现进行评分，并在你喝咖啡之前将报告发送到你的 Telegram 的代理。

每个 Hand 包含：
- **HAND.toml** —— 声明工具、设置、要求和仪表板指标的清单
- **系统提示** —— 多阶段操作手册（不是单行提示 —— 这些是 500+ 字的专家程序）
- **SKILL.md** —— 运行时注入上下文的领域专业知识参考
- **护栏** —— 敏感操作的审批门（例如，Browser Hand 在进行任何购买前需要审批）

所有内容都编译到二进制文件中。无需下载，无需 pip install，无需 Docker pull。

### 7 个内置 Hands

| Hand | 实际功能 |
|------|--------|
| **Clip** | 接收 YouTube URL，下载视频，识别最佳时刻，将其剪成带字幕和缩略图的垂直短片，可选添加 AI 配音，并发布到 Telegram 和 WhatsApp。8 阶段管道。FFmpeg + yt-dlp + 5 个 STT 后端。 |
| **Lead** | 每日运行。发现符合你的 ICP（理想客户档案）的潜在客户，通过网络研究丰富信息，评分 0-100，与现有数据库去重，并以 CSV/JSON/Markdown 格式提供合格的销售线索。随时间构建 ICP 档案。 |
| **Collector** | OSINT 级情报。你给它一个目标（公司、人物、主题）。它持续监控 —— 变更检测、情感跟踪、知识图谱构建，以及当重要事项发生变化时发出关键警报。 |
| **Predictor** | 超级预测引擎。从多个来源收集信号，构建校准的推理链，做出带置信区间的预测，并使用 Brier 分数跟踪自身准确性。具有故意反驳共识的逆向思维模式。 |
| **Researcher** | 深度自主研究人员。交叉引用多个来源，使用 CRAAP 标准（时效性、相关性、权威性、准确性、目的）评估可信度，生成带 APA 格式引用的报告，支持多种语言。 |
| **Twitter** | 自主 Twitter/X 账户管理器。以 7 种轮换格式创建内容，为最佳参与度安排帖子，回复提及，跟踪性能指标。有审批队列 —— 未经你的同意不会发布任何内容。 |
| **Browser** | 网络自动化代理。导航网站，填写表单，点击按钮，处理多步骤工作流。使用带有会话持久性的 Playwright 桥接。**强制购买审批门** —— 未经明确确认，它永远不会花费你的钱。 |

```bash
# 激活 Researcher Hand —— 它会立即开始工作
openfang hand activate researcher

# 随时检查其进度
openfang hand status researcher

# 按每日计划激活线索生成
openfang hand activate lead

# 暂停而不丢失状态
openfang hand pause lead

# 查看所有可用的 Hands
openfang hand list
```

**构建你自己的 Hand**。定义一个包含工具、设置和系统提示的 `HAND.toml`。发布到 FangHub。

---

## OpenFang 与其他框架对比

<p align="center">
  <img src="public/assets/openfang-vs-claws.png" width="600" alt="OpenFang vs OpenClaw vs ZeroClaw" />
</p>

### 基准测试：实测数据，而非营销宣传

所有数据来自官方文档和公共仓库 —— 2026 年 2 月。

#### 冷启动时间（越低越好）

```
ZeroClaw   ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   10 ms
OpenFang   ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  180 ms    ★
LangGraph  █████████████████░░░░░░░░░░░░░░░░░░░░░░░░░  2.5 sec
CrewAI     ████████████████████░░░░░░░░░░░░░░░░░░░░░░  3.0 sec
AutoGen    ██████████████████████████░░░░░░░░░░░░░░░░░  4.0 sec
OpenClaw   █████████████████████████████████████████░░  5.98 sec
```

#### 空闲内存使用（越低越好）

```
ZeroClaw   █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    5 MB
OpenFang   ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   40 MB    ★
LangGraph  ██████████████████░░░░░░░░░░░░░░░░░░░░░░░░░  180 MB
CrewAI     ████████████████████░░░░░░░░░░░░░░░░░░░░░░░  200 MB
AutoGen    █████████████████████████░░░░░░░░░░░░░░░░░░  250 MB
OpenClaw   ████████████████████████████████████████░░░░  394 MB
```

#### 安装大小（越低越好）

```
ZeroClaw   █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  8.8 MB
OpenFang   ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   32 MB    ★
CrewAI     ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  100 MB
LangGraph  ████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  150 MB
AutoGen    ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░  200 MB
OpenClaw   ████████████████████████████████████████░░░░  500 MB
```

#### 安全系统（越高越好）

```
OpenFang   ████████████████████████████████████████████   16      ★
ZeroClaw   ███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░    6
OpenClaw   ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    3
AutoGen    █████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    2
LangGraph  █████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    2
CrewAI     ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    1
```

#### 通道适配器（越高越好）

```
OpenFang   ████████████████████████████████████████████   40      ★
ZeroClaw   ███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░   15
OpenClaw   █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   13
CrewAI     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0
AutoGen    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0
LangGraph  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0
```

#### LLM 提供商（越高越好）

```
ZeroClaw   ████████████████████████████████████████████   28
OpenFang   ██████████████████████████████████████████░░   27      ★
LangGraph  ██████████████████████░░░░░░░░░░░░░░░░░░░░░   15
CrewAI     ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   10
OpenClaw   ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   10
AutoGen    ███████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    8
```

### 功能对比

| 功能 | OpenFang | OpenClaw | ZeroClaw | CrewAI | AutoGen | LangGraph |
|---------|----------|----------|----------|--------|---------|-----------|
| **语言** | **Rust** | TypeScript | **Rust** | Python | Python | Python |
| **自主 Hands** | **7 个内置** | 无 | 无 | 无 | 无 | 无 |
| **安全层** | **16 个独立系统** | 3 个基础 | 6 个层 | 1 个基础 | Docker | AES 加密 |
| **代理沙箱** | **WASM 双计量** | 无 | 允许列表 | 无 | Docker | 无 |
| **通道适配器** | **40** | 13 | 15 | 0 | 0 | 0 |
| **内置工具** | **53 + MCP + A2A** | 50+ | 12 | 插件 | MCP | LC 工具 |
| **内存** | **SQLite + 向量** | 文件基础 | SQLite FTS5 | 4 层 | 外部 | 检查点 |
| **桌面应用** | **Tauri 2.0** | 无 | 无 | 无 | Studio | 无 |
| **审计跟踪** | **Merkle 哈希链** | 日志 | 日志 | 追踪 | 日志 | 检查点 |
| **冷启动** | **<200ms** | ~6s | ~10ms | ~3s | ~4s | ~2.5s |
| **安装大小** | **~32 MB** | ~500 MB | ~8.8 MB | ~100 MB | ~200 MB | ~150 MB |
| **许可证** | MIT | MIT | MIT | MIT | Apache 2.0 | MIT |

---

## 16 个安全系统 —— 深度防御

OpenFang 不是事后添加安全措施。每一层都可以独立测试，并且在没有单点故障的情况下运行。

| # | 系统 | 功能 |
|---|--------|-------------|
| 1 | **WASM 双计量沙箱** | 工具代码在 WebAssembly 中运行，带有燃料计量 + 纪元中断。看门狗线程会终止失控代码。 |
| 2 | **Merkle 哈希链审计跟踪** | 每个动作都与前一个动作加密链接。篡改一个条目，整个链就会断裂。 |
| 3 | **信息流污点跟踪** | 标签通过执行传播 —— 从源到汇跟踪机密。 |
| 4 | **Ed25519 签名代理清单** | 每个代理身份和能力集都经过加密签名。 |
| 5 | **SSRF 保护** | 阻止私有 IP、云元数据端点和 DNS 重绑定攻击。 |
| 6 | **机密零化** | `Zeroizing<String>` 在 API 密钥不再需要的瞬间自动从内存中擦除它们。 |
| 7 | **OFP 相互认证** | 用于 P2P 网络的基于 HMAC-SHA256 随机数的恒定时间验证。 |
| 8 | **能力门** | 基于角色的访问控制 —— 代理声明所需工具，内核强制执行。 |
| 9 | **安全头** | 每个响应都包含 CSP、X-Frame-Options、HSTS、X-Content-Type-Options。 |
| 10 | **健康端点编辑** | 公共健康检查返回最小信息。完整诊断需要认证。 |
| 11 | **子进程沙箱** | `env_clear()` + 选择性变量传递。具有跨平台终止的进程树隔离。 |
| 12 | **提示注入扫描器** | 检测技能中的覆盖尝试、数据泄露模式和 shell 引用注入。 |
| 13 | **循环防护** | 基于 SHA256 的工具调用循环检测，带有断路器。处理乒乓模式。 |
| 14 | **会话修复** | 7 阶段消息历史验证和从损坏中自动恢复。 |
| 15 | **路径遍历预防** | 带有符号链接逃逸预防的规范化。`../` 在这里不起作用。 |
| 16 | **GCRA 速率限制器** | 具有每 IP 跟踪和过期清理的成本感知令牌桶速率限制。 |

---

## 架构

14 个 Rust crate。137,728 行代码。模块化内核设计。

```
openfang-kernel      编排、工作流、计量、RBAC、调度器、预算跟踪
openfang-runtime     代理循环、3 个 LLM 驱动程序、53 个工具、WASM 沙箱、MCP、A2A
openfang-api         140+ REST/WS/SSE 端点、OpenAI 兼容 API、仪表板
openfang-channels    40 个消息适配器，带有速率限制、DM/群组策略
openfang-memory      SQLite 持久化、向量嵌入、规范会话、压缩
openfang-types       核心类型、污点跟踪、Ed25519 清单签名、模型目录
openfang-skills      60 个捆绑技能、SKILL.md 解析器、FangHub 市场
openfang-hands       7 个自主 Hands、HAND.toml 解析器、生命周期管理
openfang-extensions  25 个 MCP 模板、AES-256-GCM 凭证库、OAuth2 PKCE
openfang-wire        带有 HMAC-SHA256 相互认证的 OFP P2P 协议
openfang-cli         带有守护进程管理、TUI 仪表板、MCP 服务器模式的 CLI
openfang-desktop     Tauri 2.0 原生应用（系统托盘、通知、全局快捷键）
openfang-migrate     OpenClaw、LangChain、AutoGPT 迁移引擎
xtask                构建自动化
```

---

## 40 个通道适配器

将你的代理连接到用户所在的每个平台。

**核心：** Telegram、Discord、Slack、WhatsApp、Signal、Matrix、Email（IMAP/SMTP）
**企业：** Microsoft Teams、Mattermost、Google Chat、Webex、飞书/Lark、Zulip
**社交：** LINE、Viber、Facebook Messenger、Mastodon、Bluesky、Reddit、LinkedIn、Twitch
**社区：** IRC、XMPP、Guilded、Revolt、Keybase、Discourse、Gitter
**隐私：** Threema、Nostr、Mumble、Nextcloud Talk、Rocket.Chat、Ntfy、Gotify
**工作场所：** Pumble、Flock、Twist、钉钉、Zalo、Webhooks

每个适配器支持每通道模型覆盖、DM/群组策略、速率限制和输出格式化。

---

## WhatsApp Web 网关（二维码）

通过二维码将你的个人 WhatsApp 账户连接到 OpenFang —— 就像 WhatsApp Web 一样。无需 Meta 商业账户。

### 先决条件

- 安装 **Node.js >= 18** ([下载](https://nodejs.org/))
- 已安装并初始化 OpenFang

### 设置

**1. 安装网关依赖：**

```bash
cd packages/whatsapp-gateway
npm install
```

**2. 配置 `config.toml`：**

```toml
[channels.whatsapp]
mode = "web"
default_agent = "assistant"
```

**3. 设置网关 URL（选择一种）：**

添加到你的 shell 配置文件以持久化：

```bash
# macOS / Linux
echo 'export WHATSAPP_WEB_GATEWAY_URL="http://127.0.0.1:3009"' >> ~/.zshrc
source ~/.zshrc
```

或在启动网关时内联设置：

```bash
export WHATSAPP_WEB_GATEWAY_URL="http://127.0.0.1:3009"
```

**4. 启动网关：**

```bash
node packages/whatsapp-gateway/index.js
```

网关默认监听端口 `3009`。使用 `WHATSAPP_GATEWAY_PORT` 覆盖。

**5. 启动 OpenFang：**

```bash
openfang start
# 仪表板在 http://localhost:4200
```

**6. 扫描二维码：**

打开仪表板 → **Channels** → **WhatsApp**。会出现一个二维码。用你的手机扫描：

> **WhatsApp** → **设置** → **链接设备** → **链接设备**

扫描后，状态变为 `connected`，传入消息将路由到你配置的代理。

### 网关环境变量

| 变量 | 描述 | 默认值 |
|----------|-------------|---------|
| `WHATSAPP_WEB_GATEWAY_URL` | OpenFang 连接的网关 URL | _(空 = 禁用)_ |
| `WHATSAPP_GATEWAY_PORT` | 网关监听的端口 | `3009` |
| `OPENFANG_URL` | 网关报告的 OpenFang API URL | `http://127.0.0.1:4200` |
| `OPENFANG_DEFAULT_AGENT` | 处理传入消息的代理 | `assistant` |

### 网关 API 端点

| 方法 | 路由 | 描述 |
|--------|-------|-------------|
| `POST` | `/login/start` | 生成二维码（返回 base64 PNG） |
| `GET` | `/login/status` | 连接状态（`disconnected`, `qr_ready`, `connected`） |
| `POST` | `/message/send` | 发送消息（`{ "to": "5511999999999", "text": "Hello" }`） |
| `GET` | `/health` | 健康检查 |

### 替代方案：WhatsApp Cloud API

对于生产工作负载，使用带有 Meta 商业账户的 [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api)。请参阅 [Cloud API 配置文档](https://openfang.sh/docs/channels/whatsapp)。



---

## 27 个 LLM 提供商 —— 123+ 模型

3 个原生驱动程序（Anthropic、Gemini、OpenAI 兼容）路由到 27 个提供商：

Anthropic、Gemini、OpenAI、Groq、DeepSeek、OpenRouter、Together、Mistral、Fireworks、Cohere、Perplexity、xAI、AI21、Cerebras、SambaNova、HuggingFace、Replicate、Ollama、vLLM、LM Studio、Qwen、MiniMax、智谱、月之暗面、千帆、Bedrock 等。

智能路由，带有任务复杂度评分、自动回退、成本跟踪和每模型定价。

---

## 从 OpenClaw 迁移

已经在运行 OpenClaw？一个命令即可：

```bash
# 迁移所有内容 —— 代理、内存、技能、配置
openfang migrate --from openclaw

# 从特定路径迁移
openfang migrate --from openclaw --path ~/.openclaw

# 先进行干运行，查看会发生什么变化
openfang migrate --from openclaw --dry-run
```

迁移引擎导入你的代理、对话历史、技能和配置。OpenFang 原生读取 SKILL.md，并且与 ClawHub 市场兼容。

---

## OpenAI 兼容 API

即插即用的替代品。将你现有的工具指向 OpenFang：

```bash
curl -X POST localhost:4200/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "researcher",
    "messages": [{"role": "user", "content": "分析 Q4 市场趋势"}],
    "stream": true
  }'
```

140+ REST/WS/SSE 端点，涵盖代理、内存、工作流、通道、模型、技能、A2A、Hands 等。

---

## 快速开始

```bash
# 1. 安装（macOS/Linux）
curl -fsSL https://openfang.sh/install | sh

# 2. 初始化 —— 引导你完成提供商设置
openfang init

# 3. 启动守护进程
openfang start

# 4. 仪表板运行在 http://localhost:4200

# 5. 激活一个 Hand —— 它开始为你工作
openfang hand activate researcher

# 6. 与代理聊天
openfang chat researcher
> "AI 代理框架的新兴趋势是什么？"

# 7. 生成一个预构建代理
openfang agent spawn coder
```

<details>
<summary><strong>Windows (PowerShell)</strong></summary>

```powershell
irm https://openfang.sh/install.ps1 | iex
openfang init
openfang start
```

</details>

---

## 开发

```bash
# 构建工作区
cargo build --workspace --lib

# 运行所有测试（1,767+）
cargo test --workspace

#  lint（必须为 0 警告）
cargo clippy --workspace --all-targets -- -D warnings

# 格式化
cargo fmt --all -- --check
```

---

## 稳定性说明

OpenFang v0.3.30 是 1.0 之前的版本。架构稳固，测试套件全面，安全模型完善。也就是说：

- 在 v1.0 之前，**破坏性更改**可能会在小版本之间发生
- **有些 Hands** 比其他更成熟（Browser 和 Researcher 是最经过实战验证的）
- **边缘情况**存在 —— 如果你发现一个，[打开一个 issue](https://github.com/RightNow-AI/openfang/issues)
- 在 v1.0 之前，**固定到特定的 commit** 用于生产部署

我们快速发布并快速修复。目标是在 2026 年年中发布一个坚如磐石的 v1.0。

---

## 安全

要报告安全漏洞，请发送电子邮件至 **jaber@rightnowai.co**。我们认真对待所有报告，并将在 48 小时内回复。

---

## 许可证

MIT — 随便你怎么用。

---

## 链接

- [网站和文档](https://openfang.sh)
- [快速开始指南](https://openfang.sh/docs/getting-started)
- [GitHub](https://github.com/RightNow-AI/openfang)
- [Discord](https://discord.gg/sSJqgNnq6X)
- [Twitter / X](https://x.com/openfangg)

---

## 由 RightNow 构建

<p align="center">
  <a href="https://www.rightnowai.co/">
    <img src="public/assets/rightnow-logo.webp" width="60" alt="RightNow 标志" />
  </a>
</p>

<p align="center">
  OpenFang 由 <a href="https://x.com/Akashi203"><strong>Jaber</strong></a> 构建和维护，<a href="https://www.rightnowai.co/"><strong>RightNow</strong></a> 的创始人。
</p>

<p align="center">
  <a href="https://www.rightnowai.co/">网站</a> &bull;
  <a href="https://x.com/Akashi203">Twitter / X</a> &bull;
  <a href="https://www.buymeacoffee.com/openfang" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="请我喝杯咖啡" style="height: 60px !important;width: 217px !important;" ></a>
</p>

---

<p align="center">
  <strong>使用 Rust 构建。通过 16 层保护。真正为你工作的智能代理。</strong>
</p>
