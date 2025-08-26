use std::fs;

const COMMANDS: &[&str] = &["get_fcm_token", "get_apns_token", "registerListener","unregister_listener","requestPermissions","checkPermissions"];
fn main() {

    // Vérifier si la feature est activée
    #[cfg(feature = "ios-fcm")]
    {
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
