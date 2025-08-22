// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import SwiftRs
import Tauri
import UIKit
import UserNotifications
import WebKit
import FirebaseCore
import FirebaseMessaging

// MARK: - Firebase Push Notifications Plugin

class PushNotificationsPlugin: Plugin, UNUserNotificationCenterDelegate, MessagingDelegate {

    // Store reference to previous delegate to chain calls
    private var pendingNotification: [AnyHashable: Any]?

    override init() {
        super.init()
    }

    // Called when plugin is loaded
    public override func load(webview: WKWebView) {
        
        // Initialize Firebase if needed
        if FirebaseApp.app() == nil {
         //   FirebaseApp.configure()
        }

        // Chain previous delegate to avoid breaking other notifications
        Messaging.messaging().delegate = self
    }

    // MARK: - JS Method: Get FCM token
    @objc public func get_fcm_token(_ invoke: Invoke) throws {
        if let token = Messaging.messaging().fcmToken {
            invoke.resolve(token)
        } else {
            invoke.reject("FCM token not available yet")
        }
    }

    // MARK: - Request notification permissions
    @objc override public func requestPermissions(_ invoke: Invoke) {
        UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .badge, .sound]) { granted, error in
            if let error = error {
                invoke.reject(error.localizedDescription)
                return
            }
            invoke.resolve(["permissionState": granted ? "granted" : "denied"])
        }
    }

    // MARK: - Check current permission status
    @objc override public func checkPermissions(_ invoke: Invoke) {
        UNUserNotificationCenter.current().getNotificationSettings { settings in
            let permission: String
            switch settings.authorizationStatus {
            case .authorized, .ephemeral, .provisional:
                permission = "granted"
            case .denied:
                permission = "denied"
            case .notDetermined:
                permission = "prompt"
            @unknown default:
                permission = "prompt"
            }
            invoke.resolve(["permissionState": permission])
        }
    }

    // MARK: - Firebase Messaging Delegate
    func messaging(_ messaging: Messaging, didReceiveRegistrationToken fcmToken: String?) {
        if let fcmToken = fcmToken {
            trigger("new_fcm_token", data: ["token": fcmToken])
        }

    }

    // MARK: - UNUserNotificationCenterDelegate
    /*
    // Called when notification is received in foreground
    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                willPresent notification: UNNotification,
                                withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void) {
        // Send data to JS

        // Present the notification as banner + sound
        completionHandler([.banner, .sound])

        // Call previous delegate if exists
        
    }

    // Called when notification is tapped (app in background or closed)
    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                didReceive response: UNNotificationResponse,
                                withCompletionHandler completionHandler: @escaping () -> Void) {

        // Store pending notification in case JS is not ready
        pendingNotification = response.notification.request.content.userInfo
        sendPendingNotificationToJS()


    }



    // Send pending notification if app just launched
    private func sendPendingNotificationToJS() {
        guard let pending = pendingNotification else { return }
        pendingNotification = nil
    }
    */
}

// MARK: - Plugin initialization
@_cdecl("init_plugin_push_notifications")
func initPlugin() -> Plugin {
    return PushNotificationsPlugin()
}