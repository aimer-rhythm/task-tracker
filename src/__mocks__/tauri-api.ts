import { vi } from "vitest";

const store: Record<string, unknown> = {};

export const invoke = vi.fn(async (cmd: string, args?: Record<string, unknown>) => {
  switch (cmd) {
    case "create_task":
      return;
    case "update_task":
      return args?.data;
    case "delete_task":
      return;
    case "get_task":
      return store[`task_${args?.id}`] ?? null;
    case "list_tasks":
      return Object.values(store).filter((v) => typeof v === "object" && v !== null && "title" in (v as object));
    case "add_subtask":
      return;
    case "toggle_subtask":
      return;
    case "delete_subtask":
      return;
    case "create_reminder":
      return;
    case "delete_reminder":
      return;
    case "toggle_reminder":
      return;
    case "list_reminders":
      return [];
    case "set_setting":
      return;
    case "get_all_settings":
      return {};
    case "set_window_opacity":
      return;
    case "set_always_on_top":
      return;
    case "minimize_to_tray":
      return;
    default:
      throw new Error(`Unknown command: ${cmd}`);
  }
});

export function __setStore(key: string, value: unknown) {
  store[key] = value;
}

export function __clearStore() {
  Object.keys(store).forEach((k) => delete store[k]);
}
