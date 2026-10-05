use std::{
    net::{
        IpAddr, 
        Ipv4Addr,
        TcpListener,
        TcpStream,
    },
    str::FromStr,
    process::exit,
};

mod config;
mod error;

use config::{
    Config,
    FromFile,
};

use crate::config::GetBindSocket;

fn main() {
    let address: IpAddr = IpAddr::from(Ipv4Addr::from_str("127.0.0.1").expect("Failed to parse string ip address"));
    let port: u16 = 5500;

    let config: Config = Config::from_file("astra.conf");

    let listener: TcpListener = match TcpListener::bind(config.get_bind_socket()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Fatal error: Failed to bind to address {} with error {}. \nPlease check your configuration file.", config.get_bind_socket(), e);
            exit(1);
        },
    };
}
