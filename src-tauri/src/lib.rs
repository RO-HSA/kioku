// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::time::Duration;
use tauri::Manager;
use tauri_plugin_zustand::ManagerExt;

pub mod auth;
pub mod services;
use crate::auth::{
    authorize_myanimelist, authorize_provider, handle_oauth_callback, init_stronghold_key,
    oauth_request, ProviderConfig, StrongholdKeyState, TokenManagerState,
};
use crate::auth::mal::{
    AUTHORIZE_URL as MAL_AUTHORIZE_URL, CLIENT_ID as MAL_CLIENT_ID,
    PROVIDER_ID as MAL_PROVIDER_ID, TOKEN_URL as MAL_TOKEN_URL,
};
use crate::services::myanimelist::synchronize_myanimelist;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(StrongholdKeyState::default())
        .manage(TokenManagerState::default())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let oauth_callback = args
                .iter()
                .find(|arg| arg.starts_with("kioku://"))
                .cloned();

            if let Some(oauth_callback) = oauth_callback {
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    handle_oauth_callback(app, oauth_callback).await
                });
            }
        }))
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_zustand::init())
        .setup(|app| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register("kioku")?;
            }

            if let Err(err) = init_stronghold_key(&app.handle()) {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err).into());
            }

            let mal_config = ProviderConfig::new(MAL_CLIENT_ID, MAL_AUTHORIZE_URL, MAL_TOKEN_URL)
                .with_authorize_param("redirect_uri", "kioku://myanimelist")
                .with_authorize_param("code_challenge_method", "plain")
                .with_token_param("redirect_uri", "kioku://myanimelist");
            
            if let Err(err) = app
                .state::<TokenManagerState>()
                .register_provider(MAL_PROVIDER_ID, mal_config)
            {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err).into());
            }

            app.zustand().set_autosave(Duration::from_secs(300));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            authorize_myanimelist,
            authorize_provider,
            oauth_request,
            synchronize_myanimelist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
