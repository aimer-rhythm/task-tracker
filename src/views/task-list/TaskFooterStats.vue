<template>
  <div v-if="totalCount > 0" class="footer-stats">
    <span>{{ activeCount }} 个待办任务</span>
    <button v-if="completedCount > 0" @click="emit('clearCompleted')">清除已完成</button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  totalCount: number;
  activeCount: number;
  completedCount: number;
}>();

const emit = defineEmits<{
  clearCompleted: [];
}>();
</script>
