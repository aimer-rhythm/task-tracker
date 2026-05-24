import { defineStore } from "pinia";
import { ref } from "vue";
import type { Reminder, CreateReminderInput } from "../repositories/types";
import { LocalReminderRepository } from "../repositories/reminder";

const repo = new LocalReminderRepository();

export const useReminderStore = defineStore("reminder", () => {
  const reminders = ref<Reminder[]>([]);
  const loading = ref(false);

  async function fetchReminders() {
    loading.value = true;
    try {
      reminders.value = await repo.listAll();
    } finally {
      loading.value = false;
    }
  }

  async function createReminder(input: CreateReminderInput) {
    const reminder = await repo.create(input);
    reminders.value.unshift(reminder);
    return reminder;
  }

  async function deleteReminder(id: string) {
    await repo.delete(id);
    reminders.value = reminders.value.filter((r) => r.id !== id);
  }

  async function toggleReminder(id: string) {
    await repo.toggle(id);
    const r = reminders.value.find((rem) => rem.id === id);
    if (r) r.isActive = !r.isActive;
  }

  return {
    reminders,
    loading,
    fetchReminders,
    createReminder,
    deleteReminder,
    toggleReminder,
  };
});
