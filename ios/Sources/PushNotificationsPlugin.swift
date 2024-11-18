// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package app.tauri.pushNotifications;

class PushNotificationsPlugin: Plugin {
    public override func load(webview: WKWebView) {}

    @objc public func push_token(_ invoke: Invoke) throws {}
}

@_cdecl("init_plugin_push_notifications")
func initPlugin() -> Plugin {
  return PushNotificationsPlugin()
}
