use tokio::net::UdpSocket;
use std::time::Duration;
use tokio::time::timeout;
use std::sync::Arc;

// full sliding window implementation stub
#[tokio::main]
async fn main() {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());
    
    // state machine + timers
    let mut seq = 1;
    loop {
        // 200ms tak wait karo, agar ack nahi aaya to wapas bhejo yaar
        let _ = socket.send_to(&seq.to_be_bytes(), "127.0.0.1:8081").await;
        
        let mut buf = [0; 1024];
        if let Ok(Ok(_)) = timeout(Duration::from_millis(200), socket.recv_from(&mut buf)).await {
            println!("ACK for seq {}", seq);
            seq += 1;
        } else {
            println!("Retry seq {}", seq);
        }
        
        if seq > 5 { break; }
    }
}
