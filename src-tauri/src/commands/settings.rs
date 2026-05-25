use std::sync::Arc;
use tauri::State;
use crate::db::Database;
use rusqlite::params;
use std::collections::HashMap;

#[tauri::command]
pub fn set_setting(db: State<Arc<Database>>, key: String, value: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_all_settings(db: State<Arc<Database>>) -> Result<HashMap<String, String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT key, value FROM app_settings")
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| e.to_string())?;
    let mut map = HashMap::new();
    for row in rows {
        let (k, v) = row.map_err(|e| e.to_string())?;
        map.insert(k, v);
    }
    Ok(map)
}

#[cfg(windows)]
fn get_hwnd(window: &tauri::Window) -> Result<*mut std::ffi::c_void, String> {
    use raw_window_handle::HasWindowHandle;
    use raw_window_handle::RawWindowHandle;

    let handle = window.window_handle().map_err(|e| e.to_string())?;
    let raw = handle.as_raw();
    if let RawWindowHandle::Win32(win32) = raw {
        Ok(win32.hwnd.get() as *mut std::ffi::c_void)
    } else {
        Err("Not a Win32 window".to_string())
    }
}

#[tauri::command]
pub fn set_always_on_top(window: tauri::Window, enabled: bool, current_opacity: Option<f64>) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::*;

        let hwnd = get_hwnd(&window)?;
        let insert_after = if enabled { HWND_TOPMOST } else { HWND_NOTOPMOST };

        unsafe {
            // First ensure WS_EX_LAYERED is set BEFORE SetWindowPos
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as isize);

            // SetWindowPos for topmost/not-topmost without resizing or moving
            SetWindowPos(
                hwnd,
                insert_after,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );

            // Re-ensure WS_EX_LAYERED after SetWindowPos (it may strip styles)
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as isize);

            // Reapply alpha
            if let Some(opacity) = current_opacity {
                let alpha = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
                SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
            }
        }
    }

    #[cfg(not(windows))]
    {
        window.set_always_on_top(enabled).map_err(|e| e.to_string())?;
        let _ = current_opacity;
    }

    Ok(())
}

#[tauri::command]
pub fn set_window_opacity(window: tauri::Window, opacity: f64) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::*;

        let hwnd = get_hwnd(&window)?;
        unsafe {
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as isize);
            let alpha = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
            SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
        }
    }
    Ok(())
}

#[tauri::command]
pub fn minimize_to_tray(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::db::Database;
    use rusqlite::params;

    fn setup_db() -> Database {
        Database::new_in_memory().unwrap()
    }

    #[test]
    fn test_set_and_get_settings() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)", params!["theme", "dark"]).unwrap();
        conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)", params!["opacity", "80"]).unwrap();

        let mut stmt = conn.prepare("SELECT key, value FROM app_settings").unwrap();
        let rows: Vec<(String, String)> = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect();

        assert_eq!(rows.len(), 2);
        assert!(rows.contains(&("theme".to_string(), "dark".to_string())));
        assert!(rows.contains(&("opacity".to_string(), "80".to_string())));
    }

    #[test]
    fn test_setting_upsert() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();

        conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)", params!["theme", "light"]).unwrap();
        conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)", params!["theme", "dark"]).unwrap();

        let val: String = conn.query_row("SELECT value FROM app_settings WHERE key=?1", params!["theme"], |row| row.get(0)).unwrap();
        assert_eq!(val, "dark");
    }
}
