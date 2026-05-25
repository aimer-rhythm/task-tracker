import { describe, it, expect, vi, beforeEach } from "vitest";
import { LocalTaskRepository } from "../task";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("LocalTaskRepository", () => {
  let repo: LocalTaskRepository;

  beforeEach(() => {
    repo = new LocalTaskRepository();
    vi.clearAllMocks();
  });

  describe("create", () => {
    it("should create a task with defaults", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const task = await repo.create({ title: "Test task" });

      expect(task.title).toBe("Test task");
      expect(task.status).toBe("todo");
      expect(task.priority).toBe("medium");
      expect(task.progress).toBe(0);
      expect(task.description).toBe("");
      expect(task.subtasks).toEqual([]);
      expect(task.id).toBeTruthy();
      expect(task.createdAt).toBeTruthy();
      expect(mockedInvoke).toHaveBeenCalledWith("create_task", { task });
    });

    it("should use provided priority and dueDate", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const task = await repo.create({
        title: "Urgent",
        priority: "high",
        dueDate: "2025-12-31",
        description: "Important",
      });

      expect(task.priority).toBe("high");
      expect(task.dueDate).toBe("2025-12-31");
      expect(task.description).toBe("Important");
    });
  });

  describe("update", () => {
    it("should call invoke with update data", async () => {
      const updated = { id: "t1", title: "Updated", status: "done" };
      mockedInvoke.mockResolvedValueOnce(updated);

      const result = await repo.update("t1", { title: "Updated" });

      expect(mockedInvoke).toHaveBeenCalledWith("update_task", {
        id: "t1",
        data: expect.objectContaining({ title: "Updated", updatedAt: expect.any(String) }),
      });
      expect(result).toEqual(updated);
    });
  });

  describe("delete", () => {
    it("should call invoke with correct id", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      await repo.delete("t1");

      expect(mockedInvoke).toHaveBeenCalledWith("delete_task", { id: "t1" });
    });
  });

  describe("getById", () => {
    it("should return task when found", async () => {
      const task = { id: "t1", title: "Found" };
      mockedInvoke.mockResolvedValueOnce(task);

      const result = await repo.getById("t1");

      expect(result).toEqual(task);
      expect(mockedInvoke).toHaveBeenCalledWith("get_task", { id: "t1" });
    });

    it("should return null when not found", async () => {
      mockedInvoke.mockResolvedValueOnce(null);

      const result = await repo.getById("nonexistent");

      expect(result).toBeNull();
    });
  });

  describe("list", () => {
    it("should list tasks with empty filter", async () => {
      mockedInvoke.mockResolvedValueOnce([]);

      const result = await repo.list();

      expect(mockedInvoke).toHaveBeenCalledWith("list_tasks", { filter: {} });
      expect(result).toEqual([]);
    });

    it("should pass filter to invoke", async () => {
      mockedInvoke.mockResolvedValueOnce([]);

      await repo.list({ status: "todo", priority: "high" });

      expect(mockedInvoke).toHaveBeenCalledWith("list_tasks", {
        filter: { status: "todo", priority: "high" },
      });
    });
  });

  describe("subtasks", () => {
    it("should add a subtask", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const subtask = await repo.addSubtask("t1", "Sub item");

      expect(subtask.title).toBe("Sub item");
      expect(subtask.isDone).toBe(false);
      expect(subtask.id).toBeTruthy();
      expect(mockedInvoke).toHaveBeenCalledWith("add_subtask", {
        taskId: "t1",
        subtask,
      });
    });

    it("should toggle a subtask", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      await repo.toggleSubtask("t1", "s1");

      expect(mockedInvoke).toHaveBeenCalledWith("toggle_subtask", {
        taskId: "t1",
        subtaskId: "s1",
      });
    });

    it("should delete a subtask", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      await repo.deleteSubtask("t1", "s1");

      expect(mockedInvoke).toHaveBeenCalledWith("delete_subtask", {
        taskId: "t1",
        subtaskId: "s1",
      });
    });
  });
});
