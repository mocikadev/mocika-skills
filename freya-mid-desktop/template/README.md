# Freya MID App

> 极简工业风桌面应用模板 · A minimalist industrial desktop app template

**[简体中文](#简体中文) · [English](#english)**

---

## 简体中文

### 简介

`freya-mid-app` 是一个基于 [Freya](https://freyaui.dev/) (v0.4.0-rc.19) 构建的桌面应用模板，采用工业/极简风格（类 Linear / Cursor），提供完整的路由、主题切换、强调色拾取、拖拽文件处理等开箱即用的功能骨架。

### 特性

- 🎨 **深色 / 浅色 / 跟随系统** 三档主题切换
- 🖌️ **全色域强调色拾取器** — 使用 `ColorPicker` 自由选色
- 🗂️ **freya-router 路由** — Home / Settings / About 页面切换，切换时保留状态
- 📂 **拖拽文件** — 支持文件拖入检测与路径列表展示
- ✨ **Material Ripple 点击波纹** — 所有可交互元素均有点击反馈
- 🔄 **自动更新检测** — 启动时请求 GitHub Releases，有新版本时 Settings/About 双入口提示
- 📐 **自适应布局** — 页面横向自适应 + 纵向可滚动
- 🏷️ **MIT OR Apache-2.0** 双协议授权

### 技术栈

| 依赖 | 版本 |
|---|---|
| freya | 0.4.0-rc.19 |
| freya features | `icons`, `material-design`, `router` |
| tokio | 1.x |
| reqwest | 0.11 (rustls-tls) |
| open | 5.x |

### 快速开始

```bash
# 克隆后进入模板目录
cd freya-mid-desktop/template

# 开发运行
cargo run

# 构建 Release
cargo build --release
```

### 项目结构

```
src/
├── main.rs              # 入口：launch + tokio runtime
├── app.rs               # AppState、Route、AppLayout、根组件
├── theme.rs             # ThemeTokens、ThemeMode、颜色工具函数
├── components/
│   ├── activity_bar.rs  # 侧边栏导航（Logo + 路由切换）
│   └── drop_zone.rs     # 文件拖拽区域
├── views/
│   ├── home.rs          # 主功能页（拖拽 + 文件列表）
│   ├── settings.rs      # 设置页（主题、强调色、系统信息）
│   └── about.rs         # 关于页（居中布局、链接、更新入口）
└── core/
    └── update.rs        # 后台更新检测（GitHub Releases API）
assets/
└── logo.svg             # 应用 Logo（单色 SVG，可由 tint 着色）
```

### 跨平台打包

项目已配置 `cargo-packager` 元数据，可生成 `.deb`（Linux）、`.msi`（Windows）、`.dmg`（macOS）：

```bash
# 安装打包工具
cargo install cargo-packager

# 打包（在对应平台上执行，或使用 CI 矩阵）
cargo packager --release
```

---

## English

### Overview

`freya-mid-app` is a desktop application template built with [Freya](https://freyaui.dev/) (v0.4.0-rc.19), featuring a minimalist industrial aesthetic (inspired by Linear / Cursor). It ships a complete, production-ready skeleton: routing, theming, full-spectrum accent color picking, file drag-and-drop, Ripple press effects, and auto-update detection.

### Features

- 🎨 **Dark / Light / Auto** theme switching
- 🖌️ **Full-spectrum accent color picker** — free color selection via `ColorPicker`
- 🗂️ **freya-router routing** — Home / Settings / About, state preserved across navigation
- 📂 **File drag-and-drop** — detects files dragged over the window, lists dropped paths
- ✨ **Material Ripple press effect** — all interactive elements have tactile feedback
- 🔄 **Auto-update detection** — checks GitHub Releases on startup; update prompts in Settings & About
- 📐 **Responsive layout** — horizontal adaptive + vertical scrollable on all pages
- 🏷️ **Licensed under MIT OR Apache-2.0**

### Tech Stack

| Dependency | Version |
|---|---|
| freya | 0.4.0-rc.19 |
| freya features | `icons`, `material-design`, `router` |
| tokio | 1.x |
| reqwest | 0.11 (rustls-tls) |
| open | 5.x |

### Quick Start

```bash
# Enter the template directory
cd freya-mid-desktop/template

# Run in development
cargo run

# Build release binary
cargo build --release
```

### Project Structure

```
src/
├── main.rs              # Entry: launch + tokio runtime
├── app.rs               # AppState, Route, AppLayout, root component
├── theme.rs             # ThemeTokens, ThemeMode, color utilities
├── components/
│   ├── activity_bar.rs  # Sidebar navigation (Logo + route switching)
│   └── drop_zone.rs     # File drag-and-drop zone
├── views/
│   ├── home.rs          # Main page (drop zone + file list)
│   ├── settings.rs      # Settings (theme, accent color, system info)
│   └── about.rs         # About (centered layout, links, update CTA)
└── core/
    └── update.rs        # Background update check (GitHub Releases API)
assets/
└── logo.svg             # App logo (monochrome SVG, tint-colorable)
```

### Cross-Platform Packaging

The project is pre-configured with `cargo-packager` metadata to generate `.deb` (Linux), `.msi` (Windows), and `.dmg` (macOS) installers:

```bash
# Install the packaging tool
cargo install cargo-packager

# Package (run on the target platform, or use a CI matrix)
cargo packager --release
```

---

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option.
