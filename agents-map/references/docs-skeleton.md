# docs/ 标准骨架

## 目录结构

```text
docs/
  requirements/   # 需求文档，按版本或功能模块拆分
  design/         # 技术设计，按模块或阶段拆分
  ui/             # UI/UX 设计，交互稿、组件规范、视觉规范
  development/    # 开发记录、实现说明、关键决策
  bugfix/         # bug 修订记录，按版本或时间拆分
  process/        # 提交规范、发布流程、分支策略
  style/          # 命名约定、注释规范、代码格式
  quality/        # 测试策略、覆盖率要求、验收标准
  ops/            # 部署、环境配置、运维约束（可按需省略）
```

## 各目录职责说明

| 目录 | 职责 | 典型文件示例 |
|------|------|-------------|
| `requirements/` | 功能需求、用户故事、验收条件 | `v1.0-requirements.md`、`feature-auth.md` |
| `design/` | 架构设计、模块划分、数据流、API 设计 | `architecture.md`、`api-design.md` |
| `ui/` | 交互设计、组件规范、视觉规范、原型说明 | `wireframes.md`、`component-spec.md` |
| `development/` | 实现细节、开发笔记、技术选型记录、关键决策 | `impl-notes.md`、`adr-001.md` |
| `bugfix/` | 已知问题、修复记录、版本变更日志 | `v1.1-fixes.md`、`known-issues.md` |
| `process/` | 提交规范、发布流程、分支策略、变更管理 | `commit-convention.md`、`release.md` |
| `style/` | 命名约定、注释规范、代码格式、语言要求 | `naming.md`、`code-style.md` |
| `quality/` | 测试策略、覆盖率要求、验收标准、QA 流程 | `test-strategy.md`、`acceptance.md` |
| `ops/` | 部署流程、环境配置、运维约束（可按需省略） | `deploy.md`、`env-setup.md` |

## 使用约束

- 每类文档是独立目录，不堆在单个文件里，随项目演进按版本/功能/时间拆分
- `ops/` 可按需省略（小项目/纯库），其余 8 个目录建议都创建
- 目录内文件按需生成，不强制每个目录初始就有文件
- 允许在以上目录下新增子目录，但命名和职责必须明确

## 与开发流程的对应关系

```
需求阶段  →  requirements/
设计阶段  →  design/ + ui/
开发阶段  →  development/
测试阶段  →  quality/
发布流程  →  process/
缺陷记录  →  bugfix/
运维部署  →  ops/
```
