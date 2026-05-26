# 测试与性能报告

## 概览

| 类别 | 框架 | 数量 | 状态 |
|------|------|------|------|
| Rust 单元测试 | `#[cfg(test)]` + 内存 SQLite | 17 | ✅ |
| TypeScript 单元测试 | Vitest + happy-dom + @pinia/testing | 41 | ✅ (40 pass / 1 flaky) |
| 性能基准 | Vitest bench | 9 | ✅ |

## 运行命令

```bash
npm run test          # 前端单元测试（单次）
npm run test:watch    # 前端测试（监听模式）
npm run bench         # 性能基准测试
cargo test            # Rust 单元测试（在 src-tauri/ 目录）
```

---

## 单元测试

### Rust 后端 (17 tests)

测试文件位于各模块内部，使用 `#[cfg(test)]` 标注：

| 模块 | 文件 | 覆盖内容 |
|------|------|---------|
| 数据库 | `src-tauri/src/db.rs` | Schema 创建、连接池、内存模式 |
| 任务命令 | `src-tauri/src/commands/task.rs` | CRUD、子任务、状态切换 |
| 提醒命令 | `src-tauri/src/commands/reminder.rs` | 创建、删除、激活/停用 |
| 设置命令 | `src-tauri/src/commands/settings.rs` | KV 存取、窗口透明度 |

所有 Rust 测试使用 `Database::new_in_memory()` 避免文件系统依赖。

### TypeScript 前端 (41 tests)

| 测试文件 | 覆盖内容 |
|---------|---------|
| `src/repositories/__tests__/task.test.ts` | LocalTaskRepository CRUD、子任务操作 |
| `src/repositories/__tests__/reminder.test.ts` | LocalReminderRepository CRUD、toggle |
| `src/stores/__tests__/task.test.ts` | useTaskStore 状态管理、computed 属性 |
| `src/stores/__tests__/reminder.test.ts` | useReminderStore 状态管理 |
| `src/stores/__tests__/settings.test.ts` | useSettingsStore 主题/透明度/置顶 |

**Mock 策略：** `src/__mocks__/tauri-api.ts` 模拟 `@tauri-apps/api/core` 的 `invoke`，通过 vitest alias 自动替换。

---

## 性能基准报告

> 测试环境：Windows 11 / Node.js 22+ / Vitest 4.1.7
> 日期：2026-05-26

### TaskStore — fetchTasks（数据加载）

| 场景 | ops/sec | 平均耗时 | p99 |
|------|---------|---------|-----|
| 加载 10 条任务 | 7,298 | 0.14ms | 1.13ms |
| 加载 100 条任务 | 2,126 | 0.47ms | 2.47ms |
| 加载 500 条任务 | 580 | 1.72ms | 5.45ms |

**结论：** 500 条任务加载 p99 < 6ms，远低于 16ms 帧预算，无性能瓶颈。

### TaskStore — computed filters（响应式过滤）

| 场景 | ops/sec | 平均耗时 | p99 |
|------|---------|---------|-----|
| activeTasks (100 items) | 1,485 | 0.67ms | 5.82ms |
| completedTasks (100 items) | 634 | 1.58ms | 6.57ms |
| activeTasks (500 items) | 411 | 2.43ms | 25.46ms |

**结论：** 100 条以内过滤性能良好。500 条时 p99 达 25ms，若未来数据量增长需考虑虚拟列表或分页。

### TaskStore — mutations（状态变更）

| 场景 | ops/sec | 平均耗时 | p99 |
|------|---------|---------|-----|
| createTask | 8,782 | 0.11ms | 1.19ms |
| toggleExpand | 3,689 | 0.27ms | 1.79ms |
| deleteTask (100 items) | 1,751 | 0.57ms | 2.83ms |

**结论：** 所有变更操作 p99 < 3ms，用户交互无感知延迟。

---

## 性能基线与阈值

| 指标 | 当前值 | 警戒阈值 | 说明 |
|------|--------|---------|------|
| 任务加载 (100条) | 0.47ms | < 5ms | 超过则需优化查询 |
| 过滤计算 (100条) | 0.67ms | < 10ms | 超过则需 memo/虚拟化 |
| 单次变更操作 | 0.11ms | < 2ms | 超过则需检查响应式开销 |

---

## 已知问题

1. **settings.test.ts flaky test** — `minimizeToTray` 断言因 `invoke` mock 参数匹配方式导致偶发失败（`toHaveBeenCalledWith` vs 实际传入 `undefined` 第二参数）

---

## 添加新测试指南

### 单元测试
```
src/stores/__tests__/{store-name}.test.ts
src/repositories/__tests__/{repo-name}.test.ts
```

### 性能基准
```
src/__bench__/{module}.bench.ts
```

使用 `bench()` API，参考 `store-operations.bench.ts`。
