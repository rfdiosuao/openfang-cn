<p align="center"><a href="README.md">简体中文</a> · <strong>English</strong></p>

<p align="center">
  <img src="public/assets/openfang-logo.png" width="112" alt="OpenFang logo">
</p>

<h1 align="center">OpenFang Chinese Edition</h1>

<p align="center">A community-maintained Chinese localization of the open-source OpenFang Agent OS.</p>

> [!IMPORTANT]
> This is a community localization repository, not an official Chinese OpenFang distribution. Follow [RightNow-AI/openfang](https://github.com/RightNow-AI/openfang) for upstream architecture, feature development, and security advisories.

## Why This Edition?

- Chinese UI coverage for major console pages and common workflows
- Upstream architecture preserved so localization changes remain reviewable
- Complete backend, frontend, SDK, example, and deployment source
- Structured issues for missing translations, terminology, and version differences

## Localization Coverage

More than 500 user-visible strings are covered across:

- overview, chat, usage, logs, sessions, and settings
- agents, skills, hands, and channels
- workflows, scheduler, approvals, and workflow builder
- setup wizard, navigation, and common status messages

Localization changes user-visible copy only. Code identifiers remain unchanged, terminology stays consistent within each product area, and divergence from upstream is kept auditable.

## Quick Start

Requirements: Git, stable Rust from [`rust-toolchain.toml`](rust-toolchain.toml), and platform dependencies described in the [getting-started guide](docs/getting-started.md).

```bash
git clone https://github.com/rfdiosuao/openfang-cn.git
cd openfang-cn
cargo build --release
./target/release/openfang init
./target/release/openfang start
```

Windows PowerShell:

```powershell
.\target\release\openfang.exe init
.\target\release\openfang.exe start
```

Open <http://127.0.0.1:4200>. Before production deployment, read [`docs/configuration.md`](docs/configuration.md), [`SECURITY.md`](SECURITY.md), and the [production checklist](docs/production-checklist.md).

## Documentation

- [Getting Started](docs/getting-started.md)
- [Configuration](docs/configuration.md)
- [Architecture](docs/architecture.md)
- [Skill Development](docs/skill-development.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Changelog](CHANGELOG.md)

## Contributing

Use the translation issue template for missing or inconsistent copy and the bug template for reproducible defects. Follow [`CONTRIBUTING.md`](CONTRIBUTING.md) and run at least:

```bash
cargo fmt --check
cargo check --workspace
```

## License

Like upstream, the project is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE). Third-party components retain their own licenses and trademarks.
