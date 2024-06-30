use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};

const SERVER_PORT: u16 = 3000;


#[tokio::main]
async fn main() {
    // Configure certificate and private key used by HTTPS
    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();
    // Create your Axum router and routes here
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));
    // Bind the server to a socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], SERVER_PORT));
    let server = axum_server::bind_rustls(addr, tls_config);
    server.serve(app.into_make_service()).await.unwrap();
}

#[cfg(test)]
mod tests {
    const SERVER_PORT: u16 = 3000;

    #[tokio::test]
    async fn http_client_works() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let req = client.get(format!("https://localhost:{SERVER_PORT}"));
        let resp = req.send().await?;
        println!("{:?}", resp);
        println!("{:?}", resp.text().await?);
        Ok(())
    }

    #[tokio::test]
    async fn http_client_works_with_info() -> Result<(), Box<dyn std::error::Error>> {
        // generate a pair of tls keys
        
        Ok(())
    }
}
