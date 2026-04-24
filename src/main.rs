use tokio::net::TcpListener;
use std::error::Error;
use std::net::SocketAddr;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Backend {
    pub address: SocketAddr,
    pub is_healthy: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let backends = Arc::new(Mutex::new(vec![
        Backend {
            address: "127.0.0.1:8081".parse().unwrap(),
            is_healthy: true,
        },
        Backend {
            address: "127.0.0.1:8082".parse().unwrap(),
            is_healthy: true,
        },
    ]));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("LBRS listening on 127.0.0.1:8080");

    loop {
        let (mut _socket, _addr) = listener.accept().await?;
        println!("Accepted connection from {}", _addr);

        let target_addr = {
            let backends_guard = backends.lock().unwrap();
            backends_guard.iter()
                .find(|b| b.is_healthy)
                .map(|b| b.address)
        };

        match target_addr {
            Some(addr) => {
                println!("Routing to {}", addr);
            }
            None => {
                eprintln!("No healthy backends available.");
            }
        }
    }
}
