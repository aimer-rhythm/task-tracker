# Add focus mode

## Goal

Add a session-only focus mode that reduces visual distractions by showing only the current task list content. The mode should help users concentrate on active tasks without changing persisted settings or native window behavior.

## What I already know

- The app shell lives in `src/App.vue` and currently renders a custom titlebar, settings menu, task/reminder tabs, and a main content area.
- The task list view lives in `src/views/task-list/index.vue` and currently renders quick task entry, all tasks, and footer stats.
- `src/stores/task.ts` already exposes `tasks`, `activeTasks`, `completedTasks`, `expandedId`, and task mutation methods.
- Focus mode should be a temporary UI state, not a persisted app setting.

## Requirements

- Add a visible focus-mode entry button in the normal titlebar controls.
- Entering focus mode must switch the visible content to tasks, even if the user was on reminders.
- In focus mode, hide the titlebar, settings controls, window controls, and task/reminder tab bar.
- In focus mode, hide the quick task entry and footer stats.
- In focus mode, render only unfinished tasks using `taskStore.activeTasks`.
- Provide a lightweight in-content exit affordance for focus mode.
- The focus-mode top area must remain draggable for moving the frameless Tauri window, while the exit affordance remains clickable.
- Pressing `Escape` must exit focus mode.
- If there are no unfinished tasks, show a minimal empty state with text equivalent to "没有待专注的任务" and an exit button.
- Focus mode state must be session-only and reset after app restart.
- Focus mode must not automatically change always-on-top, opacity, window size, or persisted settings.

## Acceptance Criteria

- [ ] Normal mode still renders the existing titlebar, settings, tabs, quick task entry, task list, and footer stats.
- [ ] Clicking the titlebar focus button enters focus mode.
- [ ] Entering focus mode while on reminders shows the task list in focus mode.
- [ ] Focus mode hides the app chrome and list-management UI.
- [ ] Focus mode shows unfinished tasks and excludes completed tasks.
- [ ] Focus mode can be exited with the in-content exit button.
- [ ] The focus-mode top area can be used to drag the window without blocking the exit button.
- [ ] Focus mode can be exited with `Escape`.
- [ ] Empty focus mode shows a minimal empty state and exit button.
- [ ] Automated frontend tests still pass.

## Out of Scope

- Persisting focus mode in `settingsStore` or Rust settings.
- Adding database fields or Tauri commands.
- Changing native window size, opacity, always-on-top, tray behavior, or window persistence.
- Reworking task sorting, task filtering, or reminder behavior.

## Technical Notes

- Expected implementation scope is limited to `src/App.vue` and `src/views/task-list/index.vue` unless validation shows additional test updates are needed.
- Reuse existing task store computed state instead of adding a new store concept.
- Prefer local component state and typed props/emits for the focus-mode boundary.
