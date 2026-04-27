# Mocika Skills

私有 AI 代理技能库，存放与日常开发工作高度相关的专属 AI 技能 (Skills)。每个技能均自包含，可通过 `skm` 按需链接到工作流。

> ⚠️ 本仓库为**私有仓库**，严禁公开。

---

## 技能清单

### [agents-map](./agents-map/)

为项目生成并维护"地图式" AI 导航体系。

- **解决什么问题**：新项目没有 AI 上下文、AGENTS.md 越写越长、文档散落混乱
- **四种模式**：生成（新项目初始化）、修订（压缩重构）、审计（只看不改）、整理（归类现有文档）
- **产物**：AGENTS.md 草稿 + docs/ 骨架（9 目录）+ project skill 建议清单
- **触发示例**：`帮我初始化项目的 AGENTS.md` / `检查一下我的 AGENTS.md 是否规范`

---

### [freya-mid-desktop](./freya-mid-desktop/)

工业级 Rust + Freya 跨平台桌面应用脚手架与开发规范。

- **解决什么问题**：Freya GUI 框架缺乏系统性的项目结构规范、组件模式和开发流程约束
- **能力**：从需求收集、UI 设计、异步架构到 CI/CD 发布的完整 SDLC 流水线；同时支持绿地新建和棕地重构
- **技术栈**：Rust · Freya v0.4.0-rc.19 · Builder API · 无 `rsx!` · MIT OR Apache-2.0
- **触发示例**：`帮我新建一个桌面应用` / `我要用 Freya 做一个 GUI 工具`

---

## 安装与使用

> 本仓库为私有仓库，安装前请确保本机 Git 已配置 GitHub 访问权限。

```bash
# 安装单个技能到所有 Agent
skm install mocikadev/mocika-skills:agents-map --link-to all
skm install mocikadev/mocika-skills:freya-mid-desktop --link-to all

# 安装后补链接到指定 Agent（如只需 opencode）
skm link agents-map opencode
skm link freya-mid-desktop opencode
```

安装后，在任意 AI Agent 会话中加载对应 skill 即可使用。

## 仓库规范

约束与规范详见 [AGENTS.md](./AGENTS.md)。
