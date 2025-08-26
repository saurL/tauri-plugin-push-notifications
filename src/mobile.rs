use base64::{engine::general_purpose, Engine as _};
use serde::de::DeserializeOwned;
use std::sync::Mutex;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime, State,
};
use log::error;
use crate::models::*;
use crate::PushTokenState;
use tauri::Listener;
use serde::{Serialize, Deserialize};
use tauri::plugin::PermissionState;
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
        match self.check_permissions()?.notification  {
            PermissionState::Granted => Ok(PushPermissionResponse { granted: true }),
            PermissionState::Denied => Ok(PushPermissionResponse { granted: false }),
            _ => match self.request_notification_permission()? {
                PermissionState::Granted => Ok(PushPermissionResponse { granted: true }),
                PermissionState::Denied => Ok(PushPermissionResponse { granted: false }),
                _ => Ok(PushPermissionResponse { granted: false }),
            },
        }
    }

    pub fn get_fcm_token(
        &self,
        state: State<Mutex<PushTokenState>>,
        _payload: PushTokenRequest,
    ) -> crate::Result<PushTokenResponse> {

        self.0
            .run_mobile_plugin("get_fcm_token", _payload)
            .map_err(Into::into)
    }
    pub fn get_apns_token(
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

    #[cfg(all(target_os = "ios", feature = "ios-fcm"))]
    pub fn init_firebase(&self, request: InitFirebaseRequest) -> crate::Result<()> {
            self.0
                .run_mobile_plugin::<()>("initFirebase", request)
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
        T: NotificationDataTrait,
    >(
        &self,
        f: F,
    ) -> crate::Result<()> {
        
        self.register_event_handler::<T>()?;
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

    pub fn on_message_received<
        F: Fn(T) + Send + 'static,
        T: NotificationDataTrait,
    >(
        &self,
        f: F,
    ) -> crate::Result<()> {

        self.register_message_channel::<T>()?;
        let _ = self
            .0
            .app()
            .listen("push-notification://notification-message", move |event| {
                if let Ok(data) = serde_json::from_str(event.payload()) {
                    f(data)
                }
            });
        Ok(())
    }  
    
    pub fn get_opening_notification_data<T: NotificationDataTrait>(
        &self,
    ) -> crate::Result<Option<T>> {
        #[cfg(target_os = "android")]
        {
            self.0
                .run_mobile_plugin::<Option<T>>("getOpeningNotificationData", ())
                .map_err(Into::into)
        }
        #[cfg(target_os = "ios")]
        {
            self.0
                .run_mobile_plugin::<Option<T>>("getOpeningNotificationData", ())
                .map_err(Into::into)
        }
    }

     fn register_event_handler<T: NotificationDataTrait>(
        &self,
    ) -> crate::Result<()> {
        
            use tauri::ipc::{Channel, InvokeResponseBody};
            use tauri::Emitter;
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct EventHandler {
                pub handler: Channel,
            }
            
            let app_handle = self.0.app().clone();
           self.0.run_mobile_plugin::<()>(
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



 fn register_message_channel<T: NotificationDataTrait>(
        &self,
    ) -> crate::Result<()> {
        
            use tauri::ipc::{Channel, InvokeResponseBody};
            use tauri::Emitter;
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct EventHandler {
                pub handler: Channel,
            }
            
            let app_handle = self.0.app().clone();
           self.0.run_mobile_plugin::<()>(
                "setMessageChannel",
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
                            let _ = app_handle.emit("push-notification://notification-message", data);
                        } 

                        Ok(())
                    }),
                },
            )?;
            Ok(())
        
    }


    fn register_event_handler<T: NotificationDataTrait>(
        &self,
    ) -> crate::Result<()> {
        
            use tauri::ipc::{Channel, InvokeResponseBody};
            use tauri::Emitter;
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct EventHandler {
                pub handler: Channel,
            }
            
            let app_handle = self.0.app().clone();
           self.0.run_mobile_plugin::<()>(
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