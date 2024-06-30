use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use server::utils::{redirect_http_to_https, Ports};
use std::{net::SocketAddr, path::PathBuf};

#[tokio::main]
async fn main() {
    // Configure certificate and private key used by HTTPS
    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("server.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("server.key"),
    )
    .await
    .unwrap();
    let ports = Ports {
        http: 7878,
        https: 3000,
    };
    tokio::spawn(redirect_http_to_https(ports));

    // Create your Axum router and routes here
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));
    // Bind the server to a socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    let server = axum_server::bind_rustls(addr, tls_config);
    server.serve(app.into_make_service()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use server::utils::Ports;

    const PORTS: Ports = Ports {
        http: 7878,
        https: 3000,
    };

    #[tokio::test]
    async fn http_client_works() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let resp = client
            .get(format!("https://localhost:{}", PORTS.https))
            .send()
            .await?;
        println!("{:?}", resp);
        assert_eq!(resp.text().await?, "Hello, Axum!");
        let resp = client
            .get(format!("http://localhost:{}", PORTS.http))
            .send()
            .await?;
        println!("{:?}", resp);
        assert_eq!(resp.text().await?, "Hello, Axum!");
        Ok(())
    }
}
