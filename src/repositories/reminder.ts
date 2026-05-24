import { invoke } from "@tauri-apps/api/core";
import type {
  Reminder,
  CreateReminderInput,
  ReminderRepository,
} from "./types";

function generateId(): string {
  return crypto.randomUUID();
}

function now(): string {
  return new Date().toISOString();
}

export class LocalReminderRepository implements ReminderRepository {
  async create(input: CreateReminderInput): Promise<Reminder> {
    const nextTrigger = input.type === "once"
      ? input.triggerAt || now()
      : new Date(Date.now() + (input.intervalSeconds || 300) * 1000).toISOString();

    const reminder: Reminder = {
      id: generateId(),
      title: input.title,
      type: input.type,
      intervalSeconds: input.intervalSeconds || null,
      nextTriggerAt: nextTrigger,
      isActive: true,
      soundEnabled: false,
      createdAt: now(),
    };
    await invoke("create_reminder", { reminder });
    return reminder;
  }

  async update(id: string, data: Partial<Reminder>): Promise<Reminder> {
    return await invoke<Reminder>("update_reminder", { id, data });
  }

  async delete(id: string): Promise<void> {
    await invoke("delete_reminder", { id });
  }

  async listActive(): Promise<Reminder[]> {
    return await invoke<Reminder[]>("list_reminders", { activeOnly: true });
  }

  async listAll(): Promise<Reminder[]> {
    return await invoke<Reminder[]>("list_reminders", { activeOnly: false });
  }

  async toggle(id: string): Promise<void> {
    await invoke("toggle_reminder", { id });
  }
}
