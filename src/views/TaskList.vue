<template>
  <div class="task-list-view">
    <!-- Quick Entry -->
    <div class="quick-entry">
      <input
        v-model="newTaskTitle"
        placeholder="+ 新任务..."
        maxlength="200"
        @keydown.enter="addTask"
      />
      <div class="priority-toggle-wrapper">
        <button class="priority-icon-btn" @click.stop="togglePriorityMenu('new')">
          <span
            class="material-symbols-outlined"
            :class="priorityColorClass(newTaskPriority)"
            style="font-variation-settings: 'FILL' 1"
          >flag</span>
        </button>
        <div v-if="activePriorityMenu === 'new'" class="priority-menu">
          <button
            :class="['priority-menu-item', { selected: newTaskPriority === 'high' }]"
            @click="selectPriority('new', 'high')"
          >
            <span class="material-symbols-outlined priority-high-color" style="font-variation-settings: 'FILL' 1">flag</span>
            <span>高</span>
            <span v-if="newTaskPriority === 'high'" class="material-symbols-outlined check-icon">check</span>
          </button>
          <button
            :class="['priority-menu-item', { selected: newTaskPriority === 'medium' }]"
            @click="selectPriority('new', 'medium')"
          >
            <span class="material-symbols-outlined priority-medium-color" style="font-variation-settings: 'FILL' 1">flag</span>
            <span>中</span>
            <span v-if="newTaskPriority === 'medium'" class="material-symbols-outlined check-icon">check</span>
          </button>
          <button
            :class="['priority-menu-item', { selected: newTaskPriority === 'low' }]"
            @click="selectPriority('new', 'low')"
          >
            <span class="material-symbols-outlined priority-low-color" style="font-variation-settings: 'FILL' 1">flag</span>
            <span>低</span>
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
          <input
            v-if="editingTaskId === task.id"
            v-model="editingTitle"
            class="task-edit-input"
            maxlength="200"
            @click.stop
            @blur="saveEdit(task.id)"
            @keydown.enter="saveEdit(task.id)"
            @keydown.escape="cancelEdit"
            ref="editInput"
          />
          <span v-else :class="['task-title', { done: task.status === 'done' }]">{{ task.title }}</span>
          <button
            v-if="editingTaskId !== task.id"
            class="edit-btn"
            @click.stop="startEdit(task)"
          >
            <span class="material-symbols-outlined">edit</span>
          </button>
          <div class="priority-flag-wrapper">
            <button
              class="priority-icon-btn"
              @click.stop="togglePriorityMenu(task.id)"
            >
              <span
                :class="['material-symbols-outlined', 'priority-flag', priorityColorClass(task.priority), { completed: task.status === 'done' }]"
                style="font-variation-settings: 'FILL' 1"
              >flag</span>
            </button>
            <div v-if="activePriorityMenu === task.id" class="priority-menu">
              <button
                :class="['priority-menu-item', { selected: task.priority === 'high' }]"
                @click.stop="selectPriority(task.id, 'high')"
              >
                <span class="material-symbols-outlined priority-high-color" style="font-variation-settings: 'FILL' 1">flag</span>
                <span>高</span>
                <span v-if="task.priority === 'high'" class="material-symbols-outlined check-icon">check</span>
              </button>
              <button
                :class="['priority-menu-item', { selected: task.priority === 'medium' }]"
                @click.stop="selectPriority(task.id, 'medium')"
              >
                <span class="material-symbols-outlined priority-medium-color" style="font-variation-settings: 'FILL' 1">flag</span>
                <span>中</span>
                <span v-if="task.priority === 'medium'" class="material-symbols-outlined check-icon">check</span>
              </button>
              <button
                :class="['priority-menu-item', { selected: task.priority === 'low' }]"
                @click.stop="selectPriority(task.id, 'low')"
              >
                <span class="material-symbols-outlined priority-low-color" style="font-variation-settings: 'FILL' 1">flag</span>
                <span>低</span>
                <span v-if="task.priority === 'low'" class="material-symbols-outlined check-icon">check</span>
              </button>
            </div>
          </div>
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
            <input
              v-if="editingTaskId === task.id"
              v-model="editingTitle"
              class="task-edit-input"
              maxlength="200"
              @click.stop
              @blur="saveEdit(task.id)"
              @keydown.enter="saveEdit(task.id)"
              @keydown.escape="cancelEdit"
            />
            <span v-else class="task-title medium">{{ task.title }}</span>
            <button
              v-if="editingTaskId !== task.id"
              class="edit-btn"
              @click.stop="startEdit(task)"
            >
              <span class="material-symbols-outlined">edit</span>
            </button>
            <div class="priority-flag-wrapper">
              <button
                class="priority-icon-btn"
                @click.stop="togglePriorityMenu(task.id)"
              >
                <span
                  :class="['material-symbols-outlined', 'priority-flag', priorityColorClass(task.priority)]"
                  style="font-variation-settings: 'FILL' 1"
                >flag</span>
              </button>
              <div v-if="activePriorityMenu === task.id" class="priority-menu">
                <button
                  :class="['priority-menu-item', { selected: task.priority === 'high' }]"
                  @click.stop="selectPriority(task.id, 'high')"
                >
                  <span class="material-symbols-outlined priority-high-color" style="font-variation-settings: 'FILL' 1">flag</span>
                  <span>高</span>
                  <span v-if="task.priority === 'high'" class="material-symbols-outlined check-icon">check</span>
                </button>
                <button
                  :class="['priority-menu-item', { selected: task.priority === 'medium' }]"
                  @click.stop="selectPriority(task.id, 'medium')"
                >
                  <span class="material-symbols-outlined priority-medium-color" style="font-variation-settings: 'FILL' 1">flag</span>
                  <span>中</span>
                  <span v-if="task.priority === 'medium'" class="material-symbols-outlined check-icon">check</span>
                </button>
                <button
                  :class="['priority-menu-item', { selected: task.priority === 'low' }]"
                  @click.stop="selectPriority(task.id, 'low')"
                >
                  <span class="material-symbols-outlined priority-low-color" style="font-variation-settings: 'FILL' 1">flag</span>
                  <span>低</span>
                  <span v-if="task.priority === 'low'" class="material-symbols-outlined check-icon">check</span>
                </button>
              </div>
            </div>
          </div>

          <!-- Progress Section -->
          <div class="progress-section">
            <div class="progress-header">
              <span>子任务 ({{ completedSubtasks(task) }}/{{ task.subtasks.length }})</span>
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
              placeholder="+ 添加子任务..."
              @keydown.enter="addSubtask(task.id)"
            />
          </div>
        </div>
      </template>
    </div>

    <!-- Footer Stats -->
    <div v-if="taskStore.tasks.length > 0" class="footer-stats">
      <span>{{ taskStore.activeTasks.length }} 个待办任务</span>
      <button v-if="taskStore.completedTasks.length > 0" @click="clearCompleted">清除已完成</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted, onUnmounted } from "vue";
import { useTaskStore } from "../stores/task";
import type { Task, Priority } from "../repositories/types";

const taskStore = useTaskStore();
const newTaskTitle = ref("");
const newTaskPriority = ref<Priority>("low");
const newSubtaskTitle = ref("");
const activePriorityMenu = ref<string | null>(null);
const editingTaskId = ref<string | null>(null);
const editingTitle = ref("");

onMounted(() => {
  taskStore.fetchTasks();
  document.addEventListener('click', closePriorityMenu);
});

onUnmounted(() => {
  document.removeEventListener('click', closePriorityMenu);
});

function closePriorityMenu() {
  activePriorityMenu.value = null;
}

function togglePriorityMenu(id: string) {
  activePriorityMenu.value = activePriorityMenu.value === id ? null : id;
}

async function selectPriority(id: string, p: Priority) {
  if (id === 'new') {
    newTaskPriority.value = p;
  } else {
    await taskStore.updateTask(id, { priority: p });
  }
  activePriorityMenu.value = null;
}

function priorityColorClass(p: string): string {
  if (p === "high") return "priority-high-color";
  if (p === "low") return "priority-low-color";
  return "priority-medium-color";
}

function startEdit(task: Task) {
  editingTaskId.value = task.id;
  editingTitle.value = task.title;
  nextTick(() => {
    const input = document.querySelector('.task-edit-input') as HTMLInputElement;
    input?.focus();
  });
}

function cancelEdit() {
  editingTaskId.value = null;
  editingTitle.value = "";
}

async function saveEdit(taskId: string) {
  const title = editingTitle.value.trim();
  if (!title) {
    cancelEdit();
    return;
  }
  const task = taskStore.tasks.find((t) => t.id === taskId);
  if (task && title !== task.title) {
    await taskStore.updateTask(taskId, { title });
  }
  editingTaskId.value = null;
  editingTitle.value = "";
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
