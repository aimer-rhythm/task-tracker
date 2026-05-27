use crate::db::Database;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

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
    conn.execute("DELETE FROM reminders WHERE id=?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_reminder(db: State<Arc<Database>>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE reminders SET is_active = CASE WHEN is_active=0 THEN 1 ELSE 0 END WHERE id=?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn list_reminders(
    db: State<Arc<Database>>,
    active_only: bool,
) -> Result<Vec<Reminder>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let sql = if active_only {
        "SELECT id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at FROM reminders WHERE is_active=1 ORDER BY created_at DESC"
    } else {
        "SELECT id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at FROM reminders ORDER BY created_at DESC"
    };
    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

pub fn get_active_reminders(db: &Database) -> Vec<Reminder> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at FROM reminders WHERE is_active=1").unwrap();
    let rows = stmt
        .query_map([], |row| {
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
        })
        .unwrap();
    rows.filter_map(|r| r.ok()).collect()
}

pub fn update_next_trigger(db: &Database, id: &str, next: &str) {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "UPDATE reminders SET next_trigger_at=?1 WHERE id=?2",
        params![next, id],
    )
    .ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn setup_db() -> Database {
        Database::new_in_memory().unwrap()
    }

    fn sample_reminder(id: &str, title: &str) -> Reminder {
        Reminder {
            id: id.to_string(),
            title: title.to_string(),
            reminder_type: "once".to_string(),
            interval_seconds: None,
            next_trigger_at: "2025-01-01T12:00:00Z".to_string(),
            is_active: true,
            sound_enabled: false,
            created_at: "2025-01-01T00:00:00Z".to_string(),
        }
    }

    fn insert_reminder(conn: &rusqlite::Connection, r: &Reminder) {
        conn.execute(
            "INSERT INTO reminders (id, title, type, interval_seconds, next_trigger_at, is_active, sound_enabled, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
            params![r.id, r.title, r.reminder_type, r.interval_seconds, r.next_trigger_at, r.is_active as i32, r.sound_enabled as i32, r.created_at],
        ).unwrap();
    }

    #[test]
    fn test_create_and_list_reminders() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        let r1 = sample_reminder("r1", "Drink water");
        insert_reminder(&conn, &r1);

        let mut stmt = conn
            .prepare("SELECT id, title FROM reminders WHERE is_active=1")
            .unwrap();
        let ids: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(ids, vec!["r1"]);
    }

    #[test]
    fn test_toggle_reminder() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        let r1 = sample_reminder("r1", "Stand up");
        insert_reminder(&conn, &r1);

        conn.execute(
            "UPDATE reminders SET is_active = CASE WHEN is_active=0 THEN 1 ELSE 0 END WHERE id=?1",
            params!["r1"],
        )
        .unwrap();

        let active: i32 = conn
            .query_row(
                "SELECT is_active FROM reminders WHERE id=?1",
                params!["r1"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(active, 0);
    }

    #[test]
    fn test_delete_reminder() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        insert_reminder(&conn, &sample_reminder("r1", "Test"));
        conn.execute("DELETE FROM reminders WHERE id=?1", params!["r1"])
            .unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM reminders", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_update_next_trigger() {
        let db = setup_db();
        {
            let conn = db.conn.lock().unwrap();
            insert_reminder(&conn, &sample_reminder("r1", "Recurring"));
        }

        update_next_trigger(&db, "r1", "2025-01-02T12:00:00Z");

        let conn = db.conn.lock().unwrap();
        let next: String = conn
            .query_row(
                "SELECT next_trigger_at FROM reminders WHERE id=?1",
                params!["r1"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(next, "2025-01-02T12:00:00Z");
    }

    #[test]
    fn test_get_active_reminders() {
        let db = setup_db();
        {
            let conn = db.conn.lock().unwrap();
            insert_reminder(&conn, &sample_reminder("r1", "Active"));
            let mut r2 = sample_reminder("r2", "Inactive");
            r2.is_active = false;
            insert_reminder(&conn, &r2);
        }

        let active = get_active_reminders(&db);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, "r1");
    }

    #[test]
    fn test_recurring_reminder_fields() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        let mut r = sample_reminder("r1", "Every 5 min");
        r.reminder_type = "recurring".to_string();
        r.interval_seconds = Some(300);
        insert_reminder(&conn, &r);

        let interval: Option<i64> = conn
            .query_row(
                "SELECT interval_seconds FROM reminders WHERE id=?1",
                params!["r1"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(interval, Some(300));
    }
}
