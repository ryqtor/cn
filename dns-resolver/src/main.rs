use std::net::UdpSocket;

fn main() {
    println!("DNS Resolver starting up...");
    let _socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
}
