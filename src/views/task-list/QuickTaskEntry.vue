<template>
  <div class="quick-entry">
    <input
      :value="title"
      placeholder="+ 新任务..."
      maxlength="200"
      @input="emit('updateTitle', ($event.target as HTMLInputElement).value)"
      @keydown.enter="emit('create')"
    />
    <div class="priority-toggle-wrapper">
      <button class="priority-icon-btn" @click.stop="emit('togglePriorityMenu')">
        <span
          class="material-symbols-outlined"
          :class="priorityColorClass(priority)"
          style="font-variation-settings: 'FILL' 1"
        >flag</span>
      </button>
      <PriorityMenu
        v-if="priorityMenuOpen"
        :value="priority"
        @select="emit('selectPriority', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Priority } from "../../repositories/types";
import PriorityMenu from "./PriorityMenu.vue";

defineProps<{
  title: string;
  priority: Priority;
  priorityMenuOpen: boolean;
}>();

const emit = defineEmits<{
  updateTitle: [title: string];
  create: [];
  togglePriorityMenu: [];
  selectPriority: [priority: Priority];
}>();

function priorityColorClass(priority: Priority): string {
  if (priority === "high") return "priority-high-color";
  if (priority === "low") return "priority-low-color";
  return "priority-medium-color";
}
</script>
