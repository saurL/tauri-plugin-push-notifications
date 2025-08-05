use serde::{Deserialize, Serialize};
use tauri::plugin::PermissionState;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushTokenRequest {
    // Nothing at this time.
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushTokenResponse {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushPermissionRequest {
    // Nothing at this time.
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushPermissionResponse {
    pub granted: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub notification: PermissionState,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermission {
    pub notification: bool,
}

pub trait NotificationData:
    Serialize + Clone + for<'de> Deserialize<'de> + Send + Sync + 'static
{
}
