# TaskTrack

轻量级 Windows 桌面待办任务跟踪应用，420×560px 小窗口，常驻桌面不打扰。

## 功能

- 任务管理：创建/完成/删除任务，内联编辑标题，支持子任务与进度条
- 优先级：高/中/低三级，内联下拉切换，收起态旗帜图标显示
- 定时提醒：循环或单次提醒，Windows 系统通知推送
- 窗口控制：置顶、透明度调节、最小化到系统托盘
- 设置持久化：置顶、主题、透明度跨重启自动恢复
- 窗口记忆：位置与尺寸跨重启保存
- 双主题：Light / Dark 一键切换
- 中文界面

## 截图

> *(待补充)*

## 环境要求

- Windows 10 / 11
- [Node.js 18+](https://nodejs.org/)
- [Rust (rustup)](https://rustup.rs/)
- VS Build Tools（C++ 桌面开发工作负载）
- WebView2 Runtime（Win10/11 自带）

## 开发

```bash
npm install
npm run tauri dev
```

## 构建

```bash
npm run tauri build
```

输出在 `src-tauri/target/release/bundle/`，包含 NSIS 安装包和 MSI。

## 技术栈

- [Tauri 2.0](https://tauri.app/) — Rust 后端 + WebView2
- Vue 3 + TypeScript + Pinia
- Vite
- SQLite (rusqlite bundled)

## License

MIT
