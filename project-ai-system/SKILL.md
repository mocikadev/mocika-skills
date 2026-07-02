---
name: project-ai-system
description: >
  Use when 需要为项目建立、修订或审计 AI 工作系统：澄清新项目 spec、
  制定执行协议、整理 AGENTS.md/CLAUDE.md/GEMINI.md、规划 docs 结构，
  或判断内容应放入口文件、docs 还是 project skill。
---

# project-ai-system

根据项目结构与约束，产出项目级 AI 工作系统：需求 spec、执行协议、地图式入口文件、docs 骨架和 project skill 清单；支持五种模式：启动、生成、修订、审计、整理。

核心原则：入口文件是地图，不是百科全书。只保留项目专属入口、导航和高优先级约束；详细规范下沉到 docs 或 project skill。

## 五种模式

| 模式 | 触发条件 | 目标 |
|------|----------|------|
| **启动** | 新项目、需求模糊、用户只描述想法 | 先问清需求，输出 spec + 执行协议 + 文档计划 |
| **生成** | 已有清晰 spec，或用户要求建立 AI 上下文 | 输出入口文件草稿 + docs/ 骨架 + skill 清单 |
| **修订** | 有入口文件，需要重构 | 压缩冗余、将细则下沉到 docs/ 或 skill |
| **审计** | 有入口文件，只检查不改写 | 输出问题清单与优先级建议 |
| **整理** | 文档散落、docs/ 结构混乱 | 扫描归类现有文档，输出迁移方案 |

---

## 第一步：按序读取项目信息

无论哪种模式，先按以下顺序读取，不得跳过或乱序：

1. 项目根目录列表（`ls`）
2. 已有 AI 入口文件：`AGENTS.md`、`CLAUDE.md`、`GEMINI.md`（读取所有存在的文件）
3. 项目依赖/清单文件（读取所有存在的，如 `package.json`、`pyproject.toml`、`go.mod`、`Cargo.toml`、`pom.xml` 等）
4. `README.md`
5. 项目 skills 目录或包（如 `.codex/skills/`、`.claude/skills/`、`.opencode/skills/`、`.agents/skills/`、`*/SKILL.md`）
6. `docs/` 目录（如存在）
7. CI 配置：`.github/workflows/` 或 `.gitlab-ci.yml`（如存在）
8. `Makefile`（如存在）

读取完毕后，归纳以下信息：
- 项目名称 / 仓库名 / 当前状态
- 技术栈
- 现有提交前检查流程（如有）
- AI 无法从代码推断的约束（特殊路径、跨文件同步要求、最低运行时/语言版本要求、共享格式等）
- 已有 spec / 需求 / 设计 / 执行协议文件
- 已有 docs/ 文件列表
- 已有 skills 列表

---

## 启动模式

用户给出新项目想法或需求不清时，先参考 `references/spec-template.md` 逐步澄清，不得直接生成入口文件。

1. 先问清：目标用户、要解决的问题、成功标准、范围边界、关键约束、验收方式
2. 若信息不足，最多一次问 3 个高价值问题；用户回答后再继续补齐 spec
3. 输出 `docs/requirements/project-spec.md` 草稿，包含目标、范围、非目标、用户场景、验收标准
4. 参考 `references/execution-protocol-template.md` 输出 `docs/ai/execution-protocol.md` 草稿
5. 再进入生成模式，生成入口文件摘要、docs 骨架和 project skill 建议

## 生成模式

参考 `references/agents-md-template.md` 填充模板，参考 `references/docs-skeleton.md` 生成骨架。若没有可引用的 spec，先回到启动模式。

1. 选择入口文件：默认 `AGENTS.md`；若项目已明确使用 `CLAUDE.md` 或 `GEMINI.md`，沿用现有入口，不强行迁移
2. 入口文件必须包含执行协议摘要，并指向 `docs/ai/execution-protocol.md`
3. 用收集到的信息填充模板，按推荐章节顺序输出，允许省略无内容章节
4. 输出按需 docs/ 目录骨架，注明哪些目录建议优先补充内容
5. 参考 `references/skill-boundary-rules.md` 输出 project skill 建议清单（名称 + 触发场景 + 放置路径 + 原因）
6. 复杂变更参考 `references/change-contract-template.md` 建议执行契约
7. 如发现跨文件同步约束，必须写入"关键约束"章节

## 修订模式

参考 `references/agents-md-template.md` 中的高价值内容清单和验收检查清单。

1. 逐章节审查现有 AI 入口文件
2. 识别以下问题内容并迁移：
   - 大段风格细则、完整枚举表格 → `docs/style/`
   - 架构背景说明、模块关系图 → `docs/design/`
   - 发布流程、分支策略 → `docs/process/`
   - 任务执行分级、执行契约、快速路径、spec/plan/TDD/验证规则 → `docs/ai/execution-protocol.md`
   - 多步骤操作手册 → 参考 `references/skill-boundary-rules.md` 判断是否拆成 project skill
3. 以下内容**不得**迁移，必须保留在入口文件：
   - 项目特有开发/协作约束（高优先级，必须高可见）
   - 执行协议摘要与链接
   - 高价值内容（见 `references/agents-md-template.md`）
4. 输出修订后的完整入口文件草稿，不只给 diff 建议

## 审计模式

参考 `references/agents-md-template.md` 和 `references/audit-report-template.md` 逐项检查，输出：
- 问题清单（每条注明：问题类型 / 具体位置 / 建议操作）
- 必须检查是否存在 spec、验收标准、执行协议、执行契约规则、快速路径、完成验证规则、收口同步规则、风险操作规则
- 优先修复顺序（缺少 spec/验收标准 > 缺少执行协议 > 过长 > 重复全局规则 > 缺少关键约束 > 缺少导航 > 空目录/过度模板化）

## 整理模式

参考 `references/classification-rules.md` 中的分类规则和迁移清单格式。

1. 扫描以下位置的所有文档文件：
   - 项目根目录下的 `.md`、`.txt`、`.rst` 等文档文件
   - 已有 `docs/` 目录（含所有子目录）
   - 其他常见文档目录（`notes/`、`wiki/`、`spec/` 等，如存在）
2. 逐文件按分类规则判断归属目录
3. 对内容混杂的单文件，建议拆分并说明拆分方式
4. 输出迁移清单供用户确认，不自动执行文件移动

---

## 全局规则不重复原则

如果当前环境存在全局代理规则（如 `~/.codex/AGENTS.md`、`~/.claude/CLAUDE.md`、`~/.gemini/GEMINI.md`、`~/.config/opencode/AGENTS.md`、`~/.agents/AGENTS.md`），则：

- 语言约束（中文回复要求）→ **不写**
- 提交格式（feat/fix/docs...）→ 仅写项目特有的例外，通用格式 **不写**
- samsara 自我进化协议 → **不写**
- 高风险操作通用原则 → **不写**
- 通用 TDD、设计先行、代码审查等代理工作流 → 仅写项目特有的强制例外

项目入口文件只写**该项目特有的、全局规则里没有的内容**；详细执行协议写入 `docs/ai/execution-protocol.md`。

---

## 不要做

- 不要把整份规范写回 AGENTS.md
- 不要生成与项目无关的泛化内容
- 不要编造未在代码或文档中出现过的约束
- 不要在没有依据时强行拆分过度
- 不要只给修改建议——修订模式必须输出可直接使用的完整草稿
- 不要为小项目强制创建全量 docs 空目录
- 不要在需求不清时跳过 spec 直接制定实现方案
