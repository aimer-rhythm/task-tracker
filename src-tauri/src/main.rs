#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod reminder;

use db::Database;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn restore_window_state(window: &tauri::WebviewWindow, db: &Database) {
    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(_) => return,
    };
    let mut stmt = match conn.prepare("SELECT key, value FROM app_settings") {
        Ok(s) => s,
        Err(_) => return,
    };
    let rows: std::collections::HashMap<String, String> = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .ok()
        .map(|iter| iter.filter_map(|r| r.ok()).collect())
        .unwrap_or_default();

    let x = rows.get("window_x").and_then(|v| v.parse::<i32>().ok());
    let y = rows.get("window_y").and_then(|v| v.parse::<i32>().ok());
    let w = rows.get("window_width").and_then(|v| v.parse::<u32>().ok());
    let h = rows
        .get("window_height")
        .and_then(|v| v.parse::<u32>().ok());

    if let (Some(x), Some(y)) = (x, y) {
        use tauri::PhysicalPosition;
        let _ = window.set_position(PhysicalPosition::new(x, y));
    }
    if let (Some(w), Some(h)) = (w, h) {
        if w >= 420 && h >= 560 {
            use tauri::PhysicalSize;
            let _ = window.set_size(PhysicalSize::new(w, h));
        }
    }

    #[cfg(windows)]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        use windows_sys::Win32::UI::WindowsAndMessaging::*;

        let hwnd = match window.window_handle() {
            Ok(handle) => match handle.as_raw() {
                RawWindowHandle::Win32(win32) => win32.hwnd.get() as *mut std::ffi::c_void,
                _ => return,
            },
            Err(_) => return,
        };

        let opacity_val = rows
            .get("opacity")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(100.0);
        let always_on_top = rows
            .get("always_on_top")
            .map(|v| v == "true")
            .unwrap_or(false);

        unsafe {
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as isize);

            if always_on_top {
                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
                let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as isize);
            }

            if opacity_val < 100.0 {
                let alpha = ((opacity_val / 100.0).clamp(0.0, 1.0) * 255.0) as u8;
                SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            let database = Database::new(app_dir).expect("failed to init database");
            let db_arc = Arc::new(database);

            app.manage(db_arc.clone());

            // Restore window position/size
            if let Some(window) = app.get_webview_window("main") {
                restore_window_state(&window, &db_arc);
            }

            // System tray
            let quit = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            let mut tray_builder = TrayIconBuilder::new().menu(&menu).tooltip("TaskTracker");

            if let Some(icon) = app.default_window_icon().cloned() {
                tray_builder = tray_builder.icon(icon);
            }

            let _tray = tray_builder
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => std::process::exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Start reminder background loop
            let app_handle = app.handle().clone();
            reminder::start_reminder_loop(app_handle, db_arc);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::task::create_task,
            commands::task::update_task,
            commands::task::delete_task,
            commands::task::get_task,
            commands::task::list_tasks,
            commands::task::add_subtask,
            commands::task::toggle_subtask,
            commands::task::delete_subtask,
            commands::reminder::create_reminder,
            commands::reminder::delete_reminder,
            commands::reminder::toggle_reminder,
            commands::reminder::list_reminders,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::settings::set_always_on_top,
            commands::settings::set_window_opacity,
            commands::settings::minimize_to_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
