use super::model::ServerInfo;

/// This file contains REST API endpoints

pub async fn web_root() -> &'static str {
    "Hello World!"
}

pub async fn get_server_info() -> axum::Json<ServerInfo> {
    axum::Json(ServerInfo {
        service_name: "axum".to_string(),
        service_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
