// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_zustand::ManagerExt;

pub mod auth;
pub mod services;
use crate::auth::anilist::{
    AUTHORIZE_URL as ANILIST_AUTHORIZE_URL, CLIENT_ID as ANILIST_CLIENT_ID,
    PROVIDER_ID as ANILIST_PROVIDER_ID, TOKEN_URL as ANILIST_TOKEN_URL,
};
use crate::auth::mal::{
    AUTHORIZE_URL as MAL_AUTHORIZE_URL, CLIENT_ID as MAL_CLIENT_ID, PROVIDER_ID as MAL_PROVIDER_ID,
    TOKEN_URL as MAL_TOKEN_URL,
};
use crate::auth::{
    authorize_anilist, authorize_myanimelist, authorize_provider, handle_oauth_callback,
    init_stronghold_key, oauth_request, ProviderConfig, StrongholdKeyState, TokenManagerState,
};
use crate::services::anilist::{
    fetch_anilist_user_info, search_anilist_media, synchronize_anilist,
};
use crate::services::anime_list_updates::{enqueue_anime_list_update, AnimeListUpdateQueue};
use crate::services::discord_rpc::{
    clear_discord_presence, configure_discord_rpc, set_discord_presence, DiscordRpcState,
};
use crate::services::myanimelist::{
    fetch_myanimelist_user_info, search_myanimelist_media, synchronize_myanimelist,
};
use crate::services::player_detection::{
    configure_playback_observer, detect_playing_anime, get_playback_observer_state,
    start_playback_observer, PlaybackObserverState, SupportedPlayer,
};

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct StoredConfigurationState {
    #[serde(default)]
    detection: StoredDetectionConfig,
    #[serde(default)]
    application: StoredApplicationConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct StoredDetectionConfig {
    #[serde(default)]
    player_detection_enabled: bool,
    #[serde(default)]
    enabled_players: Vec<SupportedPlayer>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct StoredApplicationConfig {
    #[serde(default)]
    start_minimized: bool,
    #[serde(default)]
    minimize_to_tray: bool,
    #[serde(default)]
    close_to_tray: bool,
}

fn read_bootstrap_config(app: &tauri::AppHandle) -> StoredConfigurationState {
    let stored_configuration: Option<StoredConfigurationState> =
        app.zustand().get_or_default("configMenu", "configuration");

    stored_configuration.unwrap_or_default()
}

fn process_oauth_callback(app: tauri::AppHandle, url: String) {
    tauri::async_runtime::spawn(async move {
        handle_oauth_callback(app, url).await;
    });
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.unminimize();
        let _ = main_window.show();
        let _ = main_window.set_focus();
    }
}

fn register_window_event_listeners(app: &tauri::AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        let main_window_for_events = main_window.clone();
        let app_handle_for_events = app.clone();

        main_window.on_window_event(move |event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let config = read_bootstrap_config(&app_handle_for_events);

                if config.application.close_to_tray {
                    api.prevent_close();
                    let _ = main_window_for_events.hide();
                }
            }
            WindowEvent::Resized(_) | WindowEvent::Focused(false) => {
                let is_minimized = main_window_for_events.is_minimized().unwrap_or(false);

                if is_minimized {
                    let config = read_bootstrap_config(&app_handle_for_events);

                    if config.application.minimize_to_tray {
                        let _ = main_window_for_events.hide();
                    }
                }
            }
            _ => {}
        });
    }
}

fn build_tray_icon(app: &tauri::AppHandle) -> tauri::Result<()> {
    let open_i = MenuItem::with_id(app, "open", "Open Kioku", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_i, &quit_i])?;

    TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Kioku")
        .icon(app.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                id: _,
                position: _,
                rect: _,
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
            } => {
                show_main_window(tray.app_handle());
            }
            _ => {}
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                show_main_window(app);
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_single_instance::init(
                |app: &tauri::AppHandle<_>, args: Vec<String>, _cwd: String| {
                    show_main_window(app);

                    let oauth_callback =
                        args.iter().find(|arg| arg.starts_with("kioku://")).cloned();

                    if let Some(oauth_callback) = oauth_callback {
                        process_oauth_callback(app.clone(), oauth_callback);
                    }
                },
            ));
    }

    builder
        .manage(StrongholdKeyState::default())
        .manage(TokenManagerState::default())
        .manage(DiscordRpcState::from_env())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
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

            let bootstrap_config = read_bootstrap_config(&app.handle());
            let main_window_config = app
                .config()
                .app
                .windows
                .iter()
                .find(|window| window.label == "main")
                .or_else(|| app.config().app.windows.first())
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "main window config not found")
                })?;

            let mut main_window_builder =
                WebviewWindowBuilder::from_config(app.handle(), main_window_config)?;
            if bootstrap_config.application.start_minimized {
                main_window_builder = main_window_builder.visible(false);
            }
            let main_window = main_window_builder.build()?;

            register_window_event_listeners(app.handle());

            build_tray_icon(app.handle())?;

            if bootstrap_config.application.start_minimized {
                if bootstrap_config.application.minimize_to_tray {
                    let _ = main_window.hide();
                } else {
                    let _ = main_window.minimize();
                    let _ = main_window.show();
                }
            }

            if let Err(err) = init_stronghold_key(&app.handle()) {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err).into());
            }

            let mal_config = ProviderConfig::new(MAL_CLIENT_ID, MAL_AUTHORIZE_URL, MAL_TOKEN_URL)
                .with_callback_provider_hint(MAL_PROVIDER_ID)
                .with_authorize_param("redirect_uri", "kioku://myanimelist")
                .with_authorize_param("code_challenge_method", "plain")
                .with_token_param("redirect_uri", "kioku://myanimelist");

            if let Err(err) = app
                .state::<TokenManagerState>()
                .register_provider(MAL_PROVIDER_ID, mal_config)
            {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err).into());
            }

            let anilist_config =
                ProviderConfig::new(ANILIST_CLIENT_ID, ANILIST_AUTHORIZE_URL, ANILIST_TOKEN_URL)
                    .with_pkce(false)
                    .with_state(false)
                    .with_authorize_response_type("token")
                    .without_callback_code()
                    .with_callback_access_token_param("access_token")
                    .with_refresh_token(false)
                    .with_default_access_token_ttl_secs(60 * 60 * 24 * 365)
                    .with_callback_provider_hint(ANILIST_PROVIDER_ID);

            if let Err(err) = app
                .state::<TokenManagerState>()
                .register_provider(ANILIST_PROVIDER_ID, anilist_config)
            {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err).into());
            }

            app.manage(AnimeListUpdateQueue::new(app.handle().clone()));
            let observer_config = bootstrap_config.detection;
            app.manage(PlaybackObserverState::new(
                observer_config.player_detection_enabled,
                observer_config.enabled_players,
            ));

            if observer_config.player_detection_enabled {
                start_playback_observer(app.handle().clone());
            }

            app.zustand().set_autosave(Duration::from_secs(300));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            authorize_myanimelist,
            authorize_anilist,
            authorize_provider,
            oauth_request,
            fetch_myanimelist_user_info,
            fetch_anilist_user_info,
            search_myanimelist_media,
            search_anilist_media,
            synchronize_myanimelist,
            synchronize_anilist,
            enqueue_anime_list_update,
            detect_playing_anime,
            get_playback_observer_state,
            configure_playback_observer,
            configure_discord_rpc,
            set_discord_presence,
            clear_discord_presence
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
