import { describe, it, expect, vi, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useSettingsStore } from "../settings";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("useSettingsStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("should have correct defaults", () => {
    const store = useSettingsStore();
    expect(store.theme).toBe("light");
    expect(store.opacity).toBe(100);
    expect(store.alwaysOnTop).toBe(false);
  });

  describe("setTheme", () => {
    it("should update theme and persist", () => {
      mockedInvoke.mockResolvedValue(undefined);

      const store = useSettingsStore();
      store.setTheme("dark");

      expect(store.theme).toBe("dark");
      expect(mockedInvoke).toHaveBeenCalledWith("set_setting", { key: "theme", value: "dark" });
    });
  });

  describe("setOpacity", () => {
    it("should update opacity and call window command", () => {
      mockedInvoke.mockResolvedValue(undefined);

      const store = useSettingsStore();
      store.setOpacity(75);

      expect(store.opacity).toBe(75);
      expect(mockedInvoke).toHaveBeenCalledWith("set_window_opacity", { opacity: 0.75 });
    });
  });

  describe("toggleAlwaysOnTop", () => {
    it("should toggle and call command with current opacity", () => {
      mockedInvoke.mockResolvedValue(undefined);

      const store = useSettingsStore();
      store.toggleAlwaysOnTop();

      expect(store.alwaysOnTop).toBe(true);
      expect(mockedInvoke).toHaveBeenCalledWith("set_always_on_top", {
        enabled: true,
        currentOpacity: 1,
      });
    });
  });

  describe("loadSettings", () => {
    it("should load saved settings from backend", async () => {
      mockedInvoke.mockResolvedValueOnce({
        theme: "dark",
        opacity: "80",
        always_on_top: "true",
      });

      const store = useSettingsStore();
      await store.loadSettings();

      expect(store.theme).toBe("dark");
      expect(store.opacity).toBe(80);
      expect(store.alwaysOnTop).toBe(true);
    });

    it("should keep defaults on error", async () => {
      mockedInvoke.mockRejectedValueOnce(new Error("no db"));

      const store = useSettingsStore();
      await store.loadSettings();

      expect(store.theme).toBe("light");
      expect(store.opacity).toBe(100);
      expect(store.alwaysOnTop).toBe(false);
    });
  });

  describe("minimizeToTray", () => {
    it("should call minimize command", () => {
      mockedInvoke.mockResolvedValue(undefined);

      const store = useSettingsStore();
      store.minimizeToTray();

      expect(mockedInvoke).toHaveBeenCalledWith("minimize_to_tray");
    });
  });
});
