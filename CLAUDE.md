# TaskTracker — 项目上下文

## 项目概述
轻量级 Windows 桌面待办任务跟踪应用，420x560px 小窗口，支持置顶/透明度/系统托盘。

## 技术栈
- **桌面框架**: Tauri 2.0 (Rust 后端 + WebView2)
- **前端**: Vue 3 + TypeScript + Pinia
- **构建**: Vite
- **数据库**: SQLite (rusqlite, bundled)
- **测试**: Rust `#[cfg(test)]` + Vitest + @pinia/testing + happy-dom
- **UI**: 自定义 CSS 变量双主题 (light/dark)，基于 Inter 字体，Soft Minimal 风格

## 当前进度
- [x] Phase 1: 项目初始化 — 骨架已搭建完成
- [x] Phase 1.2: SQLite 数据层 — Rust commands 已实现
- [x] Phase 1.3-1.4: 前端数据层 + Store — Repository 接口 + Pinia 已实现
- [x] Phase 2: 任务管理 UI — TaskList.vue 已实现（手风琴展开、子任务、进度条、内联编辑）
- [x] Phase 3: 定时提醒系统 — 后台线程 + ReminderList.vue 已实现
- [x] Phase 4: 窗口控制 — 置顶/透明度/托盘/主题切换已实现
- [x] Phase 4.5: 设置持久化 — 置顶、主题、透明度跨重启恢复（Rust 侧 Win32 API 直接应用）
- [x] Phase 4.6: 窗口记忆 — 位置/尺寸保存与恢复，含 0 宽高防护
- [x] 编译验证 — `cargo test` 通过（17 tests），`npm run test` 通过（41 tests）
- [x] 单元测试 — Rust 数据层 + TS stores/repositories 全覆盖
- [x] UI 打磨 — 图标统一 16px、tab 图标+文字、收起态进度条、中文本地化
- [ ] Phase 5: P1 功能（分类、搜索、开机自启）
- [ ] Phase 6: 打包优化

## 已知问题
1. 新建任务可能立即显示为完成状态（疑似前端渲染或数据问题，待复现排查）

## 设计资源
- 原型图: `design/stitch_activity_tracker/` (light/dark/opacity 三套)
- 设计系统: `design/stitch_activity_tracker/precision_soft_minimal/DESIGN.md`
- 需求文档: `docs/REQUIREMENTS.md`
- 实施计划: `docs/IMPLEMENTATION_PLAN.md`

## 架构要点
- 数据层通过 Repository 接口抽象，预留云同步（当前仅本地 SQLite）
- 提醒系统使用 Rust 后台线程轮询 + Windows 系统通知
- 窗口无边框 (decorations: false)，自定义标题栏拖拽
- CSS 变量实现主题切换，`.light` / `.dark` class 切换
- 设置持久化双保险：Rust setup() 直接通过 Win32 API 恢复 + 前端 loadSettings() 兜底
- 窗口状态保存带防护：onResized 过滤 0x0，restore_window_state 校验最小尺寸

## 关键命令
```bash
npm install          # 安装前端依赖
npm run dev          # 启动 Vite 开发服务器
npm run test         # 运行前端单元测试 (Vitest)
npm run test:watch   # 监听模式运行测试
npm run tauri dev    # 启动 Tauri 开发模式（前后端联调）
npm run tauri build  # 构建生产包
cargo test           # 运行 Rust 单元测试（在 src-tauri 目录）
cargo check          # 检查 Rust 编译（在 src-tauri 目录）
```

## 测试架构
- **Rust 测试** (17 tests): `db.rs`、`commands/task.rs`、`commands/reminder.rs`、`commands/settings.rs` 使用 `#[cfg(test)]` + 内存 SQLite (`Database::new_in_memory()`)
- **TypeScript 测试** (41 tests): Vitest + happy-dom + @pinia/testing，mock `@tauri-apps/api/core` 的 `invoke`
- **测试文件位置**: `src/repositories/__tests__/`、`src/stores/__tests__/`
- **配置**: `vitest.config.ts`（独立于 vite.config.ts）

## Windows 环境要求
- Node.js 18+
- Rust (rustup)
- VS Build Tools (C++ 桌面开发工作负载)
- WebView2 Runtime (Win10/11 自带)
