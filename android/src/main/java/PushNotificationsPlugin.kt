// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package app.tauri.pushNotifications

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import android.os.Build
import android.webkit.WebView
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.util.concurrent.atomic.AtomicBoolean

@TauriPlugin(
  permissions = [
    Permission(strings = [Manifest.permission.POST_NOTIFICATIONS], alias = "notification")
  ]
)
class PushNotificationsPlugin(private val activity: Activity) : Plugin(activity) {
    // Whether permissions have been granted for messaging.
    private val messagingPermissionGranted = AtomicBoolean(false)

    override fun load(webView: WebView) {
        // nothing to do yet
    }

    @Command
    fun request_push_permission(invoke: Invoke) {
        if (!messagingPermissionGranted.get()) {
            askNotificationPermission()
        }
    }

    @Command
    fun push_token(invoke: Invoke) {
        val ret = JSObject()

        AppMessagingService.obtain().pushToken().let { tokenOrNull ->
            // if the token is `null`, pass `null`, otherwise encode it (if needed)
            // and pass it back to the caller as a string.
            ret.put("value", tokenOrNull)
            invoke.resolve(ret)          
        }
    }
}
