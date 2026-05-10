use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn is_auto_start_enabled(app: AppHandle) -> Result<bool, String> {
    is_auto_start_enabled_impl(&app)
}

#[tauri::command]
pub fn set_auto_start_enabled(enabled: bool, app: AppHandle) -> Result<bool, String> {
    set_auto_start_enabled_impl(&app, enabled)?;
    is_auto_start_enabled_impl(&app)
}

pub fn sync_auto_start<R: Runtime>(app: &AppHandle<R>, enabled: bool) -> Result<(), String> {
    set_auto_start_enabled_impl(app, enabled)
}

fn is_auto_start_enabled_impl<R: Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|err| err.to_string())
}

fn set_auto_start_enabled_impl<R: Runtime>(
    app: &AppHandle<R>,
    enabled: bool,
) -> Result<(), String> {
    let autostart = app.autolaunch();

    if enabled {
        autostart.enable().map_err(|err| err.to_string())
    } else {
        autostart.disable().map_err(|err| err.to_string())
    }
}
