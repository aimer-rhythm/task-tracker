<template>
  <div
    v-if="!expanded"
    :class="['task-card', { completed: task.status === 'done' }]"
    @click="emit('toggleExpand', task.id)"
  >
    <div class="task-card-main">
      <div :class="['task-checkbox', { checked: isDone }]" @click.stop="emit('toggleStatus', task.id)">
        <span v-if="isDone" class="material-symbols-outlined">check</span>
      </div>
      <input
        v-if="isEditing"
        ref="editInput"
        v-model="editingTitle"
        class="task-edit-input"
        maxlength="200"
        @click.stop
        @blur="saveEdit"
        @keydown.enter="saveEdit"
        @keydown.escape="cancelEdit"
      />
      <span v-else :class="['task-title', { done: isDone }]">{{ task.title }}</span>
      <button v-if="!isEditing" class="edit-btn" @click.stop="startEdit">
        <span class="material-symbols-outlined">edit</span>
      </button>
      <div class="priority-flag-wrapper">
        <button class="priority-icon-btn" @click.stop="emit('togglePriorityMenu', task.id)">
          <span
            :class="[
              'material-symbols-outlined',
              'priority-flag',
              priorityColorClass(task.priority),
              { completed: isDone },
            ]"
            style="font-variation-settings: 'FILL' 1"
          >flag</span>
        </button>
        <PriorityMenu
          v-if="activePriorityMenu === task.id"
          :value="task.priority"
          @select="emit('selectPriority', task.id, $event)"
        />
      </div>
    </div>
    <TaskProgress
      v-if="task.subtasks.length > 0"
      :completed="completedSubtasks"
      :total="task.subtasks.length"
      :progress="taskProgress"
      compact
    />
  </div>

  <div v-else class="task-expanded">
    <div class="task-expanded-header" @click="emit('toggleExpand', task.id)">
      <div :class="['task-checkbox', { checked: isDone }]" @click.stop="emit('toggleStatus', task.id)">
        <span v-if="isDone" class="material-symbols-outlined">check</span>
      </div>
      <input
        v-if="isEditing"
        ref="editInput"
        v-model="editingTitle"
        class="task-edit-input"
        maxlength="200"
        @click.stop
        @blur="saveEdit"
        @keydown.enter="saveEdit"
        @keydown.escape="cancelEdit"
      />
      <span v-else class="task-title medium">{{ task.title }}</span>
      <button v-if="!isEditing" class="edit-btn" @click.stop="startEdit">
        <span class="material-symbols-outlined">edit</span>
      </button>
      <div class="priority-flag-wrapper">
        <button class="priority-icon-btn" @click.stop="emit('togglePriorityMenu', task.id)">
          <span
            :class="['material-symbols-outlined', 'priority-flag', priorityColorClass(task.priority)]"
            style="font-variation-settings: 'FILL' 1"
          >flag</span>
        </button>
        <PriorityMenu
          v-if="activePriorityMenu === task.id"
          :value="task.priority"
          @select="emit('selectPriority', task.id, $event)"
        />
      </div>
    </div>

    <div class="progress-section">
      <template v-if="task.subtasks.length > 0">
        <TaskProgress
          :completed="completedSubtasks"
          :total="task.subtasks.length"
          :progress="taskProgress"
        />
        <SubtaskList :subtasks="task.subtasks" @toggle="emit('toggleSubtask', task.id, $event)" />
      </template>

      <input
        v-model="newSubtaskTitle"
        class="subtask-input"
        placeholder="+ 添加子任务..."
        @keydown.enter="addSubtask"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref } from "vue";
import type { Priority, Task } from "../../repositories/types";
import PriorityMenu from "./PriorityMenu.vue";
import SubtaskList from "./SubtaskList.vue";
import TaskProgress from "./TaskProgress.vue";

const props = defineProps<{
  task: Task;
  expanded: boolean;
  activePriorityMenu: string | null;
}>();

const emit = defineEmits<{
  toggleExpand: [taskId: string];
  toggleStatus: [taskId: string];
  togglePriorityMenu: [taskId: string];
  selectPriority: [taskId: string, priority: Priority];
  updateTitle: [taskId: string, title: string];
  toggleSubtask: [taskId: string, subtaskId: string];
  addSubtask: [taskId: string, title: string];
}>();

const isEditing = ref(false);
const editingTitle = ref("");
const editInput = ref<HTMLInputElement | null>(null);
const newSubtaskTitle = ref("");

const isDone = computed(() => props.task.status === "done");
const completedSubtasks = computed(() => props.task.subtasks.filter((subtask) => subtask.isDone).length);
const taskProgress = computed(() => {
  if (props.task.subtasks.length === 0) return props.task.progress;
  return Math.round((completedSubtasks.value / props.task.subtasks.length) * 100);
});

function priorityColorClass(priority: Priority): string {
  if (priority === "high") return "priority-high-color";
  if (priority === "low") return "priority-low-color";
  return "priority-medium-color";
}

function startEdit() {
  isEditing.value = true;
  editingTitle.value = props.task.title;
  nextTick(() => {
    editInput.value?.focus();
  });
}

function cancelEdit() {
  isEditing.value = false;
  editingTitle.value = "";
}

function saveEdit() {
  const title = editingTitle.value.trim();
  if (title && title !== props.task.title) {
    emit("updateTitle", props.task.id, title);
  }
  cancelEdit();
}

function addSubtask() {
  const title = newSubtaskTitle.value.trim();
  if (!title) return;
  emit("addSubtask", props.task.id, title);
  newSubtaskTitle.value = "";
}
</script>
