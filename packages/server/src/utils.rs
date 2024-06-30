use std::path::PathBuf;

use axum_server::tls_rustls::RustlsConfig;

pub async fn get_tls_config() -> Result<RustlsConfig, Box<dyn std::error::Error>> {
    Ok(RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("server.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("server.key"),
    )
    .await?)
}
