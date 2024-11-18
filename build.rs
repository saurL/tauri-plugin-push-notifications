const COMMANDS: &[&str] = &["push_token", "request_push_permission"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
