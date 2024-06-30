use serde::Serialize;

#[derive(Serialize)]
pub struct ServerInfo {
    pub service_name: String,
    pub service_version: String,
}



#[derive(Clone)]
struct ServerState {
    // that holds some api specific state
    // app_handle: AppHandle,
}
