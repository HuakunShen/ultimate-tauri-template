use serde::{Deserialize, Serialize};
use std::{collections::HashSet, net::SocketAddr};
use tokio::{
    net::UdpSocket,
    sync::broadcast,
    time::{self, Duration},
};

const SERVICE_NAME: &str = "ultimate-tauri";

#[derive(Serialize, Deserialize)]
pub struct ServiceDiscoverPayload {
    pub service_name: String,
    pub service_port: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct ServiceDiscoverInfo {
    pub addr: SocketAddr,
    pub service_port: u16,
}

async fn collect_responses(
    socket: &UdpSocket,
    response_timeout_secs: u8,
) -> Result<HashSet<ServiceDiscoverInfo>, Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    let mut discovered_services = HashSet::new();

    let end_time = time::Instant::now() + Duration::from_secs(response_timeout_secs as u64);
    while time::Instant::now() < end_time {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let received_msg = std::str::from_utf8(&buf[..len]);
                match received_msg {
                    Ok(msg) => {
                        let payload: ServiceDiscoverPayload = serde_json::from_str(msg)?;
                        if payload.service_name != SERVICE_NAME {
                            continue;
                        }
                        discovered_services.insert(ServiceDiscoverInfo {
                            service_port: payload.service_port,
                            addr,
                        });
                    }
                    Err(e) => eprintln!("Failed to parse message: {:?}", e),
                };
            }
            Err(e) => eprintln!("Failed to receive message: {:?}", e),
        }
    }

    Ok(discovered_services)
}

pub async fn discover(
    port: u16,
    payload: &ServiceDiscoverPayload,
    duration_secs: u8,
) -> Result<HashSet<ServiceDiscoverInfo>, Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;
    let broadcast_addr: std::net::SocketAddr = format!("255.255.255.255:{port}").parse()?;
    let payload_str = serde_json::to_string(&payload)?;
    let message = payload_str.as_bytes();
    socket.send_to(message, &broadcast_addr).await?;
    let discovered_services = collect_responses(&socket, duration_secs).await?;
    Ok(discovered_services)
}

pub async fn discover_udp_listener(
    udp_socket: UdpSocket,
    mut stop_rx: broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    loop {
        tokio::select! {
            _ = stop_rx.recv() => {
                println!("Stopping UDP listener");
                break;
            }
            result = udp_socket.recv_from(&mut buf) => {
                match result {
                    Ok((len, src)) => {
                        println!(
                            "Received UDP message: {} from {}",
                            String::from_utf8_lossy(&buf[..len]),
                            src
                        );
                        let received_msg = std::str::from_utf8(&buf[..len]);
                        match received_msg {
                            Ok(msg) => {
                                let payload: ServiceDiscoverPayload = serde_json::from_str(msg)?;
                                if payload.service_name != SERVICE_NAME {
                                    continue;
                                } else {
                                    let local_addr = udp_socket.local_addr()?;
                                    let response = ServiceDiscoverInfo{
                                        addr: local_addr,
                                        service_port: payload.service_port,
                                    };
                                    let response = serde_json::to_string(&response)?;
                                    match udp_socket.send_to(response.as_bytes(), &src).await {
                                        Ok(sent_len) => {
                                            println!("Sent response of {} bytes to {}", sent_len, src);
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to send response: {}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("Failed to parse message: {:?}", e),
                        };
                    }
                    Err(e) => {
                        eprintln!("UDP listener error: {}", e);
                        break;
                    }
                }
            }
        }
    }
    println!("UDP listener stopped");
    Ok(())
}
