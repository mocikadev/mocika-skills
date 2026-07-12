# Mocika Skills

AI 代理技能库，存放与日常开发工作高度相关的专属 AI 技能 (Skills)。每个技能均自包含，可通过 `skm` 按需链接到工作流。

---

## 技能清单

### [arch-guard](./arch-guard/)

在代码变更前后守护职责边界、模块拆分和依赖方向。

- **解决什么问题**：防止 AI 把新职责持续堆入巨型文件或模块，也防止机械分层和过度设计
- **能力**：L0–L3 变更分级；函数到服务的多层级边界判断；package、crate、workspace 拆分评估；实现后架构验收
- **产物**：架构摘要、候选方案与取舍、边界契约、验证结果和后续治理建议
- **触发示例**：`使用 arch-guard 实现这个功能` / `检查这里是否应该拆 module 或 crate`

---

### [project-ai-system](./project-ai-system/)

为项目生成并维护项目级 AI 工作系统。

> 原 `agents-map` 已升级并改名为 `project-ai-system`。

- **解决什么问题**：新项目需求不清、缺少 AI 执行规范、AGENTS.md 越写越长、文档散落混乱
- **五种模式**：启动（澄清 spec 和执行协议）、生成（建立入口导航）、修订（压缩重构）、审计（只看不改）、整理（归类现有文档）
- **产物**：项目 spec + AI 执行协议 + 入口文件草稿 + docs/ 骨架 + project skill 建议清单
- **触发示例**：`帮我先把新项目需求和 AI 执行规则定清楚` / `检查一下我的 AGENTS.md 是否规范`

---

### [freya-mid-desktop](./freya-mid-desktop/)

工业级 Rust + Freya 跨平台桌面应用脚手架与开发规范。

- **解决什么问题**：Freya GUI 框架缺乏系统性的项目结构规范、组件模式和开发流程约束
- **能力**：从需求收集、UI 设计、异步架构到 CI/CD 发布的完整 SDLC 流水线；同时支持绿地新建和棕地重构
- **技术栈**：Rust · Freya v0.4.0-rc.19 · Builder API · 无 `rsx!` · MIT OR Apache-2.0
- **触发示例**：`帮我新建一个桌面应用` / `我要用 Freya 做一个 GUI 工具`

---

## 安装与使用

```bash
skm install mocikadev/mocika-skills:arch-guard --link-to all
skm install mocikadev/mocika-skills:project-ai-system --link-to all
skm install mocikadev/mocika-skills:freya-mid-desktop --link-to all
```

安装后，在任意 AI Agent 会话中加载对应 skill 即可使用。

## 仓库规范

约束与规范详见 [AGENTS.md](./AGENTS.md)。
