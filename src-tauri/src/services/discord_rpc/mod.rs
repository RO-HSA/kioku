mod client;
mod ipc;
mod model;
mod sanitize;

use tauri::Manager;

pub use client::DiscordRpcState;
pub use model::{DiscordPresenceButton, DiscordPresenceRequest};

#[tauri::command]
pub fn configure_discord_rpc(
    client_id: Option<String>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    app.state::<DiscordRpcState>()
        .configure_client_id(client_id)
}

#[tauri::command]
pub fn set_discord_presence(
    request: DiscordPresenceRequest,
    app: tauri::AppHandle,
) -> Result<(), String> {
    app.state::<DiscordRpcState>().set_presence(request)
}

#[tauri::command]
pub fn clear_discord_presence(app: tauri::AppHandle) -> Result<(), String> {
    app.state::<DiscordRpcState>().clear_presence()
}
