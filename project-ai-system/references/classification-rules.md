# 文档内容分类规则

整理模式使用。逐文件判断内容归属，按以下规则分类：

## 分类规则

| 内容特征 | 归属目录 |
|----------|----------|
| 项目 spec、目标、范围、非目标、功能描述、用户故事、验收条件、产品需求 | `docs/requirements/` |
| 架构设计、模块划分、数据流、API 设计、系统交互 | `docs/design/` |
| 界面稿、交互说明、组件规范、视觉规范、原型 | `docs/ui/` |
| 实现细节、开发笔记、技术选型记录、关键决策（ADR） | `docs/development/` |
| 已知问题、修复记录、版本变更、changelog | `docs/bugfix/` |
| 提交规范、发布流程、分支策略、变更管理 | `docs/process/` |
| 命名约定、注释规范、代码格式、语言要求 | `docs/style/` |
| 测试策略、覆盖率要求、验收标准、QA 流程 | `docs/quality/` |
| 部署流程、环境配置、运维约束、基础设施 | `docs/ops/` |
| AI 执行协议、变更执行契约、审计报告、代理导航补充、提示词约定、模型/工具使用说明、project skill 设计记录 | `docs/ai/` |

## 混杂文件处理

当一个文件包含多类内容时（如同时有架构说明和 UI 规范），应建议拆分：

1. 识别文件中各段落的内容类型
2. 按内容类型划分拆分点
3. 输出建议：拆成哪几个文件、分别放到哪个目录

## 迁移清单格式

输出迁移方案时使用以下表格格式：

| 当前路径 | 建议移至 | 原因 | 是否需要拆分 |
|----------|----------|------|--------------|
| `README-dev.md` | `docs/development/` | 开发说明 | 否 |
| `DESIGN.md` | `docs/design/` + `docs/ui/` | 混合架构和 UI 内容 | 是 |
| `AI-NOTES.md` | `docs/ai/` | AI 代理使用说明 | 否 |
| `SPEC.md` | `docs/requirements/project-spec.md` | 项目 spec | 否 |
| `EXECUTION.md` | `docs/ai/execution-protocol.md` | AI 执行协议 | 否 |
| `CONTRACT.md` | `docs/ai/changes/<change-name>/execution-contract.md` | 变更执行契约 | 否 |
| `AI-AUDIT.md` | `docs/ai/audit-report.md` | AI 工作系统审计报告 | 否 |

> ⚠️ 只输出迁移方案，不自动执行文件移动，待用户确认后再操作。
