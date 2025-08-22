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

class InitFirebaseRequest: Decodable {
  let token: Data
}

// MARK: - Firebase Push Notifications Plugin
class PushNotificationsPlugin: Plugin, UNUserNotificationCenterDelegate, MessagingDelegate {

    // Store reference to pending notification
    private var pendingNotification: [AnyHashable: Any]?

    override init() {
        super.init()
    }

    // Called when plugin is loaded
    public override func load(webview: WKWebView) {
    }




    // MARK: - Firebase initialization
    @objc public func initFirebase(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(InitFirebaseRequest.self)

        if FirebaseApp.app() == nil {
            FirebaseApp.configure()
        }
        Messaging.messaging().delegate = self
        Messaging.messaging().setAPNSToken(args.token, type: .unknown)

        if UNUserNotificationCenter.current().delegate == nil {
            UNUserNotificationCenter.current().delegate = self
        }
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
    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                willPresent notification: UNNotification,
                                withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void) {
        completionHandler([.banner, .sound])
    }

    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                didReceive response: UNNotificationResponse,
                                withCompletionHandler completionHandler: @escaping () -> Void) {
        pendingNotification = response.notification.request.content.userInfo
        sendPendingNotificationToJS()
    }

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
