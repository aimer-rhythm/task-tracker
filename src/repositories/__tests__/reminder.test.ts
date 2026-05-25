import { describe, it, expect, vi, beforeEach } from "vitest";
import { LocalReminderRepository } from "../reminder";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("LocalReminderRepository", () => {
  let repo: LocalReminderRepository;

  beforeEach(() => {
    repo = new LocalReminderRepository();
    vi.clearAllMocks();
  });

  describe("create", () => {
    it("should create a one-time reminder with triggerAt", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const reminder = await repo.create({
        title: "Meeting",
        type: "once",
        triggerAt: "2025-06-01T10:00:00Z",
      });

      expect(reminder.title).toBe("Meeting");
      expect(reminder.type).toBe("once");
      expect(reminder.nextTriggerAt).toBe("2025-06-01T10:00:00Z");
      expect(reminder.isActive).toBe(true);
      expect(reminder.id).toBeTruthy();
    });

    it("should create a recurring reminder with interval", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const reminder = await repo.create({
        title: "Drink water",
        type: "recurring",
        intervalSeconds: 3600,
      });

      expect(reminder.type).toBe("recurring");
      expect(reminder.intervalSeconds).toBe(3600);
      expect(reminder.nextTriggerAt).toBeTruthy();
    });

    it("should default interval to 300s when not provided for recurring", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const reminder = await repo.create({
        title: "Stretch",
        type: "recurring",
      });

      const expectedTime = new Date(Date.now() + 300 * 1000);
      const actualTime = new Date(reminder.nextTriggerAt);
      expect(Math.abs(actualTime.getTime() - expectedTime.getTime())).toBeLessThan(1000);
    });
  });

  describe("delete", () => {
    it("should call invoke with correct id", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      await repo.delete("r1");

      expect(mockedInvoke).toHaveBeenCalledWith("delete_reminder", { id: "r1" });
    });
  });

  describe("listActive", () => {
    it("should call with activeOnly true", async () => {
      mockedInvoke.mockResolvedValueOnce([]);

      await repo.listActive();

      expect(mockedInvoke).toHaveBeenCalledWith("list_reminders", { activeOnly: true });
    });
  });

  describe("listAll", () => {
    it("should call with activeOnly false", async () => {
      mockedInvoke.mockResolvedValueOnce([]);

      await repo.listAll();

      expect(mockedInvoke).toHaveBeenCalledWith("list_reminders", { activeOnly: false });
    });
  });

  describe("toggle", () => {
    it("should call invoke with correct id", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      await repo.toggle("r1");

      expect(mockedInvoke).toHaveBeenCalledWith("toggle_reminder", { id: "r1" });
    });
  });
});
