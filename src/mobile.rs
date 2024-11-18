use base64::{engine::general_purpose, Engine as _};
use serde::de::DeserializeOwned;
use std::sync::Mutex;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime, State,
};

use crate::models::*;
use crate::PushTokenState;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_push_notifications);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("app.tauri.pushNotifications", "PushNotificationsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_push_notifications)?;
    Ok(PushNotifications(handle))
}

/// Access to the push-notifications APIs.
pub struct PushNotifications<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> PushNotifications<R> {
    pub fn get_push_token(
        &self,
        state: State<Mutex<PushTokenState>>,
        _payload: PushTokenRequest,
    ) -> crate::Result<PushTokenResponse> {
        let state = state.lock().unwrap();
        match &state.token {
            Some(token) => {
                let encoded = general_purpose::STANDARD.encode(&token);
                Ok(PushTokenResponse {
                    value: Some(encoded.clone()),
                })
            }
            None => Ok(PushTokenResponse { value: None }),
        }
    }
}
