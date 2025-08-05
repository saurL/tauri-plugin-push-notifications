use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{
    plugin::{PermissionState, PluginApi, PluginHandle},
    AppHandle, Runtime, State,
};
use std::marker::PhantomData;

use crate::models::*;
use crate::PushTokenState;
use log::error;
use tauri::Listener;
#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_push_notifications);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned,T: Serialize + Clone + for<'de> Deserialize<'de>>(
    app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R,T>> {
    #[cfg(target_os = "android")]
    {let handle =
        api.register_android_plugin("app.tauri.pushNotifications", "PushNotificationsPlugin")?;
        register_event_handler::<T,R>(app,&handle)?;
        Ok(PushNotifications(handle, PhantomData))
    }
    #[cfg(target_os = "ios")]{
    let handle = api.register_ios_plugin(init_plugin_push_notifications)?;
    Ok(PushNotifications(handle, PhantomData))
}
}

/// Access to the push-notifications APIs.
pub struct PushNotifications<R: Runtime,T: Serialize + Clone + for<'de> Deserialize<'de>>(PluginHandle<R>,PhantomData<T>);

impl<R: Runtime,T: Serialize + Clone + for<'de> Deserialize<'de>> PushNotifications<R,T> {
    /// Requests permission to access push services.
    pub fn request_push_permission(
        &self,
        _state: State<Mutex<PushTokenState>>,
        _payload: PushPermissionRequest,
    ) -> crate::Result<PushPermissionResponse> {
        match self.check_permissions()?.notification {
            PermissionState::Granted => Ok(PushPermissionResponse { granted: true }),
            PermissionState::Denied => Ok(PushPermissionResponse { granted: false }),
            _ => match self.request_notification_permission()? {
                PermissionState::Granted => Ok(PushPermissionResponse { granted: true }),
                PermissionState::Denied => Ok(PushPermissionResponse { granted: false }),
                _ => Ok(PushPermissionResponse { granted: false }),
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

    pub fn on_notification_clicked<
        F: Fn(T) + Send + 'static,
    >(
        &self,
        f: F,
    ) -> crate::Result<()> {
    
        let _ = self
            .0
            .app()
            .listen("push-notification://notification-clicked", move |event| {
                if let Ok(data) = serde_json::from_str(event.payload()) {
                    f(data)
                }
            });

        Ok(())
    }

    
}

fn register_event_handler<T: Serialize + Clone + for<'de> Deserialize<'de>,R: Runtime>(
        app:  &AppHandle<R>,
        handle: &PluginHandle<R>,
    ) -> crate::Result<()> {
        #[cfg(target_os = "android")]
        {
            use tauri::ipc::{Channel, InvokeResponseBody};
            use tauri::Emitter;
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct EventHandler {
                pub handler: Channel,
            }
            let app_handle = app.clone();

           handle.run_mobile_plugin::<()>(
                "setEventHandler",
                EventHandler {
                    handler: Channel::new(move |event| {
                        let data: Option<T> = match event {
                            InvokeResponseBody::Json(payload) => {
                                match serde_json::from_str::<T>(&payload) {
                                    Ok(parsed) => Some(parsed),
                                    Err(e) => {
                                        error!(
                                            "Notification deserialization error: the target class is missing required fields or has incompatible structure.\nError: {}\nPayload: {}",
                                            e,
                                            payload
                                        );
                                        None
                                    }
                                }
                            }
                            _ => None,
                        };
                        if let Some(data) = data {
                            let _ = app_handle.emit("push-notification://notification-clicked", data);
                        } 

                        Ok(())
                    }),
                },
            )?;
            Ok(())
        }
    }