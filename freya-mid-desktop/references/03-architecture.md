# Phase 3: Rust + Freya System Architecture

本文档定义了 AI 在 **Phase 3 (系统架构与依赖规划)** 和 **Phase 4 (编码实现)** 中必须遵守的目录结构、状态流和底层系统约束。同时规定了如何生成项目级的 `AGENTS.md`。

---

## 1. 工业级目录结构 (Directory Skeleton)

所有基于 MID-UI Scaffold 开发的应用，必须严格采用以下分层架构。严禁将业务逻辑和 UI 代码杂糅在一个文件内。

```text
project_root/
├── .github/workflows/   # CI/CD 打包流
├── docs/                # 需求(requirements.md)、设计(ui-design.md, design.md)
├── src/
│   ├── components/      # UI 纯展示组件 (遵守 MID-UI)
│   │   ├── activity_bar.rs
│   │   ├── color_picker_panel.rs  # 内嵌 HSV 渐变拾色器，无 swatch 包装层
│   │   ├── drop_zone.rs
│   │   └── mod.rs
│   ├── core/            # 纯 Rust 业务逻辑，与 Freya 无关 (纯异步)
│   │   ├── update.rs    # GitHub Release 更新检测
│   │   └── mod.rs
│   ├── views/           # 页面级路由组件 (Home, Settings, About)
│   │   ├── home.rs
│   │   ├── settings.rs
│   │   ├── about.rs
│   │   └── mod.rs
│   ├── app.rs           # 路由定义、AppState、AppLayout（Context 注入）
│   ├── theme.rs         # MID-UI 规范的色板枚举与 ThemeTokens 定义
│   └── main.rs          # 程序入口，调用 launch(app)
├── assets/
│   └── logo.svg         # 纯 stroke path，无 mask/filter
├── AGENTS.md            # 【重要】本项目专属的 AI 代理开发宪法
├── README.md            # 面向全球的开源介绍
└── Cargo.toml           # 依赖清单
```

---

## 2. Freya v0.4 组件规范（Component API Constraints）

**所有 UI 组件必须遵守以下规则，无一例外：**

### 2.1 组件声明方式（MANDATORY）

```rust
// ✅ 正确：使用 impl Component
#[derive(PartialEq)]
pub struct HomeView;

impl Component for HomeView {
    fn render(&self) -> impl IntoElement {
        // use_state / use_consume 必须在此函数顶层
        let state = use_consume::<State<AppState>>();
        rect()...
    }
}

// ❌ 错误：不允许使用 #[component] 宏（Freya v0.4 rc.19 Builder API 不支持）
#[component]
fn HomeView() -> Element { ... }
```

### 2.2 Hook 调用限制

- `use_state`、`use_consume`、`use_effect` 等 hook **只能在 `render()` 方法体的顶层调用**
- **禁止**在条件分支（`if/else`）、循环（`for`）、或由 `render()` 调用的普通函数中调用 hook
- `State<T>` 实现了 `Copy`：多个 closure 需要共享同一个 state 时，直接在各 closure 中分别捕获即可，无需 `.clone()`

### 2.3 元素构造规范

- 使用 Builder API：`rect()`, `label()`, `svg()`, `ScrollView::new()` 等
- **禁止**使用 `rsx!` 宏或 JSX 语法
- SVG 文件必须用 `svg(include_bytes!("path/to/file.svg"))` 加载，**禁止**用 `ImageViewer`（SVG 不支持）
- SVG 颜色必须显式指定 `.color((r, g, b))`，否则在彩色背景上不可见

### 2.4 布局规则

- 绝对定位（`Position::new_absolute()`）**只允许**用于通知红点这类徽标叠加，其他布局全部使用流式布局
- 指示器、分隔线等等高元素用 `Size::fill()` 而非固定像素，确保随容器高度自动拉伸

---

**严禁在页面路由切换 (如 Home 切换到 Settings 再切回) 时丢失业务状态。**

### 2.1 状态提升铁律 (State Hoisting)
任何跨路由需要保持的业务状态，**绝不能定义在 `views/home.rs` 内部的局部 `use_signal` 中**。必须将其“提升”到全局作用域。

### 2.2 全局 Context 注入 (`use_context_provider`)
必须在 `main.rs` 的顶级 `app` 组件中注入：
1. **业务状态池 (Business Store)**：存储任务队列、进度。切换页面时状态在后台持续更新，切回页面直接读取最新数据。
2. **应用配置 (AppConfig)**：存储从 `~/.config/` 读取的用户偏好。在 Settings 页修改后，全局响应生效。

### 2.3 局部 Signal (`use_signal`)
**仅允许**用于单个页面内即看即焚的渲染状态（如：按钮的 `is_pressed` 动效状态、输入框焦点）。

---

## 3. 阻塞操作隔离铁律 (Async Isolation)

**绝对不允许在主线程中执行任何耗时阻塞操作。**
任何耗时超过 16ms 的计算或 I/O（如网络请求、大文件读写、解密压缩），必须遵循以下异步架构：

使用 Freya 内置的 `spawn` 将任务扔到后台。通过 `use_consume::<State<AppState>>()` 获取全局状态，在 closure 中直接调用 `.write()` 更新。

```rust
// 示例规范：异步执行并安全跨线程更新全局状态
// ✅ 正确：使用 Freya 的 spawn（不是 tokio::spawn），以及 State<T> API
let mut app_state = use_consume::<State<AppState>>();

let on_pick = move |_| {
    spawn(async move {
        if let Some(files) = rfd::AsyncFileDialog::new().pick_files().await {
            for f in files {
                app_state.write().dropped_files.push(
                    f.path().to_string_lossy().to_string()
                );
            }
        }
    });
};

// ❌ 错误：不要使用 tokio::spawn（无法跨线程安全写入 State<T>）
// let mut store = use_context::<Signal<GlobalTaskStore>>();  // ← 旧 API，已废弃
// tokio::spawn(async move { store.write().update(); });      // ← 错误用法
```

---

## 4. 依赖决议哲学 (Crate Selection)

脚手架追求极致轻量。除 `freya` 外，其他系统级功能必须**按需引入 (Opt-in)**：
*   **配置保存**: `directories`, `serde`, `serde_json/toml`.
*   **更新检查/网络**: `tokio` (full), `reqwest`, `open`.
*   **文件对话框**: `rfd`（提供跨平台原生文件选择器，解决 Freya 拖拽事件时序 bug 的 click-to-browse 降级方案）.
*   **系统托盘**: `tray-icon`.

---

## 5. 项目级 AGENTS.md 生成模板 (Project Constitution)

**【极其重要】**
为了防止未来的 AI 代理在维护该项目时脱离 MID-UI 规范或破坏异步架构，AI 必须在 **Phase 3** 的最后，在项目根目录生成一份强有力的 `AGENTS.md`。

该文档必须把脚手架的核心约束“继承”过去，**模板如下，直接复制并填入业务名称**：

```markdown
# [应用名称] - AI 开发宪法 (Agent Constitution)

欢迎维护本项目。在进行任何代码修改前，请严格遵守以下铁律。违反以下规则将导致代码被拒绝。

## 1. 进度与任务追踪 (Task Tracking)
- 动手前必须查阅 `docs/requirements.md` 和 `docs/design.md`，确认业务逻辑与 UI 拓扑。
- 复杂任务必须使用 Todo 列表 (`todowrite` tool) 拆解步骤。

## 2. 核心架构纪律 (Architecture Deadlines)
- **绝对异步 (Async Isolation)**: 任何耗时 I/O、文件解析或网络请求，**必须**使用 `tokio::spawn` 扔到后台。绝不允许阻塞 Freya UI 线程！
- **状态不丢失 (State Hoisting)**: 跨页面的业务数据和配置，必须通过 `use_context_provider` 注入在根节点。禁止在 `views/xxx.rs` 的局部 `use_signal` 中保存重要持久化数据。

## 3. MID-UI 视觉令牌绝对不可侵犯 (Design Tokens)
- **网格**: 只允许使用 4pt 间距 (`4, 8, 12, 16, 24, 32`)。
- **排版**: 分组标题必须是 `font-size: "11"`, `bold`, `text-transform: "uppercase"`, 色彩 `#8E8E8E`。
- **边框**: 抛弃阴影，使用 `1px` 极细微边框 (`rgba(255,255,255,0.08)` 或 `rgba(0,0,0,0.08)`)。
- **机械动效**: 任何新增的可点击按钮，必须带有 `scale(0.98)` 的 150ms 按压物理回弹动效。

## 4. 依赖约束 (Dependencies)
- 未经用户允许，严禁擅自向 `Cargo.toml` 添加新的重型依赖。
- 当前已批准的核心依赖：Freya, Tokio, [根据实际情况列出...]
```

## 6. 应用自动更新机制 (Update Checker)

如果需求明确包含“检查更新”功能，AI 必须实现一套完整的工业级检测流。
该机制不使用重量级的自动热更新（避免增加权限和签名复杂度），而是采用**“后台检测 + 浏览器下载”**的轻量级方案。

### 6.1 状态定义 (Update State)
在全局状态池中定义 `UpdateInfo` 结构体：
```rust
pub struct UpdateInfo {
    pub has_update: bool,
    pub latest_version: Option<String>,
    pub release_url: Option<String>,
    pub update_level: Option<String>, // "major", "minor", "patch"
}
```

### 6.2 后台检测逻辑 (Background Check)
在 `main.rs` 启动时（`app` 组件挂载时），立刻触发一次异步检查。需要支持版本比对和缓存，以防止频繁请求被 GitHub 限制。
*   **API 目标**: 调用 `https://api.github.com/repos/[owner]/[repo]/releases/latest`。
*   **Header 规范**: 必须带上 `User-Agent: [App-Name]` 和 `Accept: application/vnd.github.v3+json`，否则会被 GitHub API 拒绝（403 Forbidden）。
*   **版本比对**: 使用 `env!("CARGO_PKG_VERSION")` 获取当前版本。剥离 `v` 前缀后，按照 `major.minor.patch` 分段进行严格比对。如果发现 `major` 版本更新，标记 `update_level = "major"`，以此类推。

```rust
// 伪代码参考
let (cur_maj, cur_min, cur_pat) = parse_semver(env!("CARGO_PKG_VERSION"));
let (lat_maj, lat_min, lat_pat) = parse_semver(latest_version);

let update_level = if lat_maj > cur_maj { "major" }
else if lat_min > cur_min { "minor" }
else if lat_pat > cur_pat { "patch" }
else { return no_update(); };
```

### 6.3 UI 展示与交互 (UI & Interaction)
根据 `update_level` 提供不同级别的更新提示：
1. **静默红点**: 发现 `minor` 或 `patch` 更新时，在左侧 Activity Bar 的 `Settings` 齿轮图标右上角，绘制一个红色的极小圆点 (`width: 6, height: 6, background: #E06C75`)。如果用户在设置页点击了“忽略 (Dismiss)”，则清除红点状态。
2. **下载 (Action)**: 在 Settings 或 About 页面的版本号下方渲染 `Download v[latest_version]` 按钮。点击时使用 `open` Crate（或系统原生命令）在浏览器中打开 `release_url`。
3. **强制弹窗 (可选)**: 如果是 `major` 更新，才考虑在主页面顶部增加不侵入的 Banner 提示。

### 7. 桌面应用打包规范 (Desktop App Packaging)
桌面应用不能仅仅输出裸二进制文件 (Raw Binary)，必须打包为标准的系统安装包。
本项目采用 `cargo-packager` 作为打包工具。AI 必须在 `Cargo.toml` 底部添加打包配置：

```toml
[package.metadata.packager]
product-name = "App Name"
identifier = "com.mocika.app"
category = "Utility"
authors = ["Your Name"]
publisher = "Your Name"
out-dir = "dist"
icon = ["assets/logo.png"]

[package.metadata.packager.deb]
depends = ["libwebkit2gtk-4.0-37", "libgtk-3-0"]
```
