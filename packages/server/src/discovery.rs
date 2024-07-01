use serde::{Deserialize, Serialize};
use std::{collections::HashSet, net::SocketAddr};
use tokio::{
    net::UdpSocket,
    sync::broadcast,
    time::{self, Duration},
};

#[derive(Serialize, Deserialize)]
pub struct ServiceDiscoverPayload {
    pub name: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct ServiceDiscoverInfo {
    pub ip: String,
    pub port: u16,
}

async fn collect_responses(
    socket: &UdpSocket,
    response_timeout_secs: u8,
    service_name: &str,
) -> Result<HashSet<ServiceDiscoverInfo>, Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    let mut discovered_services = HashSet::new();

    let end_time = time::Instant::now() + Duration::from_secs(response_timeout_secs as u64);
    while time::Instant::now() < end_time {
        let remaining_time = end_time - time::Instant::now();
        match time::timeout(remaining_time, socket.recv_from(&mut buf)).await {
            Ok(Ok((len, addr))) => {
                let received_msg = std::str::from_utf8(&buf[..len]);
                match received_msg {
                    Ok(msg) => match serde_json::from_str::<ServiceDiscoverPayload>(msg) {
                        Ok(payload) => {
                            if payload.name == service_name {
                                discovered_services.insert(ServiceDiscoverInfo {
                                    ip: addr.ip().to_string(),
                                    port: addr.port(),
                                });
                            }
                        }
                        Err(e) => eprintln!("Failed to parse payload: {:?}", e),
                    },
                    Err(e) => eprintln!("Failed to parse message: {:?}", e),
                }
            }
            Ok(Err(e)) => eprintln!("Failed to receive message: {:?}", e),
            Err(_) => break, // Timeout expired, break the loop
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
    let discovered_services = collect_responses(&socket, duration_secs, &payload.name).await?;
    Ok(discovered_services)
}

pub async fn discover_udp_listener(
    udp_socket: UdpSocket,
    mut stop_rx: broadcast::Receiver<()>,
    service_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    println!("Starting UDP listener");
    loop {
        tokio::select! {
            _ = stop_rx.recv() => {
                break;
            }
            result = udp_socket.recv_from(&mut buf) => {
                match result {
                    Ok((len, src)) => {
                        let received_msg = std::str::from_utf8(&buf[..len]);
                        match received_msg {
                            Ok(msg) => {
                                let payload: ServiceDiscoverPayload = serde_json::from_str(msg)?;
                                if payload.name != service_name {
                                    continue;
                                } else {
                                    let response = ServiceDiscoverPayload{
                                        name: service_name.to_string(),
                                        port: payload.port,
                                    };
                                    let response = serde_json::to_string(&response)?;
                                    match udp_socket.send_to(response.as_bytes(), &src).await {
                                        Ok(_sent_len) => {}
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
    Ok(())
}
