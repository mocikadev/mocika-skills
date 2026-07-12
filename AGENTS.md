# Mocika Skills

> 本文件是 AI 代理在 `mocika-skills` 仓库中的入口地图。技能清单、能力简介和安装命令以根目录 `README.md` 为准。

## 项目概览

本仓库托管与日常开发工作相关的 AI 代理技能。采用扁平结构：每个顶层 Skill 目录都是独立、完整、可单独安装的技能包。

```text
mocika-skills/
  ├── skill-name/
  │   ├── SKILL.md           # 必需：技能入口指令
  │   ├── README.md          # 必需：面向使用者的完整说明
  │   ├── references/        # 可选：按需加载的详细参考
  │   ├── scripts/           # 可选：可复用的确定性工具
  │   └── assets/            # 可选：模板、图片等静态资源
  ├── README.md              # 技能导航、简介和安装命令
  ├── AGENTS.md              # 当前文件
  └── .gitignore             # Git 忽略规则
```

## 高优先级约束

### 安全隔离

- 绝对禁止在 Skill、参考文档、脚本、模板或示例中硬编码真实密码、Token、API Key、私钥等敏感凭证。
- 需要凭证时，指导代理从环境变量、密钥管理器或项目配置中心读取；示例只能使用明显的占位值。

### Skill 包结构

- 每个顶层 Skill 目录必须自包含，不得依赖本仓库其他 Skill 目录中的私有文件。
- 每个 Skill 必须包含 `SKILL.md` 和 `README.md`；`references/`、`scripts/`、`assets/` 按实际需要创建，禁止为了目录完整制造空目录。
- `SKILL.md` 必须包含有效 YAML frontmatter，至少提供 `name` 和 `description`。
- `SKILL.md` 应控制在 200 行以内；详细规则和重型参考下沉到该 Skill 的 `references/`。
- 新建或重构 Skill 时使用 `create-skill`，行为约束型 Skill 还应先建立无 Skill 基线，再验证加载后的行为变化。

### README 质量

每个 Skill 的 `README.md` 不得只罗列目录或提供一句话简介，必须写清：

- 解决什么问题：使用场景和用户痛点。
- 提供哪些能力：所有模式、输入和输出。
- 产物示例：生成内容及其格式。
- 安装与使用：安装命令和触发示例。

### 语言

- `SKILL.md` 正文、README、参考文档、规范和注释使用简体中文。
- Skill 名称、frontmatter 约定前缀、命令、API、文件名和第三方专有名词可保留英文。

### 根目录纯洁性

- 除 Git 元数据和 `.gitignore` 外，根目录只允许存在 Skill 目录、`README.md` 和 `AGENTS.md`。
- 严禁把外部代码仓库以克隆、子模块或手动拷贝形式放入本仓库。
- 调查外部源码必须在本仓库之外的临时目录进行，结束后不得在仓库内遗留文件。

## 跨文件同步规则

新增、删除或重命名 Skill 时，必须同步更新根目录 `README.md`：

1. 技能清单中的名称、链接、问题、能力、产物和触发示例。
2. “安装与使用”代码块中的 `skm install` 命令。

Skill 能力、模式或安装方式发生变化时，同时检查对应 Skill README 和根 README 是否仍一致。

## 常用工作流

### 创建或更新 Skill

1. 在仓库根目录加载 `create-skill`，创建或更新目标 Skill 包。
2. 将详细内容按需拆入 Skill 自己的 `references/`、`scripts/` 或 `assets/`。
3. 完善 Skill README，并同步根 README。
4. 使用该 Skill 的代表性场景验证触发、输出和约束行为。
5. 完成下方检查后再提交。

### 安装 Skill

```bash
SKILL_NAME=arch-guard
skm install "mocikadev/mocika-skills:${SKILL_NAME}" --link-to all
```

不要使用 `skm link ./skill-name` 代替仓库安装流程；根 README 中的实际命令是权威入口。

## 完成前检查

对每个新增或修改的 Skill，至少确认：

```bash
SKILL_DIR=arch-guard
test -f "$SKILL_DIR/SKILL.md"
test -f "$SKILL_DIR/README.md"
test "$(wc -l < "$SKILL_DIR/SKILL.md")" -le 200
git diff --check
```

同时人工检查：

- frontmatter 的 `name`、`description` 有效，且触发条件能够被代理检索。
- 没有 `TODO`、`TBD`、无效占位内容或真实敏感凭证。
- README 覆盖问题、能力、输入输出、产物示例、安装和触发方式。
- `references/` 中的文件能从 `SKILL.md` 或 README 找到，不存在孤立文档。
- 新增、删除、重命名或改变安装方式后，根 README 已同步。
- 行为型 Skill 已用代表性压力场景验证，不只做 Markdown 格式检查。
