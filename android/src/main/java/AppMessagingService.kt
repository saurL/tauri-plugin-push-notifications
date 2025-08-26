// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package app.tauri.pushNotifications

import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage
import java.util.concurrent.atomic.AtomicReference

// Holds the singleton instance of the messaging service.
private val messagingServiceSingleton = AtomicReference<AppMessagingService>(null)

class AppMessagingService : FirebaseMessagingService() {
    // Most recent push token.
    private val messagingToken = AtomicReference<String>(null)

    override fun onCreate() {
        check(messagingServiceSingleton.get() == null) { "AppMessagingService already active" }
        messagingServiceSingleton.set(this)
        super.onCreate()
    }

    override fun onDestroy() {
        messagingServiceSingleton.set(null)
        super.onDestroy()
    }

    internal fun pushToken(): String? =
        messagingToken.get()

    internal companion object {
        fun obtain(): AppMessagingService {
            return requireNotNull(messagingServiceSingleton.get()) {
                "AppMessagingService has not instantiated"
            }
        }
    }

    override fun onMessageReceived(remoteMessage: RemoteMessage) {
            val event = JSObject()
            for ((key, value) in remoteMessage.data) {
                event.put(key, value)
            }
            PushNotificationsBridge.sendEvent(event)
        }

    override fun onNewToken(token: String) {
        super.onNewToken(token)
        PushNotificationsPlugin.newTokenEvent(token)
    }
}

