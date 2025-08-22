const COMMANDS: &[&str] = &["get_fcm_token", "get_apns_token", "registerListener","unregister_listener","requestPermissions","checkPermissions"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
