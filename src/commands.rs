use std::sync::Mutex;
use tauri::{AppHandle, command, Runtime, Manager};

use crate::models::*;
use crate::{PushTokenState, Result};
use crate::PushNotificationsExt;

#[command]
pub(crate) async fn push_token<R: Runtime>(
    app: AppHandle<R>,
) -> Result<PushTokenResponse> {
    let state = app.state::<Mutex<PushTokenState>>();

    app
        .push_notifications()
        .get_push_token(state, PushTokenRequest {})
}
