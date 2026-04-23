use tokio::net::TcpListener;
use std::error::Error;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Backend {
    pub address: SocketAddr,
    pub is_healthy: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("LBRS listening on 127.0.0.1:8080");

    loop {
        let (mut _socket, _addr) = listener.accept().await?;
        println!("Accepted connection from {}", _addr);
    }
}
