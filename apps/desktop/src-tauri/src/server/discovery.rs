use network::discovery::discover_udp_listener;
use std::{net::SocketAddr, sync::Arc};
use tauri::AppHandle;
use tokio::{
    net::UdpSocket,
    sync::{broadcast, Mutex},
};

use crate::constants::SERVICE_NAME;

pub async fn start_service_discovery_server(
    _app_handle: AppHandle,
    server_addr: SocketAddr,
    stop_rx: broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let udp_socket = UdpSocket::bind(&server_addr).await?;
    discover_udp_listener(udp_socket, stop_rx, SERVICE_NAME).await?;
    Ok(())
}

pub struct DiscoveryServer {
    app_handle: AppHandle,
    port: u16,
    server_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl DiscoveryServer {
    pub fn new(app_handle: AppHandle, port: u16) -> Self {
        let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);

        Self {
            app_handle,
            port,
            server_handle: Arc::new(Mutex::new(None)),
            shutdown_tx,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut server_handle = self.server_handle.lock().await;
        if server_handle.is_some() {
            return Err("Discovery Server is already running".into());
        }

        // let server_addr: SocketAddr = format!("[::]:{}", self.port).parse()?;
        let server_addr = SocketAddr::from(([0, 0, 0, 0], self.port));

        let app_handle = self.app_handle.clone();

        let stop_rx = self.shutdown_tx.subscribe();
        *server_handle = Some(tauri::async_runtime::spawn(async move {
            match start_service_discovery_server(app_handle, server_addr, stop_rx).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Discovery Server start error: {}", e);
                }
            }
        }));
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.shutdown_tx.send(())?;
        let mut server_handle = self.server_handle.lock().await;
        server_handle.take();
        // if let Some(handle) = server_handle.take() {
        //     handle.abort();
        // }
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        let server_handle = self.server_handle.lock().await;
        server_handle.is_some()
    }
}
