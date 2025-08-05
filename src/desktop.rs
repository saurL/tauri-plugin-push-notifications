use crate::models::*;
use crate::PushTokenState;
use base64::{engine::general_purpose, Engine as _};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::Mutex;
use tauri::Listener;
use tauri::{plugin::PluginApi, AppHandle, Runtime, State};

pub fn init<R: Runtime, C: DeserializeOwned, T: NotificationData>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R, T>> {
    Ok(PushNotifications(app.clone(), PhantomData))
}

/// Access to the push-notifications APIs.
pub struct PushNotifications<R: Runtime, T: Serialize + Clone + for<'de> Deserialize<'de>>(
    AppHandle<R>,
    PhantomData<T>,
);

impl<R: Runtime, T: Serialize + Clone + for<'de> Deserialize<'de>> PushNotifications<R, T> {
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

    pub fn on_notification_clicked<F: Fn(T) + Send + 'static>(&self, f: F) {
        let _ = self
            .0
            .listen("push-notification://notification-clicked", move |event| {
                if let Ok(data) = serde_json::from_str(event.payload()) {
                    f(data)
                }
            });
    }
}
