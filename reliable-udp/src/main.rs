use tokio::net::UdpSocket;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    
    // retry loop
    let mut retries = 0;
    loop {
        socket.send_to(b"data", "127.0.0.1:8081").await.unwrap();
        
        let mut buf = [0; 1024];
        match timeout(Duration::from_millis(200), socket.recv_from(&mut buf)).await {
            Ok(_) => {
                println!("ACK received");
                break;
            }
            Err(_) => {
                retries += 1;
                println!("Timeout, retrying... {}", retries);
                if retries > 3 { break; }
            }
        }
    }
}
