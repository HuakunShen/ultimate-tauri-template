/// This module is responsible for controlling the main server
use super::{
    model::ServerState,
    rest::{get_server_info, web_root},
    utils::shutdown_signal,
};
use axum::routing::get;
use server::grpc::greeter::hello_world::greeter_server::GreeterServer;
use server::grpc::greeter::MyGreeter;
use std::{net::SocketAddr, sync::Arc};
use tauri::AppHandle;
use tokio::sync::{broadcast, Mutex};
use tonic::transport::Server as TonicServer;

async fn start_server(
    app_handle: AppHandle,
    server_addr: SocketAddr,
    shutdown_rx: broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let greeter = MyGreeter::default();
    let server_state = ServerState { app_handle };
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            server::grpc::greeter::hello_world::FILE_DESCRIPTOR_SET,
        )
        .build()
        .unwrap();
    let grpc_router = TonicServer::builder()
        .add_service(reflection_service)
        .add_service(GreeterServer::new(greeter))
        .into_router();
    let rest_router = axum::Router::new()
        .route("/", get(web_root))
        .route("/info", get(get_server_info))
        .with_state(server_state);
    let combined_router = axum::Router::new().merge(grpc_router).merge(rest_router);

    axum::Server::bind(&server_addr)
        .serve(combined_router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(shutdown_rx))
        .await?;
    Ok(())
}

pub struct Server {
    app_handle: AppHandle,
    port: u16,
    server_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl Server {
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
            return Err("Server is already running".into());
        }

        // let server_addr: SocketAddr = format!("[::]:{}", self.port).parse()?;
        let server_addr: SocketAddr = format!("[::]:{}", self.port).parse()?;

        let shutdown_rx = self.shutdown_tx.subscribe();
        let app_handle = self.app_handle.clone();
        *server_handle = Some(tauri::async_runtime::spawn(async move {
            match start_server(app_handle, server_addr, shutdown_rx).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Server start error: {}", e);
                }
            }
        }));
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut server_handle = self.server_handle.lock().await;
        self.shutdown_tx.send(())?;
        server_handle.take();
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        let server_handle = self.server_handle.lock().await;
        server_handle.is_some()
    }
}
