use crate::commands::reminder::{get_active_reminders, update_next_trigger};
use crate::db::Database;
use chrono::Utc;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn start_reminder_loop(app: AppHandle, db: Arc<Database>) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        let now = Utc::now();
        let reminders = get_active_reminders(&db);

        for reminder in reminders {
            if let Ok(trigger_time) =
                chrono::DateTime::parse_from_rfc3339(&reminder.next_trigger_at)
            {
                if now >= trigger_time {
                    let _ = app
                        .notification()
                        .builder()
                        .title("TaskTracker")
                        .body(&reminder.title)
                        .show();

                    if reminder.reminder_type == "recurring" {
                        if let Some(interval) = reminder.interval_seconds {
                            let next = now + chrono::Duration::seconds(interval);
                            update_next_trigger(&db, &reminder.id, &next.to_rfc3339());
                        }
                    } else {
                        update_next_trigger(
                            &db,
                            &reminder.id,
                            &(now + chrono::Duration::days(365)).to_rfc3339(),
                        );
                    }
                }
            }
        }
    });
}
