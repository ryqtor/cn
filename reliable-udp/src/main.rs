use tokio::net::UdpSocket;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug)]
struct RPacket {
    sequence_id: u32,
    data: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").await.unwrap();
    println!("Waiting for ACKs...");
    // 200ms tak wait karo, agar ack nahi aaya to wapas bhejo yaar
}
