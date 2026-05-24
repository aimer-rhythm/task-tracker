import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const useSettingsStore = defineStore("settings", () => {
  const theme = ref<"light" | "dark">("light");
  const opacity = ref(100);
  const alwaysOnTop = ref(false);

  function setTheme(t: "light" | "dark") {
    theme.value = t;
    invoke("set_setting", { key: "theme", value: t }).catch(() => {});
    reapplyOpacity();
  }

  function setOpacity(val: number) {
    opacity.value = val;
    invoke("set_window_opacity", { opacity: val / 100 }).catch(() => {});
  }

  function reapplyOpacity() {
    if (opacity.value < 100) {
      invoke("set_window_opacity", { opacity: opacity.value / 100 }).catch(() => {});
    }
  }

  function toggleAlwaysOnTop() {
    alwaysOnTop.value = !alwaysOnTop.value;
    invoke("set_always_on_top", {
      enabled: alwaysOnTop.value,
      currentOpacity: opacity.value / 100,
    }).catch(() => {});
  }

  function minimizeToTray() {
    invoke("minimize_to_tray").catch(() => {});
  }

  async function loadSettings() {
    try {
      const saved = await invoke<Record<string, string>>("get_all_settings");
      if (saved.theme) theme.value = saved.theme as "light" | "dark";
      if (saved.opacity) opacity.value = Number(saved.opacity);
      if (saved.always_on_top) alwaysOnTop.value = saved.always_on_top === "true";
    } catch {
      // defaults are fine
    }
  }

  return {
    theme,
    opacity,
    alwaysOnTop,
    setTheme,
    setOpacity,
    toggleAlwaysOnTop,
    minimizeToTray,
    loadSettings,
  };
});
