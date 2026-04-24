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
                tokio::spawn(async move {
                    match tokio::net::TcpStream::connect(addr).await {
                        Ok(mut backend_socket) => {
                            match tokio::io::copy_bidirectional(&mut _socket, &mut backend_socket).await {
                                Ok((bytes_tx, bytes_rx)) => {
                                    println!("Proxy finished. Tx: {}, Rx: {}", bytes_tx, bytes_rx);
                                }
                                Err(e) => eprintln!("Error proxying data: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to connect to backend: {}", e),
                    }
                });
            }
            None => {
                eprintln!("No healthy backends available.");
            }
        }
    }
}
