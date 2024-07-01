use crate::constants::SERVICE_NAME;
use network::discovery::{discover, ServiceDiscoverInfo, ServiceDiscoverPayload};
use std::collections::HashSet;

#[tauri::command]
pub async fn discovery_peers(
    duration_secs: Option<u8>,
) -> Result<HashSet<ServiceDiscoverInfo>, String> {
    let duration_secs = duration_secs.unwrap_or(1);
    let discovered_peers = discover(
        1566,
        &ServiceDiscoverPayload {
            name: SERVICE_NAME.to_string(),
            port: 1566,
        },
        duration_secs,
    )
    .await
    .map_err(|err| err.to_string())?;
    Ok(discovered_peers)
}
