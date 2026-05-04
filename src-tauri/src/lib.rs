use std::sync::atomic::{AtomicBool, Ordering};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime, WindowEvent};

struct AppExitState(AtomicBool);

fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn hide_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_exit_state = AppExitState(AtomicBool::new(false));

    tauri::Builder::default()
        .manage(app_exit_state)
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "Show Slate", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide Slate", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit Slate", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

            let app_handle = app.handle().clone();
            TrayIconBuilder::new()
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .tooltip("Slate")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "hide" => hide_main_window(app),
                    "quit" => {
                        app.state::<AppExitState>().0.store(true, Ordering::Relaxed);
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(&app_handle);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                let is_quitting = window
                    .state::<AppExitState>()
                    .0
                    .load(Ordering::Relaxed);

                if !is_quitting {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
