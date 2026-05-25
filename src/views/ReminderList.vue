<template>
  <div class="reminder-list-view">
    <!-- Quick Entry -->
    <div class="quick-entry">
      <input
        v-model="newTitle"
        placeholder="+ 新提醒..."
        @keydown.enter="showForm = true"
      />
    </div>
    <div v-if="showForm" class="reminder-form">
      <div class="form-row">
        <select v-model="newType" class="form-select">
          <option value="recurring">循环</option>
          <option value="once">单次</option>
        </select>
        <input
          v-if="newType === 'recurring'"
          v-model.number="newInterval"
          type="number"
          min="1"
          placeholder="分钟"
          class="form-number"
        />
        <span v-if="newType === 'recurring'" class="form-unit">分钟</span>
        <button class="form-btn-primary" @click="addReminder">添加</button>
        <button class="form-btn-secondary" @click="showForm = false">取消</button>
      </div>
    </div>

    <!-- Reminder List -->
    <div class="task-list">
      <div v-for="reminder in reminderStore.reminders" :key="reminder.id" class="reminder-card">
        <div class="reminder-info">
          <div class="reminder-title">{{ reminder.title }}</div>
          <div class="reminder-interval">
            {{ reminder.type === 'recurring' ? `每 ${formatInterval(reminder.intervalSeconds)}` : '单次' }}
          </div>
        </div>
        <div
          :class="['toggle-switch', { active: reminder.isActive }]"
          @click="reminderStore.toggleReminder(reminder.id)"
        ></div>
        <button class="btn-icon" @click="reminderStore.deleteReminder(reminder.id)">
          <span class="material-symbols-outlined" style="font-size:16px;">delete</span>
        </button>
      </div>
    </div>

    <div v-if="!reminderStore.reminders.length" class="empty-state">
      <span class="material-symbols-outlined" style="font-size:32px;color:var(--on-surface-variant);opacity:0.5;">notifications_off</span>
      <span>暂无提醒</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useReminderStore } from "../stores/reminder";

const reminderStore = useReminderStore();
const newTitle = ref("");
const newType = ref<"recurring" | "once">("recurring");
const newInterval = ref(5);
const showForm = ref(false);

onMounted(() => {
  reminderStore.fetchReminders();
});

function formatInterval(seconds: number | null): string {
  if (!seconds) return "";
  if (seconds < 60) return `${seconds}秒`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}分钟`;
  return `${Math.round(seconds / 3600)}小时`;
}

async function addReminder() {
  const title = newTitle.value.trim();
  if (!title) return;
  await reminderStore.createReminder({
    title,
    type: newType.value,
    intervalSeconds: newType.value === "recurring" ? newInterval.value * 60 : undefined,
  });
  newTitle.value = "";
  showForm.value = false;
}
</script>

<style scoped>
.reminder-form {
  margin-bottom: var(--spacing-md);
}

.form-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.form-select {
  font-size: 12px;
  padding: 4px 8px;
  border-radius: var(--radius-lg);
  border: 1px solid rgba(194, 198, 214, 0.2);
  background: var(--surface-container);
  color: var(--on-surface);
  outline: none;
}

.form-select:focus {
  border-color: var(--primary);
}

.form-number {
  width: 60px;
  font-size: 12px;
  padding: 4px 8px;
  border-radius: var(--radius-lg);
  border: 1px solid rgba(194, 198, 214, 0.2);
  background: var(--surface-container);
  color: var(--on-surface);
  outline: none;
  font-family: var(--font-family);
}

.form-unit {
  font-size: 11px;
  color: var(--on-surface-variant);
}

.form-btn-primary {
  font-size: 11px;
  font-weight: 600;
  padding: 4px 12px;
  border-radius: var(--radius-lg);
  border: none;
  background: var(--primary);
  color: white;
  cursor: pointer;
  transition: opacity 0.15s;
}

.form-btn-primary:hover {
  opacity: 0.9;
}

.form-btn-secondary {
  font-size: 11px;
  padding: 4px 8px;
  border-radius: var(--radius-lg);
  border: 1px solid rgba(194, 198, 214, 0.2);
  background: transparent;
  color: var(--on-surface-variant);
  cursor: pointer;
  transition: background 0.15s;
}

.form-btn-secondary:hover {
  background: var(--surface-container-high);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 24px;
  font-size: 12px;
  color: var(--on-surface-variant);
}
</style>
