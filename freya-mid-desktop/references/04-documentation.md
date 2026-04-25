# Phase 5: Documentation & Release Standards

本文档定义了 AI 在 **Phase 5 (测试与发布)** 阶段生成项目说明文档、多语言入口以及开源协议时必须遵循的标准结构与排版规范。

---

## 1. README 与多语言结构 (Principles & Structure)

为保持项目根目录的绝对清爽，多语言版本必须归档至 `docs/` 目录。

1. **主文档 (`/README.md`)**: 默认采用英文 (English)，面向全球开源社区。必须在顶部提供指向其他语言版本的清晰链接。
2. **本地化文档 (`/docs/README_zh.md`)**: 中文版及其他语言翻译，存放于 `docs/` 目录下。
3. **开源协议 (`/LICENSE-MIT`, `/LICENSE-APACHE`)**: 根目录必须包含从脚手架 assets 中复制的 MIT 与 Apache-2.0 双协议证书。

---

## 2. 英文主入口标准模板 (/README.md)

必须使用 `<div align="center">` 将 Logo、应用名称、简介和 Badges 居中对齐，并在下方使用 `---` 分割条。

```markdown
<div align="center">
  <img src="assets/logo.png" alt="[App Name] Logo" width="128" />
  
  <h1>[App Name]</h1>

  <p><em>A minimalist, industrial-grade desktop utility for [Core Function]. Built with Rust & Freya.</em></p>

  <p>
    <a href="https://github.com/[user]/[repo]/actions/workflows/release.yml">
      <img src="https://github.com/[user]/[repo]/actions/workflows/release.yml/badge.svg" alt="CI">
    </a>
    <a href="https://github.com/[user]/[repo]/releases">
      <img src="https://img.shields.io/github/v/release/[user]/[repo]" alt="Release">
    </a>
    <a href="https://github.com/[user]/[repo]/releases">
      <img src="https://img.shields.io/github/downloads/[user]/[repo]/total" alt="Downloads">
    </a>
    <a href="./LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>
    <a href="./LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
  </p>

  <p>
    <a href="./docs/README_zh.md"><b>简体中文</b></a> | <b>English</b>
  </p>
</div>

---

[ 预留演示截图/GIF 占位符: `![Demo](./docs/assets/demo.gif)` ]

## ⚡ Features
- **Lightning Fast**: Powered by Rust, memory-safe and deeply optimized.
- **MID-UI Design**: Minimalist Industrial Desktop UI with a 4pt grid system and mechanical interactions.
- **Adaptive Theme**: Native support for Dark and Light modes.
- **Cross-Platform**: Compiles to standalone binaries for macOS, Linux, and Windows.
- [ 核心业务功能 1, e.g., Drag & drop bulk processing ]

## 🚀 Installation

Download the latest pre-compiled binaries from the [Releases](https://github.com/[user]/[repo]/releases) page.

- **macOS**: Download `.dmg` or `.tar.gz`.
- **Windows**: Download `.exe`.
- **Linux**: Download AppImage or binary.

## 🛠️ Build from Source

Ensure you have [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/[user]/[repo].git
cd [repo]

# For Linux users, install required dependencies first:
# sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

cargo build --release
```

## 🏗️ Architecture & Design
This application is built upon the **freya-mid-desktop** scaffold, ensuring:
- **Async Isolation**: Heavy computations run on a dedicated Tokio thread pool to prevent UI blocking.
- **State Hoisting**: Cross-route states are preserved in a global Context.

## 📄 License
This project is dual-licensed under either the [MIT License](./LICENSE-MIT) or the [Apache License, Version 2.0](./LICENSE-APACHE), at your option.
```

---

## 3. 中文本地化模板 (/docs/README_zh.md)

本地化文档必须同样使用 `<div align="center">` 居中对齐头部元素，并添加 `---` 分割条。注意 Logo 和 License 的相对路径。

```markdown
<div align="center">
  <img src="../assets/logo.png" alt="[应用名称] Logo" width="128" />
  
  <h1>[应用名称]</h1>

  <p><em>一款基于极简工业风 (MID-UI) 设计的跨平台桌面工具，专为 [核心功能] 打造。</em></p>

  <p>
    <a href="https://github.com/[user]/[repo]/actions/workflows/release.yml">
      <img src="https://github.com/[user]/[repo]/actions/workflows/release.yml/badge.svg" alt="CI">
    </a>
    <a href="https://github.com/[user]/[repo]/releases">
      <img src="https://img.shields.io/github/v/release/[user]/[repo]" alt="Release">
    </a>
    <a href="https://github.com/[user]/[repo]/releases">
      <img src="https://img.shields.io/github/downloads/[user]/[repo]/total" alt="Downloads">
    </a>
    <a href="../LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>
    <a href="../LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
  </p>

  <p>
    <b>简体中文</b> | <a href="../README.md"><b>English</b></a>
  </p>
</div>

---

[ 预留演示截图/GIF 占位符: `![演示图](../assets/demo.gif)` ]

## ⚡ 核心特性
- **极致性能**: 纯 Rust 编写，内存安全，零 GC 负担。
- **极简工业风 (MID-UI)**: 遵循严格的 4pt 网格系统，提供深浅模式，无多余视觉噪音。
- **物理级交互**: 所有按钮与列表均提供清脆的 `scale(0.98)` 机械按压反馈。
- **全平台支持**: 提供开箱即用的 macOS, Linux, Windows 单文件二进制包。
- [ 核心业务功能 1, 如：支持文件批量拖拽解析 ]

## 🚀 安装指南

请前往 [Releases](https://github.com/[user]/[repo]/releases) 页面下载最新编译好的版本：

- **macOS**: 下载 `.dmg` 或压缩包。
- **Windows**: 下载 `.exe` 格式。
- **Linux**: 下载 AppImage 或二进制文件。

## 🛠️ 本地编译

请确保已安装最新的 [Rust](https://rustup.rs/) 工具链。

```bash
git clone https://github.com/[user]/[repo].git
cd [repo]

# Linux 用户请先安装底层依赖：
# sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

cargo run --release
```

## 🏗️ 架构说明
本应用采用标准的 **freya-mid-desktop** 工程架构：
- 耗时计算被强行隔离至 Tokio 异步后台线程池，彻底杜绝 UI 假死。
- 跨路由的核心业务数据通过顶层 Context 状态提升，切换页面不丢进度。

## 📄 开源协议
本项目采用 [MIT 协议](../LICENSE-MIT) 或 [Apache 2.0 协议](../LICENSE-APACHE) 双重授权。
```
