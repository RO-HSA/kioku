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
use crate::services::anime_list_updates::{enqueue_anime_list_update, AnimeListUpdateQueue};
use crate::services::myanimelist::synchronize_myanimelist;
use crate::services::player_detection::detect_playing_anime;

fn process_oauth_callback(app: tauri::AppHandle, url: String) {
    tauri::async_runtime::spawn(async move {
        handle_oauth_callback(app, url).await;
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app: &tauri::AppHandle<_>, args: Vec<String>, _cwd: String| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();

            let oauth_callback = args
                .iter()
                .find(|arg| arg.starts_with("kioku://"))
                .cloned();

            if let Some(oauth_callback) = oauth_callback {
                process_oauth_callback(app.clone(), oauth_callback);
            }
        }));
    }
    
    builder
        .manage(StrongholdKeyState::default())
        .manage(TokenManagerState::default())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_zustand::init())
        .setup(|app: &mut tauri::App<_>| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register("kioku")?;
            }

            use tauri_plugin_deep_link::DeepLinkExt;

            let app_handle = app.handle().clone();
            app.deep_link().on_open_url(move |event| {
                for url in event.urls() {
                    if url.as_str().starts_with("kioku://") {
                        process_oauth_callback(app_handle.clone(), url.to_string());
                    }
                }
            });

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

            app.manage(AnimeListUpdateQueue::new(app.handle().clone()));
            app.zustand().set_autosave(Duration::from_secs(300));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            authorize_myanimelist,
            authorize_provider,
            oauth_request,
            synchronize_myanimelist,
            enqueue_anime_list_update,
            detect_playing_anime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
