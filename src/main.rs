use anyhow::Result;
use simple_redis::{network, Backend};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:7890";
    info!("Simple Redis Server is listening on {}", addr);
    let lister = TcpListener::bind(&addr).await?;

    let backend = Backend::new();

    loop {
        let (stream, raddr) = lister.accept().await?;
        info!("Accepted connection from {}", raddr);
        let cloned_backend = backend.clone();
        tokio::spawn(async move {
            if let Err(e) = network::steam_handler(stream, cloned_backend).await {
                warn!("Error handling for: {} while connection: {:?}", raddr, e);
            }
        });
    }
}
