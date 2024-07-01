/// This example demonstrate how to listen and discover services within local network
use server::discovery::{discover, discover_udp_listener, ServiceDiscoverPayload};
use std::net::SocketAddr;
use tokio::{net::UdpSocket, sync::broadcast};

const SERVICE_NAME: &str = "ultimate-tauri";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* -------------------------------------------------------------------------- */
    /*                         Start UDP Discovery Server                         */
    /* -------------------------------------------------------------------------- */
    let server_addr = SocketAddr::from(([0, 0, 0, 0], 1566));
    let udp_socket = UdpSocket::bind(&server_addr).await?;
    let (_shutdown_tx, shutdown_rx) = broadcast::channel(1);

    tokio::spawn(async move {
        discover_udp_listener(udp_socket, shutdown_rx, SERVICE_NAME)
            .await
            .unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    /* -------------------------------------------------------------------------- */
    /*                             Run discover client                            */
    /* -------------------------------------------------------------------------- */
    let discovered_services_res = discover(
        1566,
        &ServiceDiscoverPayload {
            name: SERVICE_NAME.to_string(),
            port: 1566,
        },
        1,
    )
    .await;
    match discovered_services_res {
        Ok(discovered_services) => {
            println!("Discovered Peers: {:?}", discovered_services);
            // assert_eq!(discovered_services.len(), 0);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            // assert!(false);
        }
    };
    Ok(())
}
