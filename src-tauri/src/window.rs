use tauri::WebviewWindow;

/// Applies the Mica backdrop to the main window and shows it. Called once from the
/// setup hook. Window starts hidden (`"visible": false` in tauri.conf.json) so the
/// effect is in place before the first paint — otherwise there's a visible flash of
/// an opaque window before Mica kicks in.
pub fn init_main_window(window: &WebviewWindow) {
    let dark_mode = window
        .theme()
        .map(|theme| theme == tauri::Theme::Dark)
        .unwrap_or(true);

    if let Err(err) = window_vibrancy::apply_mica(window, Some(dark_mode)) {
        // Mica is Windows 11+ only; on Windows 10 or if it otherwise fails, fall back
        // to a plain opaque window rather than leaving it invisible.
        eprintln!("apply_mica failed, falling back to opaque window: {err}");
    }

    let _ = window.show();
}
