use std::{
    net::{
        IpAddr, 
        Ipv4Addr,
        TcpListener,
        TcpStream,
    },
    str::FromStr,
};

mod config;
mod error;

use config::Config;

fn main() {
    let address: IpAddr = IpAddr::from(Ipv4Addr::from_str("127.0.0.1").expect("Failed to parse string ip address"));
    let port: u16 = 5500;

    let listener: TcpListener = match TcpListener::bind((config.address, config.control_port)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Fatal error: Failed to bind to address {} with error {}. \nPlease check your configuration file.", config.address, e);
            exit(1);
        },
    };
}
