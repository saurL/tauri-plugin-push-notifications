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
    /// Requests permission to access push services.
    pub fn request_push_permission(
        &self,
        _state: State<Mutex<PushTokenState>>,
        _payload: PushPermissionRequest,
    ) -> crate::Result<PushPermissionResponse> {
        match self.check_permissions()? {
            PermissionState::Granted => Ok(PushPermissionResponse { granted: true }),
            PermissionState::Denied => Ok(PushPermissionResponse { granted: false }),
            PermissionState::Prompt => match self.request_notification_permission()? {
                PermissionState::Granted => Ok(PushPermissionResponse { granted: true }),
                PermissionState::Denied => Ok(PushPermissionResponse { granted: false }),
                PermissionState::Prompt => Ok(PushPermissionResponse { granted: false }),
            },
        }
    }

    pub fn get_push_token(
        &self,
        state: State<Mutex<PushTokenState>>,
        _payload: PushTokenRequest,
    ) -> crate::Result<PushTokenResponse> {
        self.0
            .run_mobile_plugin("push_token", _payload)
            .map_err(Into::into)
    }

    pub fn request_notification_permission(&self) -> crate::Result<PermissionState> {
        self.0
            .run_mobile_plugin::<PermissionResponse>(
                "requestPermissions",
                RequestPermission { notification: true },
            )
            .map(|r| r.notification)
            .map_err(Into::into)
    }

    pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())
            .map_err(Into::into)
    }
}
