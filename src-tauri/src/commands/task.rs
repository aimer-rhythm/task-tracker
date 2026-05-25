use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use crate::db::Database;
use rusqlite::params;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub progress: i32,
    pub category: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub subtasks: Vec<Subtask>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subtask {
    pub id: String,
    pub title: String,
    pub is_done: bool,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskFilter {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub category: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub progress: Option<i32>,
    pub category: Option<String>,
    pub due_date: Option<String>,
    pub updated_at: Option<String>,
}

#[tauri::command]
pub fn create_task(db: State<Arc<Database>>, task: Task) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, progress, category, due_date, created_at, updated_at, completed_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
        params![task.id, task.title, task.description, task.status, task.priority, task.progress, task.category, task.due_date, task.created_at, task.updated_at, task.completed_at],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_task(db: State<Arc<Database>>, id: String, data: UpdateTaskData) -> Result<Task, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();

    if let Some(ref title) = data.title { conn.execute("UPDATE tasks SET title=?1, updated_at=?2 WHERE id=?3", params![title, now, id]).map_err(|e| e.to_string())?; }
    if let Some(ref desc) = data.description { conn.execute("UPDATE tasks SET description=?1, updated_at=?2 WHERE id=?3", params![desc, now, id]).map_err(|e| e.to_string())?; }
    if let Some(ref status) = data.status {
        let completed_at = if status == "done" { Some(now.clone()) } else { None };
        conn.execute("UPDATE tasks SET status=?1, completed_at=?2, updated_at=?3 WHERE id=?4", params![status, completed_at, now, id]).map_err(|e| e.to_string())?;
    }
    if let Some(ref priority) = data.priority { conn.execute("UPDATE tasks SET priority=?1, updated_at=?2 WHERE id=?3", params![priority, now, id]).map_err(|e| e.to_string())?; }
    if let Some(progress) = data.progress { conn.execute("UPDATE tasks SET progress=?1, updated_at=?2 WHERE id=?3", params![progress, now, id]).map_err(|e| e.to_string())?; }

    get_task_internal(&conn, &id)
}

#[tauri::command]
pub fn delete_task(db: State<Arc<Database>>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM subtasks WHERE task_id=?1", params![id]).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_task(db: State<Arc<Database>>, id: String) -> Result<Option<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    Ok(Some(get_task_internal(&conn, &id)?))
}

#[tauri::command]
pub fn list_tasks(db: State<Arc<Database>>, filter: TaskFilter) -> Result<Vec<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut sql = "SELECT id, title, description, status, priority, progress, category, due_date, created_at, updated_at, completed_at FROM tasks WHERE 1=1".to_string();
    let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

    if let Some(ref status) = filter.status {
        sql.push_str(" AND status=?");
        params_vec.push(Box::new(status.clone()));
    }
    if let Some(ref priority) = filter.priority {
        sql.push_str(" AND priority=?");
        params_vec.push(Box::new(priority.clone()));
    }
    if let Some(ref search) = filter.search {
        sql.push_str(" AND title LIKE ?");
        params_vec.push(Box::new(format!("%{}%", search)));
    }
    sql.push_str(" ORDER BY created_at DESC");

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            priority: row.get(4)?,
            progress: row.get(5)?,
            category: row.get(6)?,
            tags: vec![],
            subtasks: vec![],
            due_date: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
            completed_at: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut tasks: Vec<Task> = vec![];
    for row in rows {
        let mut task = row.map_err(|e| e.to_string())?;
        task.subtasks = get_subtasks(&conn, &task.id).unwrap_or_default();
        tasks.push(task);
    }
    Ok(tasks)
}

#[tauri::command]
pub fn add_subtask(db: State<Arc<Database>>, task_id: String, subtask: Subtask) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO subtasks (id, task_id, title, is_done, sort_order) VALUES (?1,?2,?3,?4,?5)",
        params![subtask.id, task_id, subtask.title, subtask.is_done as i32, subtask.sort_order],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_subtask(db: State<Arc<Database>>, task_id: String, subtask_id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE subtasks SET is_done = CASE WHEN is_done=0 THEN 1 ELSE 0 END WHERE id=?1 AND task_id=?2",
        params![subtask_id, task_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_subtask(db: State<Arc<Database>>, task_id: String, subtask_id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM subtasks WHERE id=?1 AND task_id=?2", params![subtask_id, task_id]).map_err(|e| e.to_string())?;
    Ok(())
}

fn get_task_internal(conn: &rusqlite::Connection, id: &str) -> Result<Task, String> {
    let mut stmt = conn.prepare("SELECT id, title, description, status, priority, progress, category, due_date, created_at, updated_at, completed_at FROM tasks WHERE id=?1")
        .map_err(|e| e.to_string())?;
    let task = stmt.query_row(params![id], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            priority: row.get(4)?,
            progress: row.get(5)?,
            category: row.get(6)?,
            tags: vec![],
            subtasks: vec![],
            due_date: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
            completed_at: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?;
    let mut task = task;
    task.subtasks = get_subtasks(conn, &task.id).unwrap_or_default();
    Ok(task)
}

fn get_subtasks(conn: &rusqlite::Connection, task_id: &str) -> Result<Vec<Subtask>, String> {
    let mut stmt = conn.prepare("SELECT id, title, is_done, sort_order FROM subtasks WHERE task_id=?1 ORDER BY sort_order")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![task_id], |row| {
        Ok(Subtask {
            id: row.get(0)?,
            title: row.get(1)?,
            is_done: row.get::<_, i32>(2)? != 0,
            sort_order: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn setup_db() -> Database {
        Database::new_in_memory().unwrap()
    }

    fn sample_task(id: &str, title: &str) -> Task {
        Task {
            id: id.to_string(),
            title: title.to_string(),
            description: String::new(),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            progress: 0,
            category: String::new(),
            tags: vec![],
            subtasks: vec![],
            due_date: None,
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            completed_at: None,
        }
    }

    fn insert_task(conn: &rusqlite::Connection, task: &Task) {
        conn.execute(
            "INSERT INTO tasks (id, title, description, status, priority, progress, category, due_date, created_at, updated_at, completed_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![task.id, task.title, task.description, task.status, task.priority, task.progress, task.category, task.due_date, task.created_at, task.updated_at, task.completed_at],
        ).unwrap();
    }

    #[test]
    fn test_create_and_get_task() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let task = sample_task("t1", "Buy groceries");
        insert_task(&conn, &task);

        let fetched = get_task_internal(&conn, "t1").unwrap();
        assert_eq!(fetched.id, "t1");
        assert_eq!(fetched.title, "Buy groceries");
        assert_eq!(fetched.status, "todo");
        assert_eq!(fetched.priority, "medium");
    }

    #[test]
    fn test_get_task_not_found() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let result = get_task_internal(&conn, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_tasks_with_filter() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        let t1 = sample_task("t1", "Task A");
        let mut t2 = sample_task("t2", "Task B");
        t2.status = "done".to_string();
        t2.priority = "high".to_string();
        insert_task(&conn, &t1);
        insert_task(&conn, &t2);

        // filter by status
        let mut sql = "SELECT id FROM tasks WHERE 1=1 AND status=?1".to_string();
        let mut stmt = conn.prepare(&sql).unwrap();
        let ids: Vec<String> = stmt.query_map(params!["done"], |row| row.get(0)).unwrap()
            .filter_map(|r| r.ok()).collect();
        assert_eq!(ids, vec!["t2"]);
    }

    #[test]
    fn test_subtask_crud() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let task = sample_task("t1", "Parent task");
        insert_task(&conn, &task);

        // add subtask
        conn.execute(
            "INSERT INTO subtasks (id, task_id, title, is_done, sort_order) VALUES (?1,?2,?3,?4,?5)",
            params!["s1", "t1", "Sub item", 0, 0],
        ).unwrap();

        let subs = get_subtasks(&conn, "t1").unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].title, "Sub item");
        assert!(!subs[0].is_done);

        // toggle subtask
        conn.execute(
            "UPDATE subtasks SET is_done = CASE WHEN is_done=0 THEN 1 ELSE 0 END WHERE id=?1 AND task_id=?2",
            params!["s1", "t1"],
        ).unwrap();

        let subs = get_subtasks(&conn, "t1").unwrap();
        assert!(subs[0].is_done);

        // delete subtask
        conn.execute("DELETE FROM subtasks WHERE id=?1 AND task_id=?2", params!["s1", "t1"]).unwrap();
        let subs = get_subtasks(&conn, "t1").unwrap();
        assert!(subs.is_empty());
    }

    #[test]
    fn test_delete_task_cascades_subtasks() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        // enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON").unwrap();

        let task = sample_task("t1", "Parent");
        insert_task(&conn, &task);
        conn.execute(
            "INSERT INTO subtasks (id, task_id, title, is_done, sort_order) VALUES (?1,?2,?3,?4,?5)",
            params!["s1", "t1", "Child", 0, 0],
        ).unwrap();

        conn.execute("DELETE FROM tasks WHERE id=?1", params!["t1"]).unwrap();
        let subs = get_subtasks(&conn, "t1").unwrap();
        assert!(subs.is_empty());
    }

    #[test]
    fn test_update_task_fields() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let task = sample_task("t1", "Original");
        insert_task(&conn, &task);

        conn.execute("UPDATE tasks SET title=?1, updated_at=?2 WHERE id=?3",
            params!["Updated Title", "2025-01-02T00:00:00Z", "t1"]).unwrap();

        let fetched = get_task_internal(&conn, "t1").unwrap();
        assert_eq!(fetched.title, "Updated Title");
        assert_eq!(fetched.updated_at, "2025-01-02T00:00:00Z");
    }

    #[test]
    fn test_search_filter() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        insert_task(&conn, &sample_task("t1", "Buy milk"));
        insert_task(&conn, &sample_task("t2", "Read book"));
        insert_task(&conn, &sample_task("t3", "Buy eggs"));

        let mut stmt = conn.prepare("SELECT id FROM tasks WHERE title LIKE ?1").unwrap();
        let ids: Vec<String> = stmt.query_map(params!["%Buy%"], |row| row.get(0)).unwrap()
            .filter_map(|r| r.ok()).collect();
        assert_eq!(ids.len(), 2);
    }
}
