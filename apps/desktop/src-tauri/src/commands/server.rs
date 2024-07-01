use crate::server::{discovery::DiscoveryServer, main::Server};
use network::Protocol;

#[tauri::command]
pub async fn set_server_protocol(
    server: tauri::State<'_, Server>,
    protocol: Protocol,
) -> Result<(), String> {
    server.set_server_protocol(protocol).await;
    Ok(())
}

#[tauri::command]
pub async fn start_server(server: tauri::State<'_, Server>) -> Result<(), String> {
    server.start().await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_server(server: tauri::State<'_, Server>) -> Result<(), String> {
    server.stop().await.map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn restart_server(server: tauri::State<'_, Server>) -> Result<(), String> {
    server.stop().await.map_err(|err| err.to_string())?;
    server.start().await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn server_is_running(server: tauri::State<'_, Server>) -> Result<bool, String> {
    Ok(server.is_running().await)
}

#[tauri::command]
pub async fn start_service_discovery_server(
    server: tauri::State<'_, DiscoveryServer>,
) -> Result<(), String> {
    server.start().await.map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn stop_service_discovery_server(
    server: tauri::State<'_, DiscoveryServer>,
) -> Result<(), String> {
    server.stop().await.map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn restart_service_discovery_server(
    server: tauri::State<'_, DiscoveryServer>,
) -> Result<(), String> {
    server.stop().await.map_err(|err| err.to_string())?;
    server.start().await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn service_discovery_server_is_running(
    server: tauri::State<'_, DiscoveryServer>,
) -> Result<bool, String> {
    Ok(server.is_running().await)
}
