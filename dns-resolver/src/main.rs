use std::net::UdpSocket;

// hardcoded header builder
fn build_dns_header(id: u16) -> [u8; 12] {
    let mut buf = [0u8; 12];
    // bhai ye bit shift bahut confusing hai, endianness check karna
    buf[0] = (id >> 8) as u8;
    buf[1] = id as u8;
    buf[2] = 0x01; // Recursion Desired
    buf[3] = 0x00;
    buf[4] = 0x00;
    buf[5] = 0x01; // QDCOUNT
    buf
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
    let header = build_dns_header(0x1234);
    
    // send raw bits
    socket.send_to(&header, "8.8.8.8:53").expect("Failed to send");
    
    let mut resp = [0u8; 512];
    if let Ok((amt, src)) = socket.recv_from(&mut resp) {
        println!("Received {} bytes from {}", amt, src);
    }
    println!("DNS Resolution Finished.");
}
