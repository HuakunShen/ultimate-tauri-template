use axum::routing::get;
use axum_server::tls_rustls::RustlsConfig;
use serde::Serialize;
use server::grpc::greeter::hello_world::greeter_server::GreeterServer;
use server::grpc::greeter::MyGreeter;
use server::utils::get_tls_config;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::UdpSocket;
use tonic::transport::Server as TonicServer;

const SERVER_PORT: u16 = 1566;

async fn web_root() -> &'static str {
    "Hello World!"
}

#[derive(Serialize)]
struct ServerInfo {
    server_name: String,
    server_version: String,
}

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

async fn get_server_info() -> axum::Json<ServerInfo> {
    axum::Json(ServerInfo {
        server_name: "axum".to_string(),
        server_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn udp_listener(udp_socket: UdpSocket) -> std::io::Result<()> {
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
                    "Echo: Hello, from UDP Server on {}:{}",
                    local_addr.ip(),
                    local_addr.port()
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr: SocketAddr = format!("[::]:{SERVER_PORT}").parse()?;
    let udp_socket = UdpSocket::bind(&server_addr).await?;
    tokio::task::spawn(udp_listener(udp_socket));
    let greeter = MyGreeter::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            server::grpc::greeter::hello_world::FILE_DESCRIPTOR_SET,
        )
        .build()
        .unwrap();
    let grpc_router = TonicServer::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(reflection_service)
        .into_router()
        .route("/", get(web_root))
        .route("/info", get(get_server_info));
    // let tls_config = get_tls_config().await?;
    // let ports = Ports {
    //     http: 7878,
    //     https: 3000,
    // };
    // let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    // axum_server::bind_rustls(addr, tls_config)
    //     .serve(grpc_router.into_make_service())
    //     .await?;
    axum::Server::bind(&server_addr)
        .serve(grpc_router.into_make_service())
        .await?;
    Ok(())
}

/// To run this test, start server first by running `cargo run --bin server`, then run `cargo test --bin server`
#[cfg(test)]
mod tests {
    use server::grpc::greeter::hello_world::greeter_client::GreeterClient;
    use server::grpc::greeter::hello_world::HelloRequest;
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use tokio::net::UdpSocket;

    const SERVER_PORT: u16 = 1566;

    #[tokio::test]
    async fn grpc_client_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GreeterClient::connect(format!("http://[::1]:{SERVER_PORT}")).await?;
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });
        let response = client.say_hello(request).await?;
        println!("GRPC RESPONSE={:?}", response);
        assert_eq!(response.into_inner().message, "Hello Tonic!");
        Ok(())
    }

    #[tokio::test]
    async fn http_client_works() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(format!("http://[::1]:{SERVER_PORT}"))
            .await?
            .text()
            .await?;
        println!("{resp}");
        assert_eq!(resp, "Hello World!");
        let resp = reqwest::get(format!("http://[::1]:{SERVER_PORT}/info"))
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        assert_eq!(resp["server_name"], "axum");
        println!("{resp:#?}");

        Ok(())
    }

    #[tokio::test]
    async fn udp_service_discovery_works() -> Result<(), Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;

        // Enable broadcast
        socket.set_broadcast(true)?;

        // Define the broadcast address and port
        let broadcast_addr: SocketAddr = format!("255.255.255.255:{SERVER_PORT}").parse()?;
        let message = b"Hello, network!";

        // Send the message
        socket.send_to(message, &broadcast_addr).await?;
        println!("Broadcast message sent to {}", broadcast_addr);
        let mut buf = vec![0; 1024];
        match socket.recv_from(&mut buf).await {
            Ok((len, src)) => {
                let received_message = String::from_utf8_lossy(&buf[..len]);
                println!("Received response: {} from {}", received_message, src);
            }
            Err(e) => {
                eprintln!("Error receiving response: {}", e);
            }
        }
        Ok(())
    }
}
