<template>
  <ul class="subtask-list">
    <li
      v-for="subtask in subtasks"
      :key="subtask.id"
      :class="['subtask-item', { done: subtask.isDone }]"
    >
      <span
        class="material-symbols-outlined"
        :class="subtask.isDone ? '' : 'unchecked'"
        :style="subtask.isDone ? 'font-variation-settings: \'FILL\' 1' : ''"
        @click="emit('toggle', subtask.id)"
      >{{ subtask.isDone ? 'check_box' : 'check_box_outline_blank' }}</span>
      <span>{{ subtask.title }}</span>
    </li>
  </ul>
</template>

<script setup lang="ts">
import type { Subtask } from "../../repositories/types";

defineProps<{
  subtasks: Subtask[];
}>();

const emit = defineEmits<{
  toggle: [subtaskId: string];
}>();
</script>
