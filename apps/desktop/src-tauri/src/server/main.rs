/// This module is responsible for controlling the main server
use super::{
    model::ServerState,
    rest::{get_server_info, web_root},
};
use axum::routing::get;
use axum_server::tls_rustls::RustlsConfig;
use server::grpc::greeter::hello_world::greeter_server::GreeterServer;
use server::grpc::greeter::MyGreeter;
use server::Protocol;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tauri::AppHandle;
use tokio::sync::Mutex;
use tonic::transport::Server as TonicServer;

async fn start_server(
    protocol: Protocol,
    server_addr: SocketAddr,
    app_handle: AppHandle,
    shtdown_handle: axum_server::Handle,
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
    let svr = match protocol {
        Protocol::Http => {
            axum_server::bind(server_addr)
                .handle(shtdown_handle)
                .serve(combined_router.into_make_service())
                .await
        }
        Protocol::Https => {
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let tls_config = RustlsConfig::from_pem_file(
                manifest_dir.join("self_signed_certs").join("server.crt"),
                manifest_dir.join("self_signed_certs").join("server.key"),
            )
            .await?;
            axum_server::bind_rustls(server_addr, tls_config)
                .handle(shtdown_handle)
                .serve(combined_router.into_make_service())
                .await
        }
    };
    Ok(svr?)
}

pub struct Server {
    pub app_handle: AppHandle,
    pub shtdown_handle: Arc<Mutex<Option<axum_server::Handle>>>,
    pub protocol: Mutex<Protocol>,
    pub port: u16,
    pub server_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
}

impl Server {
    pub fn new(app_handle: AppHandle, port: u16, protocol: Protocol) -> Self {
        Self {
            app_handle,
            protocol: Mutex::new(protocol),
            port,
            server_handle: Arc::new(Mutex::new(None)),
            shtdown_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn set_server_protocol(&self, protocol: Protocol) {
        let mut p = self.protocol.lock().await;
        *p = protocol;
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut server_handle = self.server_handle.lock().await;
        let mut shtdown_handle = self.shtdown_handle.lock().await;
        if server_handle.is_some() {
            return Err("Server is already running".into());
        }
        let server_addr: SocketAddr = format!("[::]:{}", self.port).parse()?;
        let app_handle = self.app_handle.clone();
        let _shutdown_handle = axum_server::Handle::new();
        *shtdown_handle = Some(_shutdown_handle.clone());
        let protocol = self.protocol.lock().await.clone();
        *server_handle = Some(tauri::async_runtime::spawn(async move {
            match start_server(protocol, server_addr, app_handle, _shutdown_handle).await {
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
        let mut shtdown_handle = self.shtdown_handle.lock().await;
        match shtdown_handle.as_ref() {
            Some(handle) => {
                handle.shutdown();
            }
            None => {
                return Err("Server is not running".into());
            }
        }
        shtdown_handle.take();
        // self.shutdown_tx.send(())?;
        server_handle.take();
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        self.server_handle.lock().await.is_some() && self.shtdown_handle.lock().await.is_some()
    }
}
