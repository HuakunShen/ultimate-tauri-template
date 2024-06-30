/// This module is responsible for controlling the main server
use super::{
    discovery::udp_listener,
    rest::{get_server_info, web_root},
    utils::shutdown_signal,
};
use axum::{extract::State, routing::get};
use hyper::server::conn::AddrIncoming;
use server::grpc::greeter::hello_world::greeter_server::GreeterServer;
use server::grpc::greeter::MyGreeter;
use std::{net::SocketAddr, sync::Arc};
use tauri::{AppHandle, Runtime};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::Mutex,
};
use tonic::transport::Server as TonicServer;

pub struct Server {
    app_handle: AppHandle,
    port: u16,
    server_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
}

async fn start_server(
    app_handle: AppHandle,
    server_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    // let tcp_listener = TcpListener::bind(&server_addr).await?;
    let greeter = MyGreeter::default();
    // let incoming = AddrIncoming::from_listener(tcp_listener)?;
    let router = TonicServer::builder()
        .add_service(GreeterServer::new(greeter))
        .into_router()
        .route("/", get(web_root))
        .route("/info", get(get_server_info));
    // let server = axum::Server::builder(incoming).serve(router.into_make_service());
    axum::Server::bind(&server_addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

impl Server {
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
            return Err("Server is already running".into());
        }

        // let server_addr: SocketAddr = format!("[::]:{}", self.port).parse()?;
        let server_addr = SocketAddr::from(([0, 0, 0, 1], self.port));

        let app_handle = self.app_handle.clone();
        *server_handle = Some(tauri::async_runtime::spawn(async move {
            match start_server(app_handle, server_addr).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Server start error: {}", e);
                }
            }
        }));
        Ok(())
    }

    // pub fn start_service_discovery_server(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let app_handle = self.app_handle.clone();
    //     // let mut udp_server_join_handle = self.udp_server_join_handle.lock().unwrap();
    //     // if udp_server_join_handle.is_some() {
    //     //     return Err("UDP Server is already running".into());
    //     // }
    //     let server_addr: SocketAddr = format!("[::]:{}", self.port).parse()?;
    //     tauri::async_runtime::spawn(async move {
    //         match start_service_discovery_server(app_handle, server_addr).await {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 eprintln!("UDP Server start error: {}", e);
    //             }
    //         }
    //     });
    //     // *udp_server_join_handle = Some(tauri::async_runtime::spawn(async move {
    //     //     match start_service_discovery_server(app_handle, server_addr).await {
    //     //         Ok(_) => {}
    //     //         Err(e) => {
    //     //             eprintln!("UDP Server start error: {}", e);
    //     //         }
    //     //     }
    //     // }));
    //     Ok(())
    // }

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
