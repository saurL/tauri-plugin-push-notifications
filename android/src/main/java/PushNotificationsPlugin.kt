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
import com.google.firebase.messaging.FirebaseMessaging
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.InvokeArg
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import app.tauri.annotation.Permission
import app.tauri.plugin.JSObject
import app.tauri.plugin.Channel
import android.content.Intent
import java.util.concurrent.atomic.AtomicBoolean

@InvokeArg
class SetEventHandlerArgs {
    lateinit var handler: Channel
}

const val NOTIFICATION_PERMISSION = android.Manifest.permission.POST_NOTIFICATIONS
@TauriPlugin(
  permissions = [
    Permission(strings = [NOTIFICATION_PERMISSION], alias = "notification")
  ]
)
class PushNotificationsPlugin(private val activity: Activity) : Plugin(activity) {
    // Whether permissions have been granted for messaging.
    private val messagingPermissionGranted = AtomicBoolean(false)
    private var channel: Channel? = null
    private var openningNotificationData: JSObject? = null
    override fun load(webView: WebView) {
        super.load(webView)
        val activity = this.activity
        val intent = activity.intent
        val bundle = intent.extras
        val event = JSObject()

        bundle?.keySet()?.forEach { key ->
            val value = bundle.getString(key)
            event.put(key, value)
        }
        this.openningNotificationData = event
        this.channel?.send(event)
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        val bundle = intent.extras
        val event = JSObject()

        bundle?.keySet()?.forEach { key ->
            val value = bundle.getString(key)
            event.put(key, value)
        }
        println("Received notification data: $event")
        this.channel?.send(event)
    }

    @Command
    fun get_fcm_token(invoke: Invoke) {
        FirebaseMessaging.getInstance().token.addOnCompleteListener { task ->
            if (!task.isSuccessful) {
                invoke.reject("Failed to get FCM token", task.exception)
                return@addOnCompleteListener
            }
            val token = task.result
            val result = JSObject()
            result.put("value", token)
            invoke.resolve(result)
        }
    }

    @Command
    fun setEventHandler(invoke: Invoke) {
        val args = invoke.parseArgs(SetEventHandlerArgs::class.java)
        this.channel = args.handler
        invoke.resolve()
    }

    @Command
    fun getOpeningNotificationData(invoke: Invoke) {
        invoke.resolve(this.openningNotificationData)
    }

    override fun onNewToken(token: String) {
        super.onNewToken(token)
        val payload = JSObject()
        payload.put("token", token)
        trigger("new_fcm_token", payload)
    }

}