// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
pub mod commands;
pub mod server;
pub mod setup;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::server::start_server,
            commands::server::stop_server,
            commands::server::restart_server,
            commands::server::server_is_running,
            commands::server::start_service_discovery_server,
            commands::server::stop_service_discovery_server,
            commands::server::restart_service_discovery_server,
            commands::server::service_discovery_server_is_running,
        ])
        .setup(|app| {
            // setup::setup(app);
            app.manage(server::Server::new(app.handle().clone(), 1566));
            app.manage(server::DiscoveryServer::new(app.handle().clone(), 1566));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
