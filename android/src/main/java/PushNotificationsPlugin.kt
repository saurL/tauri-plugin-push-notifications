package app.tauri.pushNotifications

import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@TauriPlugin
class PushNotificationsPlugin(private val activity: Activity) : Plugin(activity) {
    override fun load(webView: WebView) {
        // nothing to do yet
    }

    @Command
    fun push_token(invoke: Invoke) {}
}
