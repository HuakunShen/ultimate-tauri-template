use std::{net::SocketAddr, sync::Arc};
use tauri::{AppHandle, Runtime};
use tokio::{net::UdpSocket, sync::Mutex};

pub async fn udp_listener(udp_socket: UdpSocket) -> std::io::Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        match udp_socket.recv_from(&mut buf).await {
            Ok((len, src)) => {
                println!(
                    "Received UDP message: {} from {}",
                    String::from_utf8_lossy(&buf[..len]),
                    src
                );
                let local_addr = udp_socket.local_addr()?;

                let response = format!(
                    "Echo: {}",
                    format!(
                        "Hello, from UDP Server on {}:{}",
                        local_addr.ip(),
                        local_addr.port()
                    )
                );
                match udp_socket.send_to(response.as_bytes(), &src).await {
                    Ok(sent_len) => {
                        println!("Sent response of {} bytes to {}", sent_len, src);
                    }
                    Err(e) => {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("UDP listener error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub async fn start_service_discovery_server(
    app_handle: AppHandle,
    server_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let udp_socket = UdpSocket::bind(&server_addr).await?;
    udp_listener(udp_socket).await?;
    Ok(())
}

pub struct DiscoveryServer {
    app_handle: AppHandle,
    port: u16,
    server_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
}

impl DiscoveryServer {
    pub fn new(app_handle: AppHandle, port: u16) -> Self {
        Self {
            app_handle,
            port,
            server_handle: Arc::new(Mutex::new(None)),
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
        *server_handle = Some(tauri::async_runtime::spawn(async move {
            match start_service_discovery_server(app_handle, server_addr).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Discovery Server start error: {}", e);
                }
            }
        }));
        Ok(())
    }

    pub async fn stop(&self) {
        let mut server_handle = self.server_handle.lock().await;
        if let Some(handle) = server_handle.take() {
            handle.abort();
        }
    }

    pub async fn is_running(&self) -> bool {
        let server_handle = self.server_handle.lock().await;
        server_handle.is_some()
    }
}
