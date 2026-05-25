import { describe, it, expect, vi, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useTaskStore } from "../task";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("useTaskStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("should start with empty state", () => {
    const store = useTaskStore();
    expect(store.tasks).toEqual([]);
    expect(store.loading).toBe(false);
    expect(store.expandedId).toBeNull();
  });

  describe("fetchTasks", () => {
    it("should load tasks from backend", async () => {
      const tasks = [
        { id: "t1", title: "Task 1", status: "todo" },
        { id: "t2", title: "Task 2", status: "done" },
      ];
      mockedInvoke.mockResolvedValueOnce(tasks);

      const store = useTaskStore();
      await store.fetchTasks();

      expect(store.tasks).toEqual(tasks);
      expect(store.loading).toBe(false);
    });

    it("should set loading false even on error", async () => {
      mockedInvoke.mockRejectedValueOnce(new Error("fail"));

      const store = useTaskStore();
      await expect(store.fetchTasks()).rejects.toThrow("fail");
      expect(store.loading).toBe(false);
    });
  });

  describe("createTask", () => {
    it("should add task to beginning of list", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const store = useTaskStore();
      const task = await store.createTask({ title: "New task" });

      expect(task.title).toBe("New task");
      expect(store.tasks[0]).toEqual(task);
    });
  });

  describe("deleteTask", () => {
    it("should remove task from list", async () => {
      mockedInvoke.mockResolvedValueOnce([{ id: "t1", title: "A", status: "todo", subtasks: [] }]);

      const store = useTaskStore();
      await store.fetchTasks();

      mockedInvoke.mockResolvedValueOnce(undefined);
      await store.deleteTask("t1");

      expect(store.tasks).toEqual([]);
    });
  });

  describe("toggleStatus", () => {
    it("should toggle todo to done", async () => {
      const tasks = [{ id: "t1", title: "A", status: "todo", subtasks: [] }];
      mockedInvoke.mockResolvedValueOnce(tasks);

      const store = useTaskStore();
      await store.fetchTasks();

      const updated = { id: "t1", title: "A", status: "done", subtasks: [] };
      mockedInvoke.mockResolvedValueOnce(updated);
      await store.toggleStatus("t1");

      expect(store.tasks[0].status).toBe("done");
    });

    it("should toggle done to todo", async () => {
      const tasks = [{ id: "t1", title: "A", status: "done", subtasks: [] }];
      mockedInvoke.mockResolvedValueOnce(tasks);

      const store = useTaskStore();
      await store.fetchTasks();

      const updated = { id: "t1", title: "A", status: "todo", subtasks: [] };
      mockedInvoke.mockResolvedValueOnce(updated);
      await store.toggleStatus("t1");

      expect(store.tasks[0].status).toBe("todo");
    });
  });

  describe("computed properties", () => {
    it("should filter active and completed tasks", async () => {
      const tasks = [
        { id: "t1", title: "A", status: "todo", subtasks: [] },
        { id: "t2", title: "B", status: "done", subtasks: [] },
        { id: "t3", title: "C", status: "in_progress", subtasks: [] },
      ];
      mockedInvoke.mockResolvedValueOnce(tasks);

      const store = useTaskStore();
      await store.fetchTasks();

      expect(store.activeTasks).toHaveLength(2);
      expect(store.completedTasks).toHaveLength(1);
    });
  });

  describe("toggleExpand", () => {
    it("should expand and collapse", () => {
      const store = useTaskStore();

      store.toggleExpand("t1");
      expect(store.expandedId).toBe("t1");

      store.toggleExpand("t1");
      expect(store.expandedId).toBeNull();

      store.toggleExpand("t1");
      store.toggleExpand("t2");
      expect(store.expandedId).toBe("t2");
    });
  });

  describe("addSubtask", () => {
    it("should add subtask to task", async () => {
      const tasks = [{ id: "t1", title: "A", status: "todo", subtasks: [] }];
      mockedInvoke.mockResolvedValueOnce(tasks);

      const store = useTaskStore();
      await store.fetchTasks();

      mockedInvoke.mockResolvedValueOnce(undefined);
      await store.addSubtask("t1", "Sub item");

      expect(store.tasks[0].subtasks).toHaveLength(1);
      expect(store.tasks[0].subtasks[0].title).toBe("Sub item");
    });
  });

  describe("toggleSubtask", () => {
    it("should toggle subtask isDone", async () => {
      const tasks = [
        { id: "t1", title: "A", status: "todo", subtasks: [{ id: "s1", title: "Sub", isDone: false, sortOrder: 0 }] },
      ];
      mockedInvoke.mockResolvedValueOnce(tasks);

      const store = useTaskStore();
      await store.fetchTasks();

      mockedInvoke.mockResolvedValueOnce(undefined);
      await store.toggleSubtask("t1", "s1");

      expect(store.tasks[0].subtasks[0].isDone).toBe(true);
    });
  });
});
