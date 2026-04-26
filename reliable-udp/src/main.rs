use tokio::net::UdpSocket;

#[derive(Debug)]
struct RPacket {
    sequence_id: u32,
    data: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let _socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    println!("R-UDP with sequence IDs");
}
