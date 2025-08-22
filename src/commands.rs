use std::sync::Mutex;
use tauri::{command, AppHandle, Manager, Runtime};

use crate::models::*;
use crate::PushNotificationsExt;
use crate::{PushTokenState, Result};

#[cfg(mobile)]
#[command]
pub(crate) async fn request_fcm_token<R: Runtime>(app: AppHandle<R>) -> Result<PushTokenResponse> {
    let state = app.state::<Mutex<PushTokenState>>();

    app.push_notifications()
        .get_fcm_token(state, PushTokenRequest {})
}
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[command]
pub(crate) async fn request_apns_token<R: Runtime>(app: AppHandle<R>) -> Result<PushTokenResponse> {
    let state = app.state::<Mutex<PushTokenState>>();

    app.push_notifications()
        .get_apns_token(state, PushTokenRequest {})
}
