<template>
  <div v-if="compact" class="task-card-progress">
    <span class="task-card-progress-label">{{ completed }}/{{ total }}</span>
    <div class="progress-bar compact">
      <div class="progress-fill" :style="{ width: progress + '%' }"></div>
    </div>
  </div>

  <template v-else>
    <div class="progress-header">
      <span>子任务 ({{ completed }}/{{ total }})</span>
      <span class="progress-percent">{{ progress }}%</span>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" :style="{ width: progress + '%' }"></div>
    </div>
  </template>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    completed: number;
    total: number;
    progress: number;
    compact?: boolean;
  }>(),
  {
    compact: false,
  },
);
</script>
