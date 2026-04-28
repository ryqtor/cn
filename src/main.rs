use tokio::net::TcpListener;
use std::error::Error;
use std::net::SocketAddr;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    port: String,

    #[arg(short, long, required = true, num_args = 1..)]
    backends: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Backend {
    pub address: SocketAddr,
    pub is_healthy: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();

    let parsed_backends: Vec<Backend> = args.backends.into_iter()
        .map(|b| Backend {
            address: b.parse().expect("Invalid backend address"),
            is_healthy: true,
        })
        .collect();

    let backends = Arc::new(Mutex::new(parsed_backends));

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

    let listener = TcpListener::bind(&args.port).await?;
    tracing::info!("LBRS listening on {}", args.port);

    loop {
        let (mut _socket, _addr) = listener.accept().await?;
        tracing::info!("Accepted connection from {}", _addr);

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
                tracing::info!("Routing to {}", addr);
                tokio::spawn(async move {
                    match tokio::net::TcpStream::connect(addr).await {
                        Ok(mut backend_socket) => {
                            match tokio::io::copy_bidirectional(&mut _socket, &mut backend_socket).await {
                                Ok((bytes_tx, bytes_rx)) => {
                                    tracing::info!("Proxy finished. Tx: {}, Rx: {}", bytes_tx, bytes_rx);
                                }
                                Err(e) => tracing::error!("Error proxying data: {}", e),
                            }
                        }
                        Err(e) => tracing::error!("Failed to connect to backend: {}", e),
                    }
                });
            }
            None => {
                tracing::error!("No healthy backends available.");
            }
        }
    }
}
