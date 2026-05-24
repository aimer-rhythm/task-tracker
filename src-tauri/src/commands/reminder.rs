use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use crate::db::Database;
use rusqlite::params;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub reminder_type: String,
    pub interval_seconds: Option<i64>,
    pub next_trigger_at: String,
    pub is_active: bool,
    pub sound_enabled: bool,
    pub created_at: String,
}

#[tauri::command]
pub fn create_reminder(db: State<Arc<Database>>, reminder: Reminder) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO reminders (id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
        params![reminder.id, reminder.title, reminder.reminder_type, reminder.interval_seconds, reminder.next_trigger_at, reminder.is_active as i32, reminder.sound_enabled as i32, reminder.created_at],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_reminder(db: State<Arc<Database>>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM reminders WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_reminder(db: State<Arc<Database>>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE reminders SET is_active = CASE WHEN is_active=0 THEN 1 ELSE 0 END WHERE id=?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn list_reminders(db: State<Arc<Database>>, active_only: bool) -> Result<Vec<Reminder>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let sql = if active_only {
        "SELECT id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at FROM reminders WHERE is_active=1 ORDER BY created_at DESC"
    } else {
        "SELECT id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at FROM reminders ORDER BY created_at DESC"
    };
    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(Reminder {
            id: row.get(0)?,
            title: row.get(1)?,
            reminder_type: row.get(2)?,
            interval_seconds: row.get(3)?,
            next_trigger_at: row.get(4)?,
            is_active: row.get::<_, i32>(5)? != 0,
            sound_enabled: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn get_active_reminders(db: &Database) -> Vec<Reminder> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at FROM reminders WHERE is_active=1").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(Reminder {
            id: row.get(0)?,
            title: row.get(1)?,
            reminder_type: row.get(2)?,
            interval_seconds: row.get(3)?,
            next_trigger_at: row.get(4)?,
            is_active: row.get::<_, i32>(5)? != 0,
            sound_enabled: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
        })
    }).unwrap();
    rows.filter_map(|r| r.ok()).collect()
}

pub fn update_next_trigger(db: &Database, id: &str, next: &str) {
    let conn = db.conn.lock().unwrap();
    conn.execute("UPDATE reminders SET next_trigger_at=?1 WHERE id=?2", params![next, id]).ok();
}
