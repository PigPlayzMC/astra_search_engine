use std::net::{
    Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener,
};

use crate::error::{NoneError, error};

pub async fn handle_stream(order: &str, listener: TcpListener) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{}| Socket handler thread at {} initialised correctly!", order, match listener.local_addr() {
        Ok(a) => a.to_string(),
        Err(_) => {
            error::<NoneError>("Couldn't get local address for TCP Listener", &NoneError);
            "(!unknown!)".to_string()
        },
    });
    
    loop {
        let (request_stream, request_address): (std::net::TcpStream, std::net::SocketAddr) = listener.accept()?;

        println!("{:?}", request_stream);
    };
}

pub fn default_socket() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
}