use std::collections::HashSet;

use server::discovery::{discover, ServiceDiscoverInfo, ServiceDiscoverPayload};
#[tauri::command]
pub async fn discovery_peers(// server: tauri::State<'_, Server>,
) -> Result<HashSet<ServiceDiscoverInfo>, String> {
    let discovered_peers = discover(
        1566,
        &ServiceDiscoverPayload {
            service_name: "desktop".to_string(),
            service_port: 1566,
        },
        4,
    )
    .await
    .map_err(|err| err.to_string())?;
    Ok(discovered_peers)
}
