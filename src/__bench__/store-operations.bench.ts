import { bench, describe } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useTaskStore } from "../stores/task";
import { invoke } from "@tauri-apps/api/core";
import { vi } from "vitest";
import type { Task } from "../repositories/types";

vi.mock("@tauri-apps/api/core");
const mockedInvoke = vi.mocked(invoke);

function makeTasks(count: number): Task[] {
  return Array.from({ length: count }, (_, i) => ({
    id: `t${i}`,
    title: `Task ${i}`,
    description: "",
    status: i % 3 === 0 ? "done" : "todo",
    priority: i % 2 === 0 ? "high" : "medium",
    progress: Math.floor(Math.random() * 100),
    category: "",
    tags: [],
    subtasks: Array.from({ length: i % 5 }, (_, j) => ({
      id: `s${i}-${j}`,
      title: `Subtask ${j}`,
      isDone: j % 2 === 0,
      sortOrder: j,
    })),
    dueDate: null,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    completedAt: null,
  })) as Task[];
}

describe("TaskStore — fetchTasks", () => {
  bench("load 10 tasks", async () => {
    setActivePinia(createPinia());
    mockedInvoke.mockResolvedValueOnce(makeTasks(10));
    const store = useTaskStore();
    await store.fetchTasks();
  });

  bench("load 100 tasks", async () => {
    setActivePinia(createPinia());
    mockedInvoke.mockResolvedValueOnce(makeTasks(100));
    const store = useTaskStore();
    await store.fetchTasks();
  });

  bench("load 500 tasks", async () => {
    setActivePinia(createPinia());
    mockedInvoke.mockResolvedValueOnce(makeTasks(500));
    const store = useTaskStore();
    await store.fetchTasks();
  });
});

describe("TaskStore — computed filters", () => {
  bench("activeTasks filter (100 items)", () => {
    setActivePinia(createPinia());
    const store = useTaskStore();
    store.tasks = makeTasks(100);
    void store.activeTasks;
  });

  bench("completedTasks filter (100 items)", () => {
    setActivePinia(createPinia());
    const store = useTaskStore();
    store.tasks = makeTasks(100);
    void store.completedTasks;
  });

  bench("activeTasks filter (500 items)", () => {
    setActivePinia(createPinia());
    const store = useTaskStore();
    store.tasks = makeTasks(500);
    void store.activeTasks;
  });
});

describe("TaskStore — mutations", () => {
  bench("createTask", async () => {
    setActivePinia(createPinia());
    mockedInvoke.mockResolvedValue(undefined);
    const store = useTaskStore();
    await store.createTask({ title: "Bench task", priority: "high" });
  });

  bench("toggleExpand", () => {
    setActivePinia(createPinia());
    const store = useTaskStore();
    store.tasks = makeTasks(50);
    store.toggleExpand("t5");
  });

  bench("deleteTask from 100 items", async () => {
    setActivePinia(createPinia());
    mockedInvoke.mockResolvedValue(undefined);
    const store = useTaskStore();
    store.tasks = makeTasks(100);
    await store.deleteTask("t50");
  });
});
