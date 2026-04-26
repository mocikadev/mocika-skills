---
name: freya-mid-desktop
description: (MUST USE) Industrial-grade Desktop App Scaffold (Rust + Freya + MID-UI). Use this skill whenever the user requests a "desktop app", "GUI client", "cross-platform tool", or "native utility". It provides a strict SDLC pipeline from requirements gathering, UI/UX wireframing, async architecture, to CI/CD releases. Handles both Greenfield (new) and Brownfield (refactor) projects.
---

# Freya MID-UI Desktop App Scaffold

本技能定义了一套工业级桌面软件开发生命周期 (SDLC) 与极简工业风设计系统 (MID-UI)。
作为 AI 代理，当你被要求开发一款基于此规范的桌面应用时，**绝对禁止直接开始编写代码**。你必须严格遵循以下五个阶段，并在特定的节点停下来等待用户确认（卡点）。

## 核心参考资料 (Core References - MANDATORY READ)
在执行对应阶段时，你**必须**读取对应的详细标准文档：
1. **开发流程与话术模板**: `@path references/01-sdlc-workflow.md`
2. **MID-UI 设计系统（Design Tokens + 组件规格 + Do/Don't）**: `@path references/02-mid-ui-design.md` ← **Phase 2 唯一视觉标准，所有颜色/尺寸/间距必须从此文档查值，禁止自行发明**
3. **Rust + Freya 架构与工程规范**: `@path references/03-architecture.md`
4. **README 文档标准与多语言规范**: `@path references/04-documentation.md`
5. **项目开源双许可模板**: `@path assets/LICENSE-MIT`, `@path assets/LICENSE-APACHE`
6. **CI/CD 发布脚本模板**: `@path assets/release.yml`
7. **参考模板工程**: `@path template/` (包含标准入口、Activity Bar、Update 模块、状态提升示例；`template/src/theme.rs` 是 ThemeTokens 的权威实现)

---

## 阶段 0: 工程状态诊断 (Phase 0: Context Triage)
**目标**: 明确是“从零开发新项目”还是“旧有工程重构/技术栈迁移”。
**行为**: 
1. 询问用户：这是一个**全新项目 (Greenfield)** 还是**旧有工程重构 (Brownfield)**？
2. **如果是旧工程重构**：
   - 必须先使用 `explore` / `read` 工具完整梳理旧工程的目录、业务逻辑和状态管理。
   - 识别旧技术栈（如 Tauri, Iced, Electron 或纯 CLI）。
   - 核心原则：剥离并保留原有的纯业务逻辑 (Core Logic)，但准备好彻底替换旧的 UI 框架代码、构建脚本和相关文档。

## 阶段 1: 需求澄清 (Phase 1: Requirements - The PM Phase)
**目标**: 完全理解业务核心、边界和系统级依赖。
**行为**: 
1. 读取 `01-sdlc-workflow.md` 中的 Phase 1 提问大纲。
2. 扮演资深产品经理 (PM)，与用户进行多轮对话，澄清：核心输入输出、I/O 权限、常驻托盘需求、持久化配置项、国际化需求。
3. **如果是重构**：明确技术栈迁移范围（如：废弃前端 JS/TS，纯 Rust 化），生成旧业务逻辑的迁移清单。
4. 生成标准的 `docs/requirements.md`。
**[强制卡点]**: 等待用户回复“需求确认无误”。

## 阶段 2: UI 架构与原型设计 (Phase 2: UI/UX Design - The Designer Phase)
**目标**: 确定软件的视觉呈现、空间拓扑和交互逻辑。**严禁讨论 Rust 代码实现。**
**行为**:
1. 读取 `02-mid-ui-design.md`，完整理解以下内容后再开始设计：
   - 全部 Color Token（dark/light 两套，精确到 Rust 元组值）
   - Typography 体系（7 档字号，哪个角色用哪档）
   - Spacing 体系（4pt grid，合法值列表）
   - 每个组件的精确规格（Activity Bar 尺寸、Nav item 高度、Drop Zone 三态）
   - Do / Don't 规则（禁止裸色值、禁止非 4pt 间距、禁止页面布局层绝对定位等）
2. 扮演资深 UI 设计师。根据业务需求，使用 ASCII 字符画绘制出 `Main Stage` 的界面布局（输入框在哪、列表在哪）。
3. 明确各个组件对应的 Design Tokens（从 `02-mid-ui-design.md` 精确引用，不得自行发明数值）。
4. 生成 `docs/ui-design.md`。
**[强制卡点]**: 向用户展示 ASCII 草图和视觉映射，等待用户回复“UI 设计通过”。

## 阶段 3: 系统架构与依赖规划 (Phase 3: System Architecture - The Architect Phase)
**目标**: 规划目录结构、数据流和依赖选型。
**行为**:
1. 读取 `03-architecture.md`，理解状态提升 (State Hoisting) 和异步隔离 (Tokio) 规范。
2. 扮演资深架构师。**如果是重构项目**，必须规划“技术栈替换方案”：如何安全移除旧框架依赖（如 `tauri-build`），将旧的纯 Rust 逻辑解耦并放入新架构的 `src/core/` 目录。
3. 规划跨路由不丢失的**全局状态树** (Context)、局部渲染状态 (Signal)。决定需要引入的额外 Crate（如 `serde`, `directories`, `tokio`）。拒绝无用依赖。
4. 生成 `docs/design.md` 与 `AGENTS.md` (记录本项目专属的架构死线)。
**[强制卡点]**: 简述架构、状态流和选型，等待用户回复“架构通过”。

## 阶段 4: 编码实现 (Phase 4: Development - The Engineer Phase)
**目标**: 像素级还原 UI，实现稳定高效的业务逻辑。
**行为**:
1. 扮演全栈工程师。如果是新项目执行 `cargo new`。**如果是重构项目**：先大刀阔斧清理旧框架冗余代码（如 `src-tauri`、前端 `package.json`），再修改 `Cargo.toml` 引入 Freya 体系。
2. 强烈建议调用系统内置的 `rust-skills` (内存/错误处理) 和 `freya` (UI 生命周期) 技能协助编码。
3. **初始化骨架**: 必须参考或直接拷贝 `@path template/` 下的标准代码结构。先搭建外壳 (路由、主题 Context、Activity Bar 以及 Update Checker 后台任务)，验证通过后，再接入/编写 `core/` 业务逻辑，最后组装 Main Stage。
4. **实现强制验收项（新增）**:
   - `Theme` 必须是真实切换（Dark/Light/Auto）并驱动全局 token，不允许仅文字占位。
   - `Accent Color` 必须可见生效（至少作用于 active 指示器、主按钮、Drop Zone hover）。
   - `Activity Bar` 必须采用“顶部 Home、底部 Settings/About”分区，未激活图标默认透明背景，激活态使用指示器。
    - `About` 必须完整实现：Logo(56x56) + Version(Beta) + Check Update + 三链接区，居中布局。
   - `Drop Zone` 必须实现 Idle/Hover/Active 三态，不允许单态静态框。
5. 严格遵守 `03-architecture.md` 中的多线程和异步隔离规则。

## 阶段 5: 测试与发布 (Phase 5: Testing & Release - The DevOps Phase)
**目标**: 确保交互动效生效，输出全平台打包脚本和标准多语言文档，并完成远程代码托管部署。
**行为**:
1. 扮演 DevOps 工程师。校验深/浅色模式、跨路由状态是否保持、点击的物理反馈动效是否完美实现。
2. 将 `@path assets/LICENSE-MIT` 和 `@path assets/LICENSE-APACHE` 复制到项目根目录。
3. **工程化脚本**: 将 `@path assets/Makefile` 复制到项目根目录。此 Makefile 封装了全平台（Linux, macOS, Windows）的交叉编译指令和制品命名规范。
4. **Logo 生成**: 在项目根目录创建 `assets/` 文件夹。强烈建议使用 `SVG Logo Designer` 技能为应用生成一个简约的 `assets/logo.png` 或 `assets/logo.svg`，作为应用图标和 README 展示。
5. 读取 `@path references/04-documentation.md`，**如果是重构项目**，强制重写原有的 `README.md` 和架构文档，以反映全新的 Rust+Freya 技术栈。如果是新项目，则生成全新的带居中 Logo 的英文 `README.md`，并在 `docs/` 下生成中文 `README_zh.md`。
6. 清理旧的 CI/CD 流程（如 Tauri Action），将 `@path assets/release.yml` 复制到 `.github/workflows/release.yml`，开启跨端打包流。
7. **远程部署与版本发布 (GitHub Publishing & Release)**:
   - **判断场景**: 检查当前目录是否已关联远程仓库 (`git remote -v`)。
   - **场景 A (首次发布 - 全新项目)**:
     - 询问用户：**目标归属 (个人账号或组织名)**、**仓库名称**、**可见性 (公开 Public 还是私有 Private)** 以及**一句话简介 (Description)** 和 **核心标签 (Topics, 如 rust, freya, desktop)**。
     - 执行 `git init`、`git add .` 和 `git commit -m "Initial commit: Freya MID-UI Scaffold"`。
     - 格式化 Description：强制将简介格式化为**中英双语格式**（例如：`一款极简的 xlog 解密工具 · A minimalist xlog decryption utility`）。
     - 调用 `gh repo create <owner>/<repo> --<visibility> --source=. --remote=origin --push --description "<中英双语简介>"` 将代码正式推送到远程。
     - **完善 About 区**: 调用 `gh repo edit` 为仓库增加专业配置，例如：将 Homepage 指向 Latest Release (`gh repo edit --homepage "https://github.com/<owner>/<repo>/releases/latest"`)，并添加核心标签 (`gh repo edit --add-topic "rust,freya,mid-ui"` 等)。
   - **场景 B (版本迭代 - 已有旧工程/后续更新)**:
     - 询问用户：**本次发布的版本号 (如 v1.1.0)** 及 **简要更新日志 (Release Notes)**。
     - 自动修改 `Cargo.toml` 中的 `version` 字段以匹配新版本号。
     - 执行 `git add .` 和 `git commit -m "chore: release <版本号>"`。
     - 触发 CI/CD：打标签 `git tag <版本号>`，随后执行 `git push origin main --tags`，通过 `.github/workflows/release.yml` 自动触发全平台云端打包。
     - (可选) 使用 `gh release create <版本号> --title "<版本号>" --notes "..."` 创建正式的 GitHub Release 页面。
**[强制卡点]**: 在执行推送到远端 (新建仓库或 Push Tag) 之前，向用户核对发布信息（仓库配置或版本号），等待用户明确回复“确认发布”。
