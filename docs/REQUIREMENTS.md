# 待办任务跟踪桌面应用 — 需求设计文档

## 项目概述

轻量级 Windows 桌面待办任务跟踪应用，支持任务管理、定时提醒、窗口控制，纯本地运行（内外网均可）。

## 技术选型

| 层级 | 技术 | 说明 |
|------|------|------|
| 桌面框架 | Tauri 2.0 | Rust 后端 + WebView2，轻量高性能 |
| 前端 | Vue 3 + TypeScript | Composition API + Pinia 状态管理 |
| UI 风格 | 简约 | 无多余装饰，信息密度优先 |
| 数据库 | SQLite | 本地持久化，预留同步接口 |
| 构建工具 | Vite | 快速开发体验 |

## 功能模块

### 模块 1：任务管理（P0）

| 功能 | 描述 |
|------|------|
| 创建任务 | 标题、描述、优先级(高/中/低)、截止日期 |
| 任务状态流转 | 待办 → 进行中 → 已完成 |
| 进度追踪 | 百分比进度条 + 子任务 checklist |
| 完成记录 | 历史完成任务列表，含完成时间 |

### 模块 2：定时提醒（P0）

| 功能 | 描述 |
|------|------|
| 循环提醒 | 自定义间隔（如每5分钟喝水） |
| 单次提醒 | 指定时间点提醒 |
| 提醒方式 | Windows 系统通知 |
| 提醒管理 | 暂停/恢复/删除提醒 |

### 模块 3：窗口控制（P0）

| 功能 | 描述 |
|------|------|
| 窗口置顶 | 一键切换 always-on-top |
| 透明度调节 | 滑块控制 10%-100% |
| 最小化到托盘 | 关闭窗口时最小化到系统托盘 |

### 模块 4：任务分类与搜索（P1）

| 功能 | 描述 |
|------|------|
| 标签/分组 | 工作、生活、学习等自定义分类 |
| 快速搜索 | 按标题/标签筛选 |
| 窗口尺寸记忆 | 记住上次窗口位置和大小 |
| 开机自启 | 可选注册 Windows 开机启动 |

### 模块 5：扩展功能（P2）

| 功能 | 描述 |
|------|------|
| 预设提醒模板 | 喝水、休息、站立等常用模板 |
| 迷你模式 | 紧凑视图只显示当前任务 |
| 数据导出 | JSON/CSV 导出 |
| 数据备份 | 手动备份/恢复数据库文件 |

## 数据模型

```sql
-- 任务表
CREATE TABLE tasks (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL DEFAULT 'todo',  -- todo | in_progress | done
  priority TEXT NOT NULL DEFAULT 'medium',  -- high | medium | low
  progress INTEGER NOT NULL DEFAULT 0,  -- 0-100
  category TEXT,
  due_date TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  completed_at TEXT
);

-- 子任务表
CREATE TABLE subtasks (
  id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  is_done INTEGER NOT NULL DEFAULT 0,
  sort_order INTEGER NOT NULL DEFAULT 0
);

-- 标签表
CREATE TABLE tags (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

-- 任务-标签关联
CREATE TABLE task_tags (
  task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (task_id, tag_id)
);

-- 提醒表
CREATE TABLE reminders (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  type TEXT NOT NULL,  -- once | recurring
  interval_seconds INTEGER,
  next_trigger_at TEXT NOT NULL,
  is_active INTEGER NOT NULL DEFAULT 1,
  sound_enabled INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL
);

-- 应用设置
CREATE TABLE app_settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

## 数据层抽象（预留云同步）

```typescript
// 数据访问接口 — 当前实现为 SQLite，未来可替换为 API 调用
interface TaskRepository {
  create(task: CreateTaskInput): Promise<Task>
  update(id: string, data: UpdateTaskInput): Promise<Task>
  delete(id: string): Promise<void>
  getById(id: string): Promise<Task | null>
  list(filter?: TaskFilter): Promise<Task[]>
}

interface ReminderRepository {
  create(reminder: CreateReminderInput): Promise<Reminder>
  update(id: string, data: UpdateReminderInput): Promise<Reminder>
  delete(id: string): Promise<void>
  listActive(): Promise<Reminder[]>
}
```

## 性能指标

| 指标 | 目标值 |
|------|--------|
| 安装包大小 | < 10MB |
| 启动时间 | < 1秒 |
| 空闲内存占用 | < 50MB |
| 空闲 CPU 占用 | < 0.5% |
| 定时器精度 | ±1秒 |

## UI 布局

```
┌─────────────────────────────────────┐
│ [📌置顶] [透明度━━━○━] [─][□][×]  │
├─────────────────────────────────────┤
│ ┌─ 分类 ─┐  ┌─────────────────────┐│
│ │ 全部    │  │ + 新建任务          ││
│ │ 工作    │  ├─────────────────────┤│
│ │ 生活    │  │ ☐ 完成项目报告  [高]││
│ │ 学习    │  │   ████████░░ 80%    ││
│ │         │  │ ☐ 买菜        [低]  ││
│ │ ⏰提醒  │  │ ☑ 回复邮件    [中]  ││
│ └─────────┘  └─────────────────────┘│
├─────────────────────────────────────┤
│ ⏰ 喝水提醒: 每5分钟 [暂停] [删除] │
└─────────────────────────────────────┘
```

## 约束与决策

- 纯本地运行，无需网络连接
- 数据层通过 Repository 接口抽象，预留未来云同步能力
- 不实现用户认证（单机单用户）
- 提醒仅使用 Windows 系统通知，不做自定义弹窗
