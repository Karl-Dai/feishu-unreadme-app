pub mod core;
pub mod commands;

use std::sync::OnceLock;

use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tracing::field::{Field, Visit};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[derive(Serialize, Clone)]
struct LogEvent {
    level: String,
    text: String,
}

struct EventLayer;

impl<S: tracing::Subscriber> Layer<S> for EventLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _: tracing_subscriber::layer::Context<'_, S>) {
        let mut v = StringVisitor::default();
        event.record(&mut v);
        let level = match *event.metadata().level() {
            tracing::Level::INFO => "info",
            tracing::Level::WARN => "warn",
            tracing::Level::ERROR => "error",
            _ => return,
        };
        if let Some(h) = APP_HANDLE.get() {
            let _ = h.emit("log", LogEvent { level: level.into(), text: v.0 });
        }
    }
}

#[derive(Default)]
struct StringVisitor(String);

impl Visit for StringVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = format!("{value:?}").trim_matches('"').to_string();
        } else {
            self.0.push_str(&format!(" {}={value:?}", field.name()));
        }
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.0 = value.into();
        } else {
            self.0.push_str(&format!(" {}={value}", field.name()));
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EventLayer)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::detect_feishu_install,
            commands::validate_feishu_path,
            commands::get_patch_state,
            commands::get_app_version,
            commands::apply_patch_cmd,
            commands::restore_backup_cmd,
            commands::get_builtin_patches,
            commands::check_app_update,
            commands::trigger_app_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
