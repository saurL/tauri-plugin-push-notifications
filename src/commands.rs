use std::sync::Mutex;
use tauri::{command, AppHandle, Manager, Runtime};

use crate::models::*;
use crate::PushNotificationsExt;
use crate::{PushTokenState, Result};

#[command]
pub(crate) async fn request_push_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PushPermissionResponse> {
    let state = app.state::<Mutex<PushTokenState>>();
    app.push_notifications()
        .request_push_permission(state, PushPermissionRequest {})
}
#[command]
pub(crate) async fn push_token<R: Runtime>(app: AppHandle<R>) -> Result<PushTokenResponse> {
    let state = app.state::<Mutex<PushTokenState>>();

    app.push_notifications()
        .get_push_token(state, PushTokenRequest {})
}
