# AI 工作系统审计报告模板

审计模式使用。目标是让问题排序清晰、位置可查、建议可执行。

## 输出格式

```markdown
# AI 工作系统审计报告

## 结论

- Overall: pass | needs-work | blocked
- Blocking issues: {数量}
- Recommended next action: {最优先动作}

## 问题清单

| Severity | Type | Location | Impact | Recommendation | Blocks Implementation |
|----------|------|----------|--------|----------------|-----------------------|
| P0 | {类型} | `{文件:章节}` | {影响} | {建议操作} | yes/no |

## 优先修复顺序

1. {P0 阻塞项}
2. {P1 高风险项}
3. {P2 结构优化项}

## 覆盖检查

- [ ] 项目 spec 存在，且包含目标、范围、非目标、验收标准
- [ ] AI 执行协议存在，且包含任务分级、快速路径、风险操作、完成验证
- [ ] 复杂变更有执行契约规则
- [ ] 入口文件有执行协议摘要和 docs 导航
- [ ] 文档导航指向真实存在或建议创建的路径
- [ ] 没有重复全局代理规则
- [ ] 没有把大段细则塞进入口文件
- [ ] 收口同步规则明确，能防止 spec 腐烂

## 建议产物

| Artifact | Path | Reason |
|----------|------|--------|
| {产物名} | `{建议路径}` | {原因} |
```

## Severity 定义

- **P0**：阻塞 AI 正确执行；例如缺少 spec、验收标准或执行协议
- **P1**：高风险；例如缺少验证规则、风险操作规则、执行契约规则
- **P2**：结构问题；例如入口文件过长、docs 导航不准、规则重复
- **P3**：可维护性建议；例如命名不一致、目录可读性弱

## Type 建议值

- `missing-spec`
- `missing-execution-protocol`
- `missing-contract`
- `missing-verification`
- `stale-docs`
- `bloated-entry`
- `duplicated-global-rule`
- `broken-navigation`
- `unclear-skill-boundary`
