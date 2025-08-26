use base64::{engine::general_purpose, Engine as _};
use serde::de::DeserializeOwned;
use std::sync::Mutex;
use tauri::Listener;
use tauri::{plugin::PluginApi, AppHandle, Runtime, State};

use crate::models::*;
use crate::PushTokenState;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R>> {
    Ok(PushNotifications(app.clone()))
}

/// Access to the fcm APIs.
pub struct PushNotifications<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PushNotifications<R> {
    /// Requests permission to access push services.
    pub fn request_push_permission(
        &self,
        _state: State<Mutex<PushTokenState>>,
        _payload: PushPermissionRequest,
    ) -> crate::Result<PushPermissionResponse> {
        // desktop platforms don't use this hook
        Ok(PushPermissionResponse { granted: true })
    }

    /// Obtains the most recent push token.
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
    pub fn on_notification_clicked<F: Fn(T) + Send + 'static, T: NotificationDataTrait>(
        &self,
        f: F,
    ) {
        let _ = self
            .0
            .listen("push-notification://notification-clicked", move |event| {
                if let Ok(data) = serde_json::from_str(event.payload()) {
                    f(data)
                }
            });
    }

    pub fn get_opening_notification_data<T: NotificationDataTrait>(
        &self,
    ) -> crate::Result<Option<T>> {
        // Not implemented yet
        Ok(None)
    }
}
