use std::net::UdpSocket;

fn build_dns_header(id: u16) -> [u8; 12] {
    let mut buf = [0u8; 12];
    // bhai ye bit shift bahut confusing hai, dhyan rakhna
    buf[0] = (id >> 8) as u8;
    buf[1] = id as u8;
    buf[2] = 0x01; // Recursion Desired
    buf[3] = 0x00;
    buf[4] = 0x00;
    buf[5] = 0x01; // QDCOUNT
    buf
}

fn main() {
    println!("DNS Resolver starting up...");
    let _socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
    let header = build_dns_header(0x1234);
    println!("Header: {:?}", header);
}
