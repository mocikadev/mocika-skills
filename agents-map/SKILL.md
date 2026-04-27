---
name: agents-map
description: >
  Scaffold, revise, audit, or organize a project's AI navigation system (AGENTS.md + docs/ skeleton +
  project skills). Use when initializing AI context for a new project, restructuring an oversized or
  chaotic AGENTS.md, checking whether an existing AGENTS.md follows map-style principles, deciding
  what belongs in AGENTS.md vs docs/ vs a project skill, or organizing scattered docs into a proper
  structure. Triggers: "generate AGENTS.md", "init project AI context", "AGENTS.md is too long",
  "audit AGENTS.md", "set up docs skeleton", "organize docs", "clean up documentation".
compatibility: opencode
---

# agents-map

根据项目结构与约束，产出"地图式" AGENTS.md、docs/ 骨架和 project skill 清单；支持四种模式：生成、修订、审计、整理。

## 四种模式

| 模式 | 触发条件 | 目标 |
|------|----------|------|
| **生成** | 无 AGENTS.md，或用户要求从头建 | 输出完整 AGENTS.md 草稿 + docs/ 骨架 + skill 清单 |
| **修订** | 有 AGENTS.md，需要重构 | 压缩冗余、将细则下沉到 docs/ 或 skill |
| **审计** | 有 AGENTS.md，只检查不改写 | 输出问题清单与优先级建议 |
| **整理** | 文档散落、docs/ 结构混乱 | 扫描归类现有文档，输出迁移方案 |

---

## 第一步：按序读取项目信息

无论哪种模式，先按以下顺序读取，不得跳过或乱序：

1. 项目根目录列表（`ls`）
2. 已有 `AGENTS.md`（如存在）
3. 项目依赖/清单文件（读取所有存在的，如 `package.json`、`pyproject.toml`、`go.mod`、`Cargo.toml`、`pom.xml` 等）
4. `README.md`
5. `.opencode/skills/` 目录（如存在）
6. `docs/` 目录（如存在）
7. CI 配置：`.github/workflows/` 或 `.gitlab-ci.yml`（如存在）
8. `Makefile`（如存在）

读取完毕后，归纳以下信息：
- 项目名称 / 仓库名 / 当前状态
- 技术栈
- 现有提交前检查流程（如有）
- AI 无法从代码推断的约束（特殊路径、跨文件同步要求、最低运行时/语言版本要求、共享格式等）
- 已有 docs/ 文件列表
- 已有 skills 列表

---

## 生成模式

参考 `references/agents-md-template.md` 填充模板，参考 `references/docs-skeleton.md` 生成骨架。

1. 用收集到的信息填充 AGENTS.md 模板，按固定章节顺序输出
2. 输出 docs/ 目录骨架，注明哪些目录建议优先补充内容
3. 输出 project skill 建议清单（名称 + 一句话说明 + 放置路径）
4. 如发现跨文件同步约束，必须写入"关键约束"章节

## 修订模式

参考 `references/agents-md-template.md` 中的高价值内容清单和验收检查清单。

1. 逐章节审查现有 AGENTS.md
2. 识别以下问题内容并迁移：
   - 大段风格细则、完整枚举表格 → `docs/style/`
   - 架构背景说明、模块关系图 → `docs/design/`
   - 发布流程、分支策略 → `docs/process/`
   - 多步骤操作手册 → project skill
3. 以下内容**不得**迁移，必须保留在 AGENTS.md：
   - 开发流程约束章节（高优先级，必须高可见）
   - 高价值内容（见 `references/agents-md-template.md`）
4. 输出修订后的完整 AGENTS.md 草稿，不只给 diff 建议

## 审计模式

参考 `references/agents-md-template.md` 中的验收检查清单逐项检查，输出：
- 问题清单（每条注明：问题类型 / 具体位置 / 建议操作）
- 优先修复顺序（超长 > 重复全局规则 > 缺少关键约束 > 缺少导航）

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

如果当前环境存在全局 `AGENTS.md`（通常在 `~/.config/opencode/AGENTS.md` 或 `~/.agents/AGENTS.md`），则：

- 语言约束（中文回复要求）→ **不写**
- 提交格式（feat/fix/docs...）→ 仅写项目特有的例外，通用格式 **不写**
- samsara 自我进化协议 → **不写**
- 高风险操作通用原则 → **不写**

项目 AGENTS.md 只写**该项目特有的、全局规则里没有的内容**。

---

## 不要做

- 不要把整份规范写回 AGENTS.md
- 不要生成与项目无关的泛化内容
- 不要编造未在代码或文档中出现过的约束
- 不要在没有依据时强行拆分过度
- 不要只给修改建议——修订模式必须输出可直接使用的完整草稿
