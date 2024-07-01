// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
pub mod commands;
pub mod constants;
pub mod server;
pub mod utils;
pub use tauri_plugin_log::fern::colors::ColoredLevelConfig;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets(utils::log::get_log_targets())
                .level(utils::log::get_log_level())
                .with_colors(ColoredLevelConfig::default())
                .max_file_size(10_000_000) // max 10MB
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        // record.target(),
                        record.level(),
                        message
                    ))
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::server::start_server,
            commands::server::stop_server,
            commands::server::restart_server,
            commands::server::server_is_running,
            commands::server::start_service_discovery_server,
            commands::server::stop_service_discovery_server,
            commands::server::restart_service_discovery_server,
            commands::server::service_discovery_server_is_running,
            commands::server::set_server_protocol,
            commands::discovery::discovery_peers,
        ])
        .setup(|app| {
            // setup::setup(app);
            app.manage(server::Server::new(
                app.handle().clone(),
                1566,
                ::server::Protocol::Http, // default to http
            ));
            app.manage(server::DiscoveryServer::new(app.handle().clone(), 1566));
            #[cfg(debug_assertions)] // only inclupde this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
