import { invoke } from "@tauri-apps/api/core";
import type {
  Task,
  Subtask,
  CreateTaskInput,
  UpdateTaskInput,
  TaskFilter,
  TaskRepository,
} from "./types";

function generateId(): string {
  return crypto.randomUUID();
}

function now(): string {
  return new Date().toISOString();
}

export class LocalTaskRepository implements TaskRepository {
  async create(input: CreateTaskInput): Promise<Task> {
    const task: Task = {
      id: generateId(),
      title: input.title,
      description: input.description || "",
      status: "todo",
      priority: input.priority || "medium",
      progress: 0,
      category: "",
      tags: [],
      subtasks: [],
      dueDate: input.dueDate || null,
      createdAt: now(),
      updatedAt: now(),
      completedAt: null,
    };
    await invoke("create_task", { task });
    return task;
  }

  async update(id: string, data: UpdateTaskInput): Promise<Task> {
    const updated = await invoke<Task>("update_task", { id, data: { ...data, updatedAt: now() } });
    return updated;
  }

  async delete(id: string): Promise<void> {
    await invoke("delete_task", { id });
  }

  async getById(id: string): Promise<Task | null> {
    return await invoke<Task | null>("get_task", { id });
  }

  async list(filter?: TaskFilter): Promise<Task[]> {
    return await invoke<Task[]>("list_tasks", { filter: filter || {} });
  }

  async addSubtask(taskId: string, title: string): Promise<Subtask> {
    const subtask: Subtask = {
      id: generateId(),
      title,
      isDone: false,
      sortOrder: 0,
    };
    await invoke("add_subtask", { taskId, subtask });
    return subtask;
  }

  async toggleSubtask(taskId: string, subtaskId: string): Promise<void> {
    await invoke("toggle_subtask", { taskId, subtaskId });
  }

  async deleteSubtask(taskId: string, subtaskId: string): Promise<void> {
    await invoke("delete_subtask", { taskId, subtaskId });
  }
}
