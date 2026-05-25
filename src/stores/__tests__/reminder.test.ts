import { describe, it, expect, vi, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useReminderStore } from "../reminder";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("useReminderStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("should start with empty state", () => {
    const store = useReminderStore();
    expect(store.reminders).toEqual([]);
    expect(store.loading).toBe(false);
  });

  describe("fetchReminders", () => {
    it("should load reminders", async () => {
      const reminders = [{ id: "r1", title: "Water", isActive: true }];
      mockedInvoke.mockResolvedValueOnce(reminders);

      const store = useReminderStore();
      await store.fetchReminders();

      expect(store.reminders).toEqual(reminders);
      expect(store.loading).toBe(false);
    });
  });

  describe("createReminder", () => {
    it("should add reminder to beginning of list", async () => {
      mockedInvoke.mockResolvedValueOnce(undefined);

      const store = useReminderStore();
      const reminder = await store.createReminder({
        title: "Stand up",
        type: "recurring",
        intervalSeconds: 1800,
      });

      expect(reminder.title).toBe("Stand up");
      expect(store.reminders[0]).toEqual(reminder);
    });
  });

  describe("deleteReminder", () => {
    it("should remove reminder from list", async () => {
      mockedInvoke.mockResolvedValueOnce([{ id: "r1", title: "Test", isActive: true }]);

      const store = useReminderStore();
      await store.fetchReminders();

      mockedInvoke.mockResolvedValueOnce(undefined);
      await store.deleteReminder("r1");

      expect(store.reminders).toEqual([]);
    });
  });

  describe("toggleReminder", () => {
    it("should toggle isActive", async () => {
      mockedInvoke.mockResolvedValueOnce([{ id: "r1", title: "Test", isActive: true }]);

      const store = useReminderStore();
      await store.fetchReminders();

      mockedInvoke.mockResolvedValueOnce(undefined);
      await store.toggleReminder("r1");

      expect(store.reminders[0].isActive).toBe(false);
    });
  });
});
