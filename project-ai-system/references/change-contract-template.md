# 变更执行契约模板

复杂变更使用。目标是把已批准的 spec/design/tasks 压缩成实现阶段可检查的契约。

## 输出路径

默认建议：`docs/ai/changes/<change-name>/execution-contract.md`

如项目已有变更目录规范，沿用现有路径。

## 使用条件

以下任一情况建议生成执行契约：

- 跨模块、跨平台、跨服务或跨仓库
- 涉及数据结构、权限、安全、兼容性、发布或迁移
- 需要多批次实现和分批验证
- 用户需求中存在明确非目标或高风险约束

## 模板

```markdown
# {change-name} Execution Contract

## Status

- State: proposed | approved | executing | blocked | closed
- Approved by: {user / reviewer}
- Last updated: {date}

## Intent Lock

- Goal: {本次变更要达成什么}
- Non-goals:
  - {明确不做什么}
- Success criteria:
  - {可验证成功标准}

## Approved Behavior

- {已批准的行为变化 1}
- {已批准的行为变化 2}

## Design Constraints

- Architecture: {架构/模块边界约束}
- API/Data: {接口、数据结构、兼容性约束}
- UX/Flow: {交互或流程约束，如适用}
- Risk controls: {风险控制要求}

## Task Batches

| Batch | Scope | Done When | Verification |
|-------|-------|-----------|--------------|
| 1 | {任务范围} | {完成定义} | `{验证命令或人工步骤}` |

## Test Obligations

- {必须新增或更新的测试}
- {必须运行的回归验证}
- {无法自动化时的人工验收步骤}

## Review Gates

- [ ] Batch review: spec 合规
- [ ] Batch review: 代码质量
- [ ] Final review: 全量 diff 与契约一致
- [ ] Final verification: 验证证据已记录

## Change Handling

- 需求变化：回到 spec/design，更新本契约后再继续
- 契约过时：停止实现，重新确认
- 连续修复失败：升级为根因分析，不继续试错
```

## 规则

- 不把未批准想法写进 Approved Behavior
- 不允许用“稍后补测试”“视情况验证”作为完成定义
- 如果实现偏离契约，先更新并确认契约，再继续实现
