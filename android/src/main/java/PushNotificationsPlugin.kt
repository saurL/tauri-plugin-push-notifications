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

@TauriPlugin
class PushNotificationsPlugin(private val activity: Activity) : Plugin(activity) {
    // Whether permissions have been granted for messaging.
    private val messagingPermissionGranted = AtomicBoolean(false)

    // Launcher for messaging permission requests.
    private val requestPermissionLauncher = activity.registerForActivityResult(
        ActivityResultContracts.RequestPermission(),
    ) { isGranted: Boolean ->
        messagingPermissionGranted.set(isGranted)
    }

    private fun askNotificationPermission() {
        // This is only necessary for API level >= 33 (TIRAMISU)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) ==
                PackageManager.PERMISSION_GRANTED
            ) {
                // FCM SDK (and your app) can post notifications.
            } else if (activity.shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS)) {
                // TODO: display an educational UI explaining to the user the features that will be enabled
                //       by them granting the POST_NOTIFICATION permission. This UI should provide the user
                //       "OK" and "No thanks" buttons. If the user selects "OK," directly request the permission.
                //       If the user selects "No thanks," allow the user to continue without notifications.
            } else {
                // Directly ask for the permission
                requestPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
            }
        }
    }

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
        AppMessagingService.obtain().pushToken().let { tokenOrNull ->
            // if the token is `null`, pass `null`, otherwise encode it (if needed)
            // and pass it back to the caller as a string.
        }
    }
}
