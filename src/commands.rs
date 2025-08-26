use std::sync::Mutex;
use tauri::{command, AppHandle, Manager, Runtime};

use crate::models::*;
use crate::PushNotificationsExt;
use crate::{PushTokenState, Result};

#[command]
pub(crate) async fn get_fcm_token<R: Runtime>(app: AppHandle<R>) -> Result<PushTokenResponse> {
    if cfg!(all(target_os = "ios", not(feature = "ios-fcm"))) {
        return Err(
            "Firebase is not enabled. Please enable the 'ios-fcm' feature to use FCM on iOS."
                .into(),
        );
    }
    let state = app.state::<Mutex<PushTokenState>>();

    app.push_notifications()
        .get_fcm_token(state, PushTokenRequest {})
}
#[command]
pub(crate) async fn get_apns_token<R: Runtime>(app: AppHandle<R>) -> Result<PushTokenResponse> {
    if cfg!(not(any(target_os = "ios", target_os = "android"))) {
        return Err("APNs is not available , you are not using iOS or Macos.".into());
    }
    let state = app.state::<Mutex<PushTokenState>>();

    app.push_notifications()
        .get_apns_token(state, PushTokenRequest {})
}
