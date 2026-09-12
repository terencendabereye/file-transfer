use std::sync::Arc;

use tauri::State;

use crate::models::ConnectionMode;
use crate::state::AppState;

#[tauri::command]
pub fn set_mode(state: State<'_, Arc<AppState>>, mode: ConnectionMode) {
    *state.mode.lock().unwrap() = mode;
}

#[tauri::command]
pub fn get_mode(state: State<'_, Arc<AppState>>) -> ConnectionMode {
    *state.mode.lock().unwrap()
}
