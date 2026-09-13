pub mod commands;
pub mod engine;
pub mod error;
pub mod events;
pub mod models;
pub mod net;
pub mod state;
mod tauri_event_sink;
mod window;

use std::sync::Arc;

use tauri::Manager;

use state::AppState;
use tauri_event_sink::TauriEventSink;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::mode::set_mode,
            commands::mode::get_mode,
            commands::direct::list_link_local_addresses,
            commands::direct::get_download_dir,
            commands::direct::reveal_download_folder,
            commands::direct::start_listener,
            commands::direct::connect,
            commands::direct::disconnect,
            commands::transfers::send_file,
        ])
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let download_dir = app_data_dir.join("received");
            std::fs::create_dir_all(&download_dir)?;

            let sink = Box::new(TauriEventSink(app.handle().clone()));
            let app_state = Arc::new(AppState::new(Some(sink), download_dir));
            app.manage(app_state);

            let main_window = app.get_webview_window("main").expect("main window must exist");
            window::init_main_window(&main_window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
