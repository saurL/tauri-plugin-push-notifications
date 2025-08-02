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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PermissionResponse {
    pub notification: PermissionState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestPermission {
    notification: bool,
}
