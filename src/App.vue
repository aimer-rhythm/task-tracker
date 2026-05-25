<template>
  <div id="app-shell" :class="theme">
    <header class="titlebar">
      <div class="titlebar-left">
        <span class="app-title">TaskTrack</span>
      </div>
      <div class="titlebar-controls">
        <button class="btn-icon" title="最小化" @click="minimizeWindow">
          <span class="material-symbols-outlined">remove</span>
        </button>
        <div class="settings-wrapper">
          <button class="btn-icon" title="设置" @click="toggleSettings">
            <span class="material-symbols-outlined">settings</span>
          </button>
          <div v-if="showSettings" class="settings-menu">
            <button
              :class="['btn-icon', { active: settingsStore.alwaysOnTop }]"
              title="置顶"
              @click="togglePin"
            >
              <span class="material-symbols-outlined" :style="settingsStore.alwaysOnTop ? 'font-variation-settings: \'FILL\' 1' : ''">push_pin</span>
            </button>
            <div class="settings-divider"></div>
            <button class="btn-icon" title="主题切换" @click="toggleTheme">
              <span class="material-symbols-outlined">{{ theme === 'dark' ? 'dark_mode' : 'light_mode' }}</span>
            </button>
            <div class="settings-divider"></div>
            <div class="opacity-control">
              <span class="material-symbols-outlined">opacity</span>
              <div class="opacity-track">
                <div class="opacity-fill" :style="{ width: opacity + '%' }"></div>
                <input
                  type="range"
                  min="30"
                  max="100"
                  :value="opacity"
                  class="opacity-slider"
                  @input="setOpacity"
                />
              </div>
            </div>
          </div>
        </div>
        <button class="btn-icon" title="关闭" @click="closeWindow">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
    </header>
    <nav class="tab-bar">
      <button
        :class="['tab', { active: activeTab === 'tasks' }]"
        @click="activeTab = 'tasks'"
      >
        <span class="material-symbols-outlined">check_circle</span>
        <span class="tab-label">任务</span>
      </button>
      <button
        :class="['tab', { active: activeTab === 'reminders' }]"
        @click="activeTab = 'reminders'"
      >
        <span class="material-symbols-outlined">notifications</span>
        <span class="tab-label">提醒</span>
      </button>
    </nav>
    <main class="content">
      <TaskList v-if="activeTab === 'tasks'" />
      <ReminderList v-else />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import TaskList from "./views/TaskList.vue";
import ReminderList from "./views/ReminderList.vue";
import { useSettingsStore } from "./stores/settings";

const settingsStore = useSettingsStore();
const activeTab = ref<"tasks" | "reminders">("tasks");
const theme = ref(settingsStore.theme);
const opacity = ref(settingsStore.opacity);
const showSettings = ref(false);

function toggleSettings() {
  showSettings.value = !showSettings.value;
}

function closeSettingsOnOutsideClick(e: MouseEvent) {
  const wrapper = document.querySelector('.settings-wrapper');
  if (wrapper && !wrapper.contains(e.target as Node)) {
    showSettings.value = false;
  }
}

let saveTimer: ReturnType<typeof setTimeout> | null = null;
const unlisteners: Array<() => void> = [];

function saveWindowState(props: Record<string, string>) {
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    for (const [key, value] of Object.entries(props)) {
      invoke("set_setting", { key, value }).catch(() => {});
    }
  }, 500);
}

async function setupWindowListeners() {
  const win = getCurrentWindow();
  const unlisten1 = await win.onMoved(({ payload }) => {
    saveWindowState({ window_x: String(payload.x), window_y: String(payload.y) });
  });
  const unlisten2 = await win.onResized(({ payload }) => {
    saveWindowState({ window_width: String(payload.width), window_height: String(payload.height) });
  });
  unlisteners.push(unlisten1, unlisten2);
}

onMounted(() => {
  document.addEventListener('click', closeSettingsOnOutsideClick);
  setupWindowListeners();
});

onUnmounted(() => {
  document.removeEventListener('click', closeSettingsOnOutsideClick);
  unlisteners.forEach((fn) => fn());
});

function togglePin() {
  settingsStore.toggleAlwaysOnTop();
}

function toggleTheme() {
  theme.value = theme.value === "light" ? "dark" : "light";
  settingsStore.setTheme(theme.value);
}

function setOpacity(e: Event) {
  const val = Number((e.target as HTMLInputElement).value);
  opacity.value = val;
  settingsStore.setOpacity(val);
}

function minimizeWindow() {
  getCurrentWindow().minimize();
}

function closeWindow() {
  settingsStore.minimizeToTray();
}
</script>
