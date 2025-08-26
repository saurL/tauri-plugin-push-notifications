# Tauri Push Notifications Plugin

This plugin provides a cross-platform abstraction for handling push notifications in a Tauri application, supporting iOS (APNs / Firebase FCM), Android (FCM) and Macos (APNs).

⚠️ This plugin currently work on a fork of tauri , you can use this repo as long as you use the fork or they get merged to tauri ⚠️
## ⚙️ Setup
Modify `lib.rs` to initialize the plugin:

```rust title="src-tauri/src/lib.rs" ins={5-6}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_push_notifications::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
## ⚙️ Configuration
### iOS
#### Using APNs only

Add the following entitlement to your iOS app:

```xml title=src-tauri/entitlements.plist
    <key>aps-environment</key>
    <string>production</string>
```

Here's an example of a full entitlements file (yours may vary, `...` values are placeholders):

```xml title=src-tauri/entitlements.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.application-identifier</key>
    <string>...</string>
    <key>com.apple.developer.team-identifier</key>
    <string>...</string>
    <key>aps-environment</key>
    <string>production</string>
</dict>
</plist>
```

#### Using Firebase Cloud Messaging (FCM) on iOS

If you want to use Firebase in addition to APNs, you must enable the ios-fcm feature in your Cargo.toml:
```toml
[dependencies]
tauri-plugin-push-notifications = { version = "...", features = ["ios-fcm"] }
```
Follow the [Firebase documentation](https://firebase.google.com/docs/cloud-messaging/ios/client?hl=fr) for iOS setup.

#### macOS Configuration

Mac is similar to iOS. You still need an APNS certificate, which can be obtained from the Apple Developer portal. You also need to adjust your
`Info.plist` and entitlements as shown below.

Add the following to your entitlements:

```xml title=src-tauri/entitlements.plist
	<key>com.apple.developer.aps-environment</key>
	<string>production</string>
```

Here's an example of a full entitlements file (yours may vary, `...` values are placeholders):

```xml title=src-tauri/entitlements.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.application-identifier</key>
    <string>...</string>
    <key>com.apple.developer.team-identifier</key>
    <string>...</string>
    <key>com.apple.developer.aps-environment</key>
    <string>production</string>
</dict>
</plist>
```

### Android

#### Using Firebase Cloud Messaging (FCM)

Android uses Firebase Cloud Messaging (FCM) to send push notifications. To configure FCM, you need to create a Firebase project and add the
`google-services.json` file to your project. You can find more information in the [Firebase documentation](https://firebase.google.com/docs/cloud-messaging/android/client).

1. Create a Firebase project

- Go to the [Firebase Console](https://console.firebase.google.com) and create a new project (or use an existing one).
- Enable Firebase Cloud Messaging in your project settings.

2. Register your Android app

- In the Firebase Console, add a new app and select Android.
- Enter your application’s package name (must match your AndroidManifest.xml).
- Download the generated google-services.json configuration file.

3. Add google-services.json to your project

- Place the google-services.json file inside your Android project directory:`android/app/google-services.json`


4. Enable push notifications in your AndroidManifest
- Ensure your AndroidManifest.xml includes the correct permissions and services for FCM:
```xml
<application>
    <!-- Required Firebase messaging service -->
    <service
        android:name="app.tauri.pushNotifications.AppMessagingService"
        android:exported="true">
        <intent-filter>
            <action android:name="com.google.firebase.MESSAGING_EVENT" />
        </intent-filter>
    </service>
</application>
```             
## 🛠️ Rust API

The plugin exposes several event handlers for handling notifications:

`on_message_received`

Triggered when a push notification is received while the app is open (foreground).

`on_notification_clicked`

Triggered when the user clicks on a notification.

`get_opening_notification_data`

Should be used at app startup to check if the app was opened by clicking a notification.

⚠️ thoses fonctions are working on IOS and Android but are currently not handled for Macos ⚠️

### Notification Data Structure

All handlers require a structure that implements the `NotificationDataTrait`.
Currently, this trait is a placeholder for Serialize, Deserialize, and Clone.
### Example
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    pub route: String,
}

impl NotificationDataTrait for Notification {}

app_handle.push_notifications()
    .on_notification_clicked(move |data: Notification| {
        handle_notification(&app_handle_navigation, data);
    });
```
## 🛠️ JavaScript API

On the JavaScript side, the plugin exposes the following functions:
```ts
export declare function getFcmToken(): Promise<string | null>; // Returns the FCM token (only on Android and iOS with ios-fcm enabled).
export declare function getApnsToken(): Promise<string | null>; // Returns the APNs token (only on iOS and Macos).
export declare function requestPushPermission(): Promise<boolean>; // Requests notification permission from the user.
export declare function checkPushPermission(): Promise<boolean>; // Checks if push notifications permission is granted
export declare function onNewFcmToken(handler: (token: string) => void): Promise<PluginListener>; // Registers a callback for receiving new FCM tokens.
```

⚠️ Both `getFcmToken()` and `getApnsToken()` will return errors if:

Called on an unsupported platform, or on iOS, if the ios-fcm feature is not enabled when calling getFcmToken()
