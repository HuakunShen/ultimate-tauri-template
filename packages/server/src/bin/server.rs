use axum::routing::get;
use serde::Serialize;
use server::grpc::greeter::hello_world::greeter_server::GreeterServer;
use server::grpc::greeter::MyGreeter;
use std::net::SocketAddr;
use tonic::transport::Server as TonicServer;

async fn web_root() -> &'static str {
    "Hello World!"
}

#[derive(Serialize)]
struct ServerInfo {
    server_name: String,
    server_version: String,
}

async fn get_server_info() -> axum::Json<ServerInfo> {
    axum::Json(ServerInfo {
        server_name: "axum".to_string(),
        server_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeter = MyGreeter::default();
    let grpc_router = TonicServer::builder()
        .add_service(GreeterServer::new(greeter))
        .into_router()
        .route("/", get(web_root))
        .route("/info", get(get_server_info));
    let axum_addr: SocketAddr = "[::]:50051".parse()?;
    axum::Server::bind(&axum_addr)
        .serve(grpc_router.into_make_service())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use server::grpc::greeter::hello_world::greeter_client::GreeterClient;
    use server::grpc::greeter::hello_world::HelloRequest;
    use std::collections::HashMap;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GreeterClient::connect("http://[::1]:50051").await?;

        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        let response = client.say_hello(request).await?;

        println!("GRPC RESPONSE={:?}", response);
        assert_eq!(response.into_inner().message, "Hello Tonic!");

        let resp = reqwest::get("http://[::1]:50051").await?.text().await?;
        println!("{resp}");
        assert_eq!(resp, "Hello World!");
        let resp = reqwest::get("http://[::1]:50051/info")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        assert_eq!(resp["server_name"], "axum");
        println!("{resp:#?}");
        Ok(())
    }
}
