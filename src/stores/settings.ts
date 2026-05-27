import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

function invokeSilently(command: string, args?: Record<string, unknown>) {
  return invoke(command, args).catch((error) => {
    console.warn(`[settings] ${command} failed`, error);
  });
}

export const useSettingsStore = defineStore("settings", () => {
  const theme = ref<"light" | "dark">("light");
  const opacity = ref(100);
  const alwaysOnTop = ref(false);

  function setTheme(t: "light" | "dark") {
    theme.value = t;
    invokeSilently("set_setting", { key: "theme", value: t });
    reapplyOpacity();
  }

  function setOpacity(val: number) {
    opacity.value = val;
    invokeSilently("set_window_opacity", { opacity: val / 100 });
    invokeSilently("set_setting", { key: "opacity", value: String(val) });
  }

  function reapplyOpacity() {
    if (opacity.value < 100) {
      invokeSilently("set_window_opacity", { opacity: opacity.value / 100 });
    }
  }

  function toggleAlwaysOnTop() {
    alwaysOnTop.value = !alwaysOnTop.value;
    invokeSilently("set_always_on_top", {
      enabled: alwaysOnTop.value,
      currentOpacity: opacity.value / 100,
    });
    invokeSilently("set_setting", { key: "always_on_top", value: String(alwaysOnTop.value) });
  }

  function minimizeToTray() {
    invokeSilently("minimize_to_tray");
  }

  async function loadSettings() {
    try {
      const saved = await invoke<Record<string, string>>("get_all_settings");
      if (saved.theme) theme.value = saved.theme as "light" | "dark";
      if (saved.opacity) opacity.value = Number(saved.opacity);
      if (saved.always_on_top) alwaysOnTop.value = saved.always_on_top === "true";

      if (opacity.value < 100) {
        invokeSilently("set_window_opacity", { opacity: opacity.value / 100 });
      }
      if (alwaysOnTop.value) {
        invokeSilently("set_always_on_top", {
          enabled: true,
          currentOpacity: opacity.value / 100,
        });
      }
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
