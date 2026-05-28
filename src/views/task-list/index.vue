<template>
  <div class="task-list-view" :class="{ 'focus-task-list-view': focusMode }">
    <QuickTaskEntry
      v-if="!focusMode"
      :title="newTaskTitle"
      :priority="newTaskPriority"
      :priority-menu-open="activePriorityMenu === 'new'"
      @update-title="newTaskTitle = $event"
      @create="addTask"
      @toggle-priority-menu="togglePriorityMenu('new')"
      @select-priority="selectPriority('new', $event)"
    />

    <div v-if="focusMode" class="focus-toolbar">
      <span class="focus-title">专注模式</span>
      <button class="btn-icon" title="退出专注模式" @click="emit('exitFocus')">
        <span class="material-symbols-outlined">close_fullscreen</span>
      </button>
    </div>

    <div class="task-list">
      <TaskItem
        v-for="task in visibleTasks"
        :key="task.id"
        :task="task"
        :expanded="taskStore.expandedId === task.id"
        :active-priority-menu="activePriorityMenu"
        @toggle-expand="taskStore.toggleExpand"
        @toggle-status="taskStore.toggleStatus"
        @toggle-priority-menu="togglePriorityMenu"
        @select-priority="selectPriority"
        @update-title="updateTaskTitle"
        @toggle-subtask="taskStore.toggleSubtask"
        @add-subtask="taskStore.addSubtask"
      />

      <div v-if="focusMode && visibleTasks.length === 0" class="focus-empty-state">
        <span class="material-symbols-outlined">check_circle</span>
        <p>没有待专注的任务</p>
        <button class="focus-exit-button" @click="emit('exitFocus')">
          退出专注模式
        </button>
      </div>
    </div>

    <TaskFooterStats
      v-if="!focusMode"
      :total-count="taskStore.tasks.length"
      :active-count="taskStore.activeTasks.length"
      :completed-count="taskStore.completedTasks.length"
      @clear-completed="clearCompleted"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import { useTaskStore } from "../../stores/task";
import type { Priority } from "../../repositories/types";
import QuickTaskEntry from "./QuickTaskEntry.vue";
import TaskFooterStats from "./TaskFooterStats.vue";
import TaskItem from "./TaskItem.vue";

const taskStore = useTaskStore();
const props = withDefaults(defineProps<{ focusMode?: boolean }>(), {
  focusMode: false,
});
const emit = defineEmits<{
  (event: "exitFocus"): void;
}>();
const newTaskTitle = ref("");
const newTaskPriority = ref<Priority>("low");
const activePriorityMenu = ref<string | null>(null);
const visibleTasks = computed(() => (props.focusMode ? taskStore.activeTasks : taskStore.tasks));

onMounted(() => {
  taskStore.fetchTasks();
  document.addEventListener("click", closePriorityMenu);
});

onUnmounted(() => {
  document.removeEventListener("click", closePriorityMenu);
});

function closePriorityMenu() {
  activePriorityMenu.value = null;
}

function togglePriorityMenu(id: string) {
  activePriorityMenu.value = activePriorityMenu.value === id ? null : id;
}

async function selectPriority(id: string, priority: Priority) {
  if (id === "new") {
    newTaskPriority.value = priority;
  } else {
    await taskStore.updateTask(id, { priority });
  }
  activePriorityMenu.value = null;
}

async function updateTaskTitle(taskId: string, title: string) {
  await taskStore.updateTask(taskId, { title });
}

async function addTask() {
  const title = newTaskTitle.value.trim();
  if (!title) return;
  await taskStore.createTask({ title, priority: newTaskPriority.value });
  newTaskTitle.value = "";
}

async function clearCompleted() {
  await Promise.all(taskStore.completedTasks.map((task) => taskStore.deleteTask(task.id)));
}
</script>
