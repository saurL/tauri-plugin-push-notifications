const COMMANDS: &[&str] = &["get_fcm_token", "get_apns_token", "request_push_permission","registerListener","unregister_listener"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
