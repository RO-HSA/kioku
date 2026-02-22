use std::sync::Mutex;

use super::ipc::DiscordIpcConnection;
use super::model::{DiscordActivity, DiscordPresenceRequest};
use super::sanitize::normalize_client_id;

const DISCORD_CLIENT_ID_ENV: &str = "DISCORD_CLIENT_ID";
const DISCORD_CLIENT_ID_FALLBACK: Option<&str> = Some("1475090774516568215");

pub struct DiscordRpcState {
    client: Mutex<DiscordRpcClient>,
}

impl DiscordRpcState {
    pub fn from_env() -> Self {
        let client_id = std::env::var(DISCORD_CLIENT_ID_ENV)
            .ok()
            .or_else(|| DISCORD_CLIENT_ID_FALLBACK.map(str::to_string));

        Self {
            client: Mutex::new(DiscordRpcClient::new(client_id)),
        }
    }

    pub fn configure_client_id(&self, client_id: Option<String>) -> Result<bool, String> {
        let mut client = self.lock_client()?;
        Ok(client.set_client_id(client_id))
    }

    pub fn set_presence(&self, request: DiscordPresenceRequest) -> Result<(), String> {
        let mut client = self.lock_client()?;
        client.set_presence(request)
    }

    pub fn clear_presence(&self) -> Result<(), String> {
        let mut client = self.lock_client()?;
        client.clear_presence()
    }

    fn lock_client(&self) -> Result<std::sync::MutexGuard<'_, DiscordRpcClient>, String> {
        self.client
            .lock()
            .map_err(|_| "Discord RPC state is unavailable".to_string())
    }
}

#[derive(Default)]
struct DiscordRpcClient {
    client_id: Option<String>,
    connection: Option<DiscordIpcConnection>,
}

impl DiscordRpcClient {
    fn new(client_id: Option<String>) -> Self {
        Self {
            client_id: normalize_client_id(client_id),
            connection: None,
        }
    }

    fn set_client_id(&mut self, client_id: Option<String>) -> bool {
        let normalized = normalize_client_id(client_id);
        if self.client_id != normalized {
            self.connection = None;
            self.client_id = normalized;
        }

        self.client_id.is_some()
    }

    fn set_presence(&mut self, request: DiscordPresenceRequest) -> Result<(), String> {
        let activity = request.sanitize().into_activity();
        self.send_activity(activity)
    }

    fn clear_presence(&mut self) -> Result<(), String> {
        self.send_activity(None)
    }

    fn send_activity(&mut self, activity: Option<DiscordActivity>) -> Result<(), String> {
        let Some(client_id) = self.client_id.clone() else {
            return Err(format!(
                "Discord RPC client id is not configured. Set {}.",
                DISCORD_CLIENT_ID_ENV
            ));
        };

        self.ensure_connection(&client_id)?;

        let retry_activity = activity.clone();
        if let Some(connection) = self.connection.as_mut() {
            if connection.send_activity(activity).is_ok() {
                return Ok(());
            }
        }

        self.connection = None;
        self.ensure_connection(&client_id)?;

        if let Some(connection) = self.connection.as_mut() {
            return connection.send_activity(retry_activity);
        }

        Err("Discord RPC connection is unavailable".to_string())
    }

    fn ensure_connection(&mut self, client_id: &str) -> Result<(), String> {
        if self.connection.is_some() {
            return Ok(());
        }

        self.connection = Some(DiscordIpcConnection::connect(client_id)?);
        Ok(())
    }
}
