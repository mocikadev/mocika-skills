---
name: agents-map
description: >
  Scaffold, revise, or audit a project's AI navigation system (AGENTS.md + docs/ skeleton + project
  skills). Use when initializing AI context for a new project, restructuring an oversized or chaotic
  AGENTS.md, checking whether an existing AGENTS.md follows map-style principles, or deciding what
  belongs in AGENTS.md vs docs/ vs a project skill. Triggers: "generate AGENTS.md", "init project
  AI context", "AGENTS.md is too long", "audit AGENTS.md", "set up docs skeleton".
compatibility: opencode
---

# agents-map

根据项目结构与约束，产出"地图式" AGENTS.md、docs/ 骨架和 project skill 清单。

## 三种模式

| 模式 | 触发条件 | 目标 |
|------|----------|------|
| **生成** | 无 AGENTS.md，或用户要求从头建 | 输出完整 AGENTS.md 草稿 + docs/ 骨架 + skill 清单 |
| **修订** | 有 AGENTS.md，需要重构 | 压缩冗余、将细则下沉到 docs/ 或 skill |
| **审计** | 有 AGENTS.md，只检查不改写 | 输出问题清单与优先级建议 |

---

## 第一步：按序读取项目信息

无论哪种模式，先按以下顺序读取，不得跳过或乱序：

1. 项目根目录列表（`ls`）
2. 已有 `AGENTS.md`（如存在）
3. `Cargo.toml` / `package.json` / `pyproject.toml` / `go.mod`（取第一个存在的）
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

1. 用收集到的信息填充 AGENTS.md 模板（见下文），按固定章节顺序输出
2. 输出 docs/ 目录骨架（见下文），注明哪些文件建议优先补充内容
3. 输出 project skill 建议清单（名称 + 一句话说明 + 放置路径）
4. 如发现跨文件同步约束，必须写入"关键约束"章节

## 修订模式

1. 逐章节审查现有 AGENTS.md
2. 识别以下问题内容并迁移：
   - 大段风格细则、完整枚举表格 → `docs/style/`
   - 架构背景说明、模块关系图 → `docs/design/`
   - 发布流程、分支策略 → `docs/process/`
   - 多步骤操作手册 → project skill
3. 保留"高价值内容"（见下文清单）
4. 输出修订后的完整 AGENTS.md 草稿，不只给 diff 建议

## 审计模式

按验收检查清单（见下文）逐项检查，输出：
- 问题清单（每条注明：问题类型 / 具体位置 / 建议操作）
- 优先修复顺序（超长 > 重复全局规则 > 缺少关键约束 > 缺少导航）

---

## AGENTS.md 固定章节结构

必须包含以下 7 个核心章节，顺序固定。可在第 7 章之后追加项目特有章节，但核心章节不得缺失或改名：

```
# 项目名
> 导航说明一句话

## 项目概览
## 技术栈
## 提交前检查清单
## 关键约束
## 常用命令
## 文档导航
## Skills 导航
```

---

## AGENTS.md 模板

~~~markdown
# {项目名}

> 此文件是 AI 代理的项目导航地图。详细规范查阅 `docs/` 目录。

## 项目概览

**{项目名}** — {一句话说明项目做什么}。  
仓库：`{org}/{repo}`  
当前状态：**{当前阶段/状态，如：Alpha / 主体功能完成 / 生产可用}**

## 技术栈

- {语言} + {主要框架/库}
- {其他关键依赖，限 3 条以内}

## 提交前检查清单

```bash
{命令 1}    # 说明
{命令 2}    # 说明
{命令 3}    # 说明
```

> ⚠️ {需要特别提醒的踩坑点，如：两个检查步骤互相独立、顺序不可颠倒等}

## 关键约束

- **{约束类型}**：{具体内容}
- 修改 `{文件 A}` 时必须同步修改 `{文件 B}`（如有跨文件同步要求）
- {其他 AI 无法从代码推断的约束}

## 常用命令

```bash
{启动命令}
{构建命令}
{测试命令}
```

## 文档导航

| 文档 | 路径 |
|------|------|
| 需求文档 | `docs/requirements/` |
| 技术设计 | `docs/design/` |
| 开发记录 | `docs/development/` |
| Bug 修订 | `docs/bugfix/` |
| 提交规范 | `docs/process/` |
| 代码风格 | `docs/style/` |
| 测试策略 | `docs/quality/` |

## Skills 导航

| Skill | 说明 |
|-------|------|
| `{skill-name}` | {一句话说明} |
~~~

---

## docs/ 标准骨架

```text
docs/
  requirements/   # 需求文档，按版本或功能模块拆分
  design/         # 技术设计，按模块或阶段拆分
  development/    # 开发记录、实现说明、关键决策
  bugfix/         # bug 修订记录，按版本或时间拆分
  process/        # 提交规范、发布流程、分支策略
  style/          # 命名约定、注释规范、代码格式
  quality/        # 测试策略、覆盖率要求、验收标准
  ops/            # 部署、环境配置、运维约束（可按需省略）
```

- 每类文档是独立目录，不堆在单个文件里，随项目演进按版本/功能/时间拆分
- `ops/` 可按需省略（小项目/纯库），其余 7 个目录建议都创建
- 目录内文件按需生成，不强制每个目录初始就有文件

---

## 高价值内容（优先写入 AGENTS.md）

以下类型的信息必须保留在 AGENTS.md，不得下沉到 docs/：

- **跨文件同步约束**：修改 X 必须同步修改 Y（AI 最容易遗漏的）
- **有序提交前检查清单**：带顺序和踩坑提示的 bash 命令块
- **特殊路径约定**：共享锁文件、中央目录、配置文件位置
- **当前项目状态**：阶段标记，让 AI 知道现在处于开发的哪个阶段
- **不可从代码推断的约束**：最低运行时/语言版本要求、二进制命名规范、平台兼容要求
- **高风险操作提示**：需要确认才能执行的破坏性操作

---

## 全局规则不重复原则

如果当前环境存在全局 `AGENTS.md`（通常在 `~/.config/opencode/AGENTS.md` 或 `~/.agents/AGENTS.md`），则：

- 语言约束（中文回复要求）→ **不写**
- 提交格式（feat/fix/docs...）→ 仅写项目特有的例外，通用格式 **不写**
- samsara 自我进化协议 → **不写**
- 高风险操作通用原则 → **不写**

项目 AGENTS.md 只写**该项目特有的、全局规则里没有的内容**。

---

## 验收检查清单

生成或修订完成后，逐项自检：

- [ ] AGENTS.md ≤ 100 行（超出则必须继续下沉内容）
- [ ] 包含且仅包含固定的 7 个章节，顺序正确
- [ ] 项目概览包含：名称、一句话说明、当前状态
- [ ] 提交前检查清单是有序 bash 代码块（不是散文描述）
- [ ] 关键约束包含跨文件同步要求（如有）
- [ ] 没有重复全局 AGENTS.md 中已有的通用规则
- [ ] 文档导航中的路径是真实存在或本次同步新建的
- [ ] docs/ 骨架至少包含 requirements/、design/、development/、bugfix/、process/、style/、quality/ 七个目录

---

## 不要做

- 不要把整份规范写回 AGENTS.md
- 不要生成与项目无关的泛化内容
- 不要编造未在代码或文档中出现过的约束
- 不要在没有依据时强行拆分过度
- 不要只给修改建议——修订模式必须输出可直接使用的完整草稿
