// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import SwiftRs
import Tauri
import UIKit
import UserNotifications
import WebKit
/* IMPORT PLACEHOLDER */
class InitFirebaseRequest: Decodable {
  let token: Data
}
class SetEventHandlerArgs: Decodable {
  let handler: Channel
}

// MARK: - Firebase Push Notifications Plugin
class PushNotificationsPlugin: Plugin, UNUserNotificationCenterDelegate /* INTERFACE PLACEHOLDER */ {

    // Store reference to pending notification
    private var pendingNotification: [String: Any]?
    private var channel: Channel?
    private var messageChannel: Channel?

    override init() {
        super.init()
    }

    // Called when plugin is loaded
    public override func load(webview: WKWebView) {
         let center = UNUserNotificationCenter.current()
        center.delegate = self
        UIApplication.shared.registerForRemoteNotifications()
    }


    /* FUNCTION PLACEHOLDER */
    

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

    @objc public func setEventHandler(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SetEventHandlerArgs.self)
        self.channel = args.handler
        invoke.resolve()
    }

    @objc public func setMessageChannel(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SetEventHandlerArgs.self)
        self.messageChannel = args.handler
        invoke.resolve()
    }

    @objc public func getOpeningNotificationData(_ invoke: Invoke) {
        invoke.resolve(self.pendingNotification ?? [:])
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



    // MARK: - UNUserNotificationCenterDelegate

    // Called when a notification is delivered to a foreground app.
    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                willPresent notification: UNNotification,
                                withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void) {
        let userInfo = notification.request.content.userInfo
    
        // Transformer [AnyHashable: Any] en [String: Any]
        var dict: [String: Any] = [:]
        for (key, value) in userInfo {
            if let keyStr = key as? String {
                dict[keyStr] = value
            }
        }

        messageChannel?.send(dict)

        completionHandler([]) 
    }

    // Called when a notification is tapped
    func userNotificationCenter(_ center: UNUserNotificationCenter,
                                didReceive response: UNNotificationResponse,
                                withCompletionHandler completionHandler: @escaping () -> Void) {
          let userInfo = response.notification.request.content.userInfo
    
        // Transformer [AnyHashable: Any] en [String: Any]
        var dict: [String: Any] = [:]
        for (key, value) in userInfo {
            if let keyStr = key as? String {
                dict[keyStr] = value
            }
        }
        
        pendingNotification = dict
        channel?.send(dict)
        completionHandler() 
    }

    
}

// MARK: - Plugin initialization
@_cdecl("init_plugin_push_notifications")
func initPlugin() -> Plugin {
    return PushNotificationsPlugin()
}
