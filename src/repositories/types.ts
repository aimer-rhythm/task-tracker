export type TaskStatus = "todo" | "in_progress" | "done";
export type Priority = "high" | "medium" | "low";
export type ReminderType = "once" | "recurring";

export interface Task {
  id: string;
  title: string;
  description: string;
  status: TaskStatus;
  priority: Priority;
  progress: number;
  category: string;
  tags: string[];
  subtasks: Subtask[];
  dueDate: string | null;
  createdAt: string;
  updatedAt: string;
  completedAt: string | null;
}

export interface Subtask {
  id: string;
  title: string;
  isDone: boolean;
  sortOrder: number;
}

export interface Reminder {
  id: string;
  title: string;
  type: ReminderType;
  intervalSeconds: number | null;
  nextTriggerAt: string;
  isActive: boolean;
  soundEnabled: boolean;
  createdAt: string;
}

export interface CreateTaskInput {
  title: string;
  description?: string;
  priority?: Priority;
  dueDate?: string;
}

export interface UpdateTaskInput {
  title?: string;
  description?: string;
  status?: TaskStatus;
  priority?: Priority;
  progress?: number;
  category?: string;
  dueDate?: string | null;
}

export interface TaskFilter {
  status?: TaskStatus;
  priority?: Priority;
  category?: string;
  search?: string;
}

export interface CreateReminderInput {
  title: string;
  type: ReminderType;
  intervalSeconds?: number;
  triggerAt?: string;
}

export interface TaskRepository {
  create(input: CreateTaskInput): Promise<Task>;
  update(id: string, data: UpdateTaskInput): Promise<Task>;
  delete(id: string): Promise<void>;
  getById(id: string): Promise<Task | null>;
  list(filter?: TaskFilter): Promise<Task[]>;
  addSubtask(taskId: string, title: string): Promise<Subtask>;
  toggleSubtask(taskId: string, subtaskId: string): Promise<void>;
  deleteSubtask(taskId: string, subtaskId: string): Promise<void>;
}

export interface ReminderRepository {
  create(input: CreateReminderInput): Promise<Reminder>;
  delete(id: string): Promise<void>;
  listActive(): Promise<Reminder[]>;
  listAll(): Promise<Reminder[]>;
  toggle(id: string): Promise<void>;
}
