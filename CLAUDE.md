# TaskTracker — 项目上下文

## 项目概述
轻量级 Windows 桌面待办任务跟踪应用，420x560px 小窗口，支持置顶/透明度/系统托盘。

## 技术栈
- **桌面框架**: Tauri 2.0 (Rust 后端 + WebView2)
- **前端**: Vue 3 + TypeScript + Pinia
- **构建**: Vite
- **数据库**: SQLite (rusqlite, bundled)
- **UI**: 自定义 CSS 变量双主题 (light/dark)，基于 Inter 字体，Soft Minimal 风格

## 当前进度
- [x] Phase 1: 项目初始化 — 骨架已搭建完成
- [x] Phase 1.2: SQLite 数据层 — Rust commands 已实现
- [x] Phase 1.3-1.4: 前端数据层 + Store — Repository 接口 + Pinia 已实现
- [x] Phase 2: 任务管理 UI — TaskList.vue 已实现（手风琴展开、子任务、进度条）
- [x] Phase 3: 定时提醒系统 — 后台线程 + ReminderList.vue 已实现
- [x] Phase 4: 窗口控制 — 置顶/透明度/托盘/主题切换已实现
- [ ] **待验证**: cargo check 编译通过（需要 Windows 环境）
- [ ] Phase 5: P1 功能（分类、搜索、窗口记忆、开机自启）
- [ ] Phase 6: 打包优化

## 待解决
1. 编译验证 — 在 Windows 上运行 `cargo check` 确认无编译错误
2. 修复可能的 Rust 编译问题（API 兼容性等）
3. 前端 `npm run dev` 验证页面渲染
4. 集成测试 `npm run tauri dev`

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

## 关键命令
```bash
npm install          # 安装前端依赖
npm run dev          # 启动 Vite 开发服务器
npm run tauri dev    # 启动 Tauri 开发模式（前后端联调）
npm run tauri build  # 构建生产包
cargo check          # 检查 Rust 编译（在 src-tauri 目录）
```

## Windows 环境要求
- Node.js 18+
- Rust (rustup)
- VS Build Tools (C++ 桌面开发工作负载)
- WebView2 Runtime (Win10/11 自带)
