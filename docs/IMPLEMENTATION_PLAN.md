# 实施计划

## 开发阶段总览

```
Phase 1: 项目初始化与基础架构    → 预计 1 天
Phase 2: 核心任务管理 (P0)       → 预计 2 天
Phase 3: 定时提醒系统 (P0)       → 预计 1 天
Phase 4: 窗口控制 (P0)           → 预计 0.5 天
Phase 5: P1 功能                 → 预计 1 天
Phase 6: 打包与优化              → 预计 0.5 天
```

---

## Phase 1: 项目初始化与基础架构

### Step 1.1 — 初始化 Tauri + Vue 3 项目

```bash
npm create tauri-app@latest todo-tracker -- --template vue-ts
```

- 配置 Vite + TypeScript
- 配置 Tauri 2.0 权限（notification、window、tray）
- 安装依赖：pinia、uuid

### Step 1.2 — SQLite 数据层

文件：`src-tauri/src/db.rs`

- 集成 `rusqlite` crate
- 实现数据库初始化（建表 migration）
- 暴露 Tauri Command：`create_task`、`update_task`、`delete_task`、`list_tasks`

### Step 1.3 — 前端数据层抽象

文件：`src/repositories/task.ts`、`src/repositories/reminder.ts`

- 定义 TypeScript 接口（TaskRepository / ReminderRepository）
- 实现 `LocalTaskRepository`（调用 Tauri invoke）
- 预留 `RemoteTaskRepository` 接口（空实现）

### Step 1.4 — Pinia Store 搭建

文件：`src/stores/task.ts`、`src/stores/reminder.ts`、`src/stores/settings.ts`

- 任务状态管理
- 提醒状态管理
- 应用设置状态管理

---

## Phase 2: 核心任务管理

### Step 2.1 — 任务列表视图

文件：`src/views/TaskList.vue`

- 任务卡片列表（标题、优先级标记、进度条）
- 状态筛选（全部/待办/进行中/已完成）
- 空状态提示

### Step 2.2 — 创建/编辑任务

文件：`src/components/TaskForm.vue`

- 表单：标题、描述、优先级、截止日期
- 验证：标题必填
- 创建/编辑复用同一组件

### Step 2.3 — 任务详情与进度

文件：`src/components/TaskDetail.vue`

- 进度条拖拽调节
- 子任务 checklist（添加/勾选/删除）
- 状态流转按钮

### Step 2.4 — 完成记录

文件：`src/views/CompletedTasks.vue`

- 已完成任务列表
- 显示完成时间
- 支持恢复为待办

---

## Phase 3: 定时提醒系统

### Step 3.1 — Rust 端定时器

文件：`src-tauri/src/reminder.rs`

- 后台线程轮询提醒列表
- 触发时调用 Tauri notification API
- 循环提醒自动计算下次触发时间

### Step 3.2 — 提醒管理界面

文件：`src/views/ReminderList.vue`、`src/components/ReminderForm.vue`

- 创建提醒（类型选择：单次/循环）
- 循环提醒：间隔输入（分钟/小时）
- 提醒列表：显示状态、下次触发时间
- 操作：暂停/恢复/删除

### Step 3.3 — 通知权限与配置

文件：`src-tauri/capabilities/`

- 配置 Tauri notification 权限
- Windows 通知渠道注册

---

## Phase 4: 窗口控制

### Step 4.1 — 置顶与透明度

文件：`src-tauri/src/window.rs`、`src/components/WindowControls.vue`

- Tauri API：`set_always_on_top(bool)`
- Tauri API：`set_opacity(f64)` — 范围 0.1-1.0（Tauri 2.0 原生支持）
- 前端：置顶按钮 + 透明度滑块

### Step 4.2 — 系统托盘

文件：`src-tauri/src/tray.rs`

- 最小化到托盘（关闭按钮行为覆盖）
- 托盘右键菜单：显示/退出
- 双击托盘图标恢复窗口

---

## Phase 4.5: 单元测试 ✅

### Step 4.5.1 — Rust 测试基础设施

文件：`src-tauri/src/db.rs`（`Database::new_in_memory()`）

- 内存 SQLite 测试构造器，避免文件系统依赖
- 各 command 模块内 `#[cfg(test)] mod tests` 直接测试 SQL 逻辑
- 覆盖：表初始化、任务 CRUD、子任务操作、提醒管理、设置读写

### Step 4.5.2 — TypeScript 测试基础设施

文件：`vitest.config.ts`、`src/__mocks__/tauri-api.ts`

- Vitest + happy-dom 环境
- Mock `@tauri-apps/api/core` 的 `invoke` 函数
- @pinia/testing 支持 Store 隔离测试

### Step 4.5.3 — 前端单元测试

文件：`src/repositories/__tests__/`、`src/stores/__tests__/`

- Repository 层：验证 invoke 调用参数、返回值构造逻辑
- Store 层：验证状态管理、computed 属性、异步操作、错误处理

---

## Phase 5: P1 功能

### Step 5.1 — 分类与标签

- 侧边栏分类列表
- 任务关联标签
- 按分类筛选

### Step 5.2 — 搜索

- 顶部搜索框
- 实时过滤（标题 + 标签匹配）

### Step 5.3 — 窗口记忆 & 开机自启

- 保存/恢复窗口位置和尺寸
- Windows 注册表写入开机启动项（可选）

---

## Phase 6: 打包与优化

### Step 6.1 — 构建优化

- Rust release 编译优化（LTO、strip）
- 前端 tree-shaking、代码分割
- 验证包体积 < 10MB

### Step 6.2 — 性能验证

- 空闲状态内存/CPU 测量
- 长时间运行稳定性测试（8小时+）
- 定时器精度验证

### Step 6.3 — 安装包生成

- Tauri bundler 生成 `.msi` / `.exe` 安装包
- 应用图标配置

---

## 文件结构规划

```
todo-tracker/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # 入口，注册 commands
│   │   ├── db.rs            # SQLite 数据库操作 (+测试)
│   │   ├── commands/
│   │   │   ├── task.rs      # 任务相关 commands (+测试)
│   │   │   ├── reminder.rs  # 提醒相关 commands (+测试)
│   │   │   └── settings.rs  # 设置相关 commands (+测试)
│   │   ├── reminder.rs      # 定时器后台线程
│   │   ├── tray.rs          # 系统托盘
│   │   └── window.rs        # 窗口控制
│   ├── Cargo.toml
│   └── capabilities/        # Tauri 2.0 权限配置
├── src/
│   ├── App.vue
│   ├── main.ts
│   ├── __mocks__/
│   │   └── tauri-api.ts     # Tauri invoke mock
│   ├── views/
│   │   ├── TaskList.vue
│   │   ├── CompletedTasks.vue
│   │   └── ReminderList.vue
│   ├── components/
│   │   ├── TaskCard.vue
│   │   ├── TaskForm.vue
│   │   ├── TaskDetail.vue
│   │   ├── ReminderForm.vue
│   │   ├── WindowControls.vue
│   │   └── Sidebar.vue
│   ├── stores/
│   │   ├── task.ts
│   │   ├── reminder.ts
│   │   ├── settings.ts
│   │   └── __tests__/       # Store 单元测试
│   │       ├── task.test.ts
│   │       ├── reminder.test.ts
│   │       └── settings.test.ts
│   ├── repositories/
│   │   ├── types.ts         # 接口定义
│   │   ├── task.ts          # 任务 Repository
│   │   ├── reminder.ts      # 提醒 Repository
│   │   └── __tests__/       # Repository 单元测试
│   │       ├── task.test.ts
│   │       └── reminder.test.ts
│   └── styles/
│       └── main.css
├── package.json
├── vite.config.ts
├── vitest.config.ts          # 测试配置
├── tsconfig.json
└── docs/
    ├── REQUIREMENTS.md
    └── IMPLEMENTATION_PLAN.md
```

---

## 开发顺序依赖图

```
Phase 1.1 (项目初始化)
    ↓
Phase 1.2 (SQLite) → Phase 1.3 (前端数据层) → Phase 1.4 (Store)
    ↓                                              ↓
Phase 2.1-2.4 (任务管理 UI)              Phase 3.1 (定时器)
    ↓                                              ↓
Phase 4 (窗口控制)                       Phase 3.2-3.3 (提醒 UI)
    ↓                                              ↓
    └──────────────── Phase 5 (P1) ────────────────┘
                           ↓
                     Phase 6 (打包)
```

---

## 风险与缓解

| 风险 | 缓解措施 |
|------|---------|
| Tauri 2.0 透明度 API 兼容性 | 验证 WebView2 版本要求，降级方案用 CSS opacity |
| 定时器长时间运行漂移 | 使用绝对时间戳比较而非相对延迟 |
| SQLite 并发写入 | 单写入线程 + 消息队列模式 |
| Windows 通知权限被禁 | 应用内 fallback 提示 |
