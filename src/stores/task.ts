import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Task, TaskStatus, CreateTaskInput, UpdateTaskInput, TaskFilter } from "../repositories/types";
import { LocalTaskRepository } from "../repositories/task";

const repo = new LocalTaskRepository();

export const useTaskStore = defineStore("task", () => {
  const tasks = ref<Task[]>([]);
  const loading = ref(false);
  const expandedId = ref<string | null>(null);

  const activeTasks = computed(() => tasks.value.filter((t) => t.status !== "done"));
  const completedTasks = computed(() => tasks.value.filter((t) => t.status === "done"));

  async function fetchTasks(filter?: TaskFilter) {
    loading.value = true;
    try {
      tasks.value = await repo.list(filter);
    } finally {
      loading.value = false;
    }
  }

  async function createTask(input: CreateTaskInput) {
    const task = await repo.create(input);
    tasks.value.unshift(task);
    return task;
  }

  async function updateTask(id: string, data: UpdateTaskInput) {
    const updated = await repo.update(id, data);
    const idx = tasks.value.findIndex((t) => t.id === id);
    if (idx !== -1) tasks.value[idx] = updated;
    return updated;
  }

  async function deleteTask(id: string) {
    await repo.delete(id);
    tasks.value = tasks.value.filter((t) => t.id !== id);
  }

  async function toggleStatus(id: string) {
    const task = tasks.value.find((t) => t.id === id);
    if (!task) return;
    const newStatus: TaskStatus = task.status === "done" ? "todo" : "done";
    await updateTask(id, { status: newStatus });
  }

  async function addSubtask(taskId: string, title: string) {
    const subtask = await repo.addSubtask(taskId, title);
    const task = tasks.value.find((t) => t.id === taskId);
    if (task) task.subtasks.push(subtask);
  }

  async function toggleSubtask(taskId: string, subtaskId: string) {
    await repo.toggleSubtask(taskId, subtaskId);
    const task = tasks.value.find((t) => t.id === taskId);
    if (task) {
      const sub = task.subtasks.find((s) => s.id === subtaskId);
      if (sub) sub.isDone = !sub.isDone;
    }
  }

  function toggleExpand(id: string) {
    expandedId.value = expandedId.value === id ? null : id;
  }

  return {
    tasks,
    loading,
    expandedId,
    activeTasks,
    completedTasks,
    fetchTasks,
    createTask,
    updateTask,
    deleteTask,
    toggleStatus,
    addSubtask,
    toggleSubtask,
    toggleExpand,
  };
});
