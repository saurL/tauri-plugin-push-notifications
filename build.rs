use std::fs;

const COMMANDS: &[&str] = &["get_fcm_token", "get_apns_token", "registerListener","unregister_listener","requestPermissions","checkPermissions"];
fn main() {

    // Vérifier si la feature est activée
if std::env::var("CARGO_FEATURE_IOS_FCM").is_ok() {
        println!("cargo:warning=Feature ios-fcm activée !");

        // Fonction Swift à insérer
        let function = r#"// MARK: - Firebase initialization
        @objc public func initFirebase(_ invoke: Invoke) throws {
            let args = try invoke.parseArgs(InitFirebaseRequest.self)

            if FirebaseApp.app() == nil {
                FirebaseApp.configure()
            }
            Messaging.messaging().delegate = self
            Messaging.messaging().apnsToken = args.token
            invoke.resolve()
        }

        // MARK: - JS Method: Get FCM token
        @objc public func get_fcm_token(_ invoke: Invoke) throws {
            Messaging.messaging().token { token, error in
                if let error = error {
                    invoke.reject("Error fetching FCM registration token: \(error.localizedDescription)")
                } else if let token = token {
                    invoke.resolve(["value": token])
                }
                else {
                    invoke.reject("FCM registration token is nil")
                }
            }         
        }

            // MARK: - Firebase Messaging Delegate
        func messaging(_ messaging: Messaging, didReceiveRegistrationToken fcmToken: String?) {
            if let fcmToken = fcmToken {
                trigger("new_fcm_token", data: ["token": fcmToken])
            }
        }
    "#;

        // Replace si premium activé
        write_features_file(
            "ios/Sources/PushNotificationsPlugin.swift",
            "/* FUNCTION PLACEHOLDER */",
            function,
        );

        // Imports Swift à insérer
        let import_statement = r#"import FirebaseCore
    import FirebaseMessaging"#;

        write_features_file(
            "ios/Sources/PushNotificationsPlugin.swift",
            "/* IMPORT PLACEHOLDER */",
            import_statement,
        );

        let interface = r#",MessagingDelegate"#;

        write_features_file(
            "ios/Sources/PushNotificationsPlugin.swift",
            "/* INTERFACE PLACEHOLDER */",
            interface,
        );

        // Dépendances à ajouter dans Package.swift
        let dependencies = r#".package(
            url: "https://github.com/firebase/firebase-ios-sdk.git",
            .upToNextMajor(from: "12.1.0")
        ),"#;

        write_features_file(
            "ios/Package.swift",
            "    /* DEPENDENCIES PLACEHOLDER */",
            dependencies,
        );

        // Produits à ajouter dans Package.swift
        let product = r#"        .product(name: "FirebaseCore", package: "firebase-ios-sdk"),
            .product(name: "FirebaseMessaging", package: "firebase-ios-sdk")"#;

        write_features_file(
            "ios/Package.swift",
            "/* PRODUCT PLACEHOLDER */",
            product,
        );
    }
    else {
        println!("cargo:warning=Feature ios-fcm désactivée !");
    }

    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}

fn write_features_file(path: &str, string_to_replace: &str, replacement: &str) {
    let content = fs::read_to_string(path)
        .expect("Impossible de lire le fichier Swift");
    let new_content = content.replace(string_to_replace, replacement);
    fs::write(path, new_content).expect("Impossible d'écrire dans le fichier Swift");
}
