use tokio::net::TcpListener;
use std::error::Error;
use std::net::SocketAddr;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

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

    let rr_counter = Arc::new(AtomicUsize::new(0));

    let backends_health = backends.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            
            let addresses = {
                let guard = backends_health.lock().unwrap();
                guard.iter().map(|b| b.address).collect::<Vec<_>>()
            };

            for addr in addresses {
                let is_healthy = match tokio::time::timeout(
                    std::time::Duration::from_secs(2),
                    tokio::net::TcpStream::connect(addr)
                ).await {
                    Ok(Ok(_)) => true,
                    _ => false,
                };
                
                let mut guard = backends_health.lock().unwrap();
                if let Some(backend) = guard.iter_mut().find(|b| b.address == addr) {
                    backend.is_healthy = is_healthy;
                }
            }
        }
    });

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("LBRS listening on 127.0.0.1:8080");

    loop {
        let (mut _socket, _addr) = listener.accept().await?;
        println!("Accepted connection from {}", _addr);

        let target_addr = {
            let backends_guard = backends.lock().unwrap();
            let healthy_backends: Vec<_> = backends_guard.iter().filter(|b| b.is_healthy).collect();
            if healthy_backends.is_empty() {
                None
            } else {
                let idx = rr_counter.fetch_add(1, Ordering::SeqCst);
                Some(healthy_backends[idx % healthy_backends.len()].address)
            }
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
