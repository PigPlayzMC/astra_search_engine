use std::net::{
    Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener,
};

pub async fn handle_stream(listener: TcpListener) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Handler thread init complete");
    
    loop {
        let (request_stream, request_address): (std::net::TcpStream, std::net::SocketAddr) = listener.accept()?;

        println!("{:?}", request_stream);
    };
}

pub fn default_socket() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
}