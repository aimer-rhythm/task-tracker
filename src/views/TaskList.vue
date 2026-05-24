<template>
  <div class="task-list-view">
    <!-- Quick Entry -->
    <div class="quick-entry">
      <input
        v-model="newTaskTitle"
        placeholder="+ New task..."
        @keydown.enter="addTask"
      />
      <div class="priority-toggle-wrapper">
        <button class="priority-toggle" @click.stop="togglePriorityMenu">
          <span
            class="material-symbols-outlined"
            :class="priorityColorClass(newTaskPriority)"
            style="font-variation-settings: 'FILL' 1"
          >flag</span>
          <span :class="priorityColorClass(newTaskPriority)">{{ priorityLabel(newTaskPriority) }}</span>
          <span class="material-symbols-outlined expand-icon">expand_more</span>
        </button>
        <div v-if="showPriorityMenu" class="priority-menu">
          <button
            :class="['priority-menu-item', { selected: newTaskPriority === 'high' }]"
            @click="selectPriority('high')"
          >
            <span class="material-symbols-outlined priority-high-color" style="font-variation-settings: 'FILL' 1">flag</span>
            <span>High</span>
            <span v-if="newTaskPriority === 'high'" class="material-symbols-outlined check-icon">check</span>
          </button>
          <button
            :class="['priority-menu-item', { selected: newTaskPriority === 'medium' }]"
            @click="selectPriority('medium')"
          >
            <span class="material-symbols-outlined priority-medium-color" style="font-variation-settings: 'FILL' 1">flag</span>
            <span>Medium</span>
            <span v-if="newTaskPriority === 'medium'" class="material-symbols-outlined check-icon">check</span>
          </button>
          <button
            :class="['priority-menu-item', { selected: newTaskPriority === 'low' }]"
            @click="selectPriority('low')"
          >
            <span class="material-symbols-outlined priority-low-color" style="font-variation-settings: 'FILL' 1">flag</span>
            <span>Low</span>
            <span v-if="newTaskPriority === 'low'" class="material-symbols-outlined check-icon">check</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Task List -->
    <div class="task-list">
      <template v-for="task in taskStore.tasks" :key="task.id">
        <!-- Collapsed task card -->
        <div
          v-if="taskStore.expandedId !== task.id"
          :class="['task-card', { completed: task.status === 'done' }]"
          @click="taskStore.toggleExpand(task.id)"
        >
          <div
            :class="['task-checkbox', { checked: task.status === 'done' }]"
            @click.stop="taskStore.toggleStatus(task.id)"
          >
            <span v-if="task.status === 'done'" class="material-symbols-outlined">check</span>
          </div>
          <span :class="['task-title', { done: task.status === 'done' }]">{{ task.title }}</span>
          <span
            :class="['material-symbols-outlined', 'priority-flag', priorityColorClass(task.priority), { completed: task.status === 'done' }]"
            style="font-variation-settings: 'FILL' 1"
          >flag</span>
        </div>

        <!-- Expanded task card -->
        <div v-else class="task-expanded">
          <div class="task-expanded-header" @click="taskStore.toggleExpand(task.id)">
            <div
              :class="['task-checkbox', { checked: task.status === 'done' }]"
              @click.stop="taskStore.toggleStatus(task.id)"
            >
              <span v-if="task.status === 'done'" class="material-symbols-outlined">check</span>
            </div>
            <span class="task-title medium">{{ task.title }}</span>
            <span
              :class="['material-symbols-outlined', 'priority-flag', priorityColorClass(task.priority)]"
              style="font-variation-settings: 'FILL' 1"
            >flag</span>
          </div>

          <!-- Progress Section -->
          <div class="progress-section">
            <div class="progress-header">
              <span>Subtasks ({{ completedSubtasks(task) }}/{{ task.subtasks.length }})</span>
              <span class="progress-percent">{{ taskProgress(task) }}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: taskProgress(task) + '%' }"></div>
            </div>

            <!-- Subtasks -->
            <ul class="subtask-list">
              <li
                v-for="sub in task.subtasks"
                :key="sub.id"
                :class="['subtask-item', { done: sub.isDone }]"
              >
                <span
                  class="material-symbols-outlined"
                  :class="sub.isDone ? '' : 'unchecked'"
                  :style="sub.isDone ? 'font-variation-settings: \'FILL\' 1' : ''"
                  @click="taskStore.toggleSubtask(task.id, sub.id)"
                >{{ sub.isDone ? 'check_box' : 'check_box_outline_blank' }}</span>
                <span>{{ sub.title }}</span>
              </li>
            </ul>

            <!-- Add subtask -->
            <input
              v-model="newSubtaskTitle"
              class="subtask-input"
              placeholder="+ Add subtask..."
              @keydown.enter="addSubtask(task.id)"
            />
          </div>
        </div>
      </template>
    </div>

    <!-- Footer Stats -->
    <div v-if="taskStore.tasks.length > 0" class="footer-stats">
      <span>{{ taskStore.activeTasks.length }} active tasks remaining</span>
      <button v-if="taskStore.completedTasks.length > 0" @click="clearCompleted">Clear completed</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useTaskStore } from "../stores/task";
import type { Task, Priority } from "../repositories/types";

const taskStore = useTaskStore();
const newTaskTitle = ref("");
const newTaskPriority = ref<Priority>("low");
const newSubtaskTitle = ref("");
const showPriorityMenu = ref(false);

onMounted(() => {
  taskStore.fetchTasks();
  document.addEventListener('click', closePriorityMenu);
});

onUnmounted(() => {
  document.removeEventListener('click', closePriorityMenu);
});

function closePriorityMenu() {
  showPriorityMenu.value = false;
}

function togglePriorityMenu() {
  showPriorityMenu.value = !showPriorityMenu.value;
}

function selectPriority(p: Priority) {
  newTaskPriority.value = p;
  showPriorityMenu.value = false;
}

function priorityLabel(p: string): string {
  if (p === "high") return "High";
  if (p === "low") return "Low";
  return "Medium";
}

function priorityColorClass(p: string): string {
  if (p === "high") return "priority-high-color";
  if (p === "low") return "priority-low-color";
  return "priority-medium-color";
}

function completedSubtasks(task: Task): number {
  return task.subtasks.filter((s) => s.isDone).length;
}

function taskProgress(task: Task): number {
  if (task.subtasks.length === 0) return task.progress;
  const done = task.subtasks.filter((s) => s.isDone).length;
  return Math.round((done / task.subtasks.length) * 100);
}

async function addTask() {
  const title = newTaskTitle.value.trim();
  if (!title) return;
  await taskStore.createTask({ title, priority: newTaskPriority.value });
  newTaskTitle.value = "";
}

async function addSubtask(taskId: string) {
  const title = newSubtaskTitle.value.trim();
  if (!title) return;
  await taskStore.addSubtask(taskId, title);
  newSubtaskTitle.value = "";
}

async function clearCompleted() {
  for (const task of taskStore.completedTasks) {
    await taskStore.deleteTask(task.id);
  }
}
</script>
