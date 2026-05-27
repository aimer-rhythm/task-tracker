<template>
  <div class="priority-menu">
    <button
      v-for="option in PRIORITY_OPTIONS"
      :key="option.value"
      :class="['priority-menu-item', { selected: value === option.value }]"
      @click.stop="emit('select', option.value)"
    >
      <span
        class="material-symbols-outlined"
        :class="option.colorClass"
        style="font-variation-settings: 'FILL' 1"
      >flag</span>
      <span>{{ option.label }}</span>
      <span v-if="value === option.value" class="material-symbols-outlined check-icon">check</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { Priority } from "../../repositories/types";

type PriorityOption = {
  value: Priority;
  label: string;
  colorClass: string;
};

defineProps<{
  value: Priority;
}>();

const emit = defineEmits<{
  select: [priority: Priority];
}>();

const PRIORITY_OPTIONS: PriorityOption[] = [
  { value: "high", label: "高", colorClass: "priority-high-color" },
  { value: "medium", label: "中", colorClass: "priority-medium-color" },
  { value: "low", label: "低", colorClass: "priority-low-color" },
];
</script>
