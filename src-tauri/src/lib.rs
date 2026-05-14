pub mod core;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::detect_feishu_install,
            commands::validate_feishu_path,
            commands::get_patch_state,
            commands::get_app_version,
            commands::apply_patch_cmd,
            commands::restore_backup_cmd,
            commands::get_builtin_patches,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
