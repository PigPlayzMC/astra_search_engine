use std::{
    net::{
        IpAddr, 
        Ipv4Addr,
        TcpListener,
        AddrParseError
    },
    str::FromStr,
};

mod config;
mod error;

use config::{
    Config,
    FromFile,
};

use crate::{config::GetBindSocket, error::fatal_error};

fn main() {
    let config: Config = Config::from_file("astra.conf");

    let listener: TcpListener = match TcpListener::bind(config.get_bind_socket()) {
        Ok(t) => t,
        Err(e) => {
            fatal_error::<AddrParseError>(&("Unable to bind to socket ".to_string() + &config.get_bind_socket().to_string()), &e);

            unreachable!();
        },
    };
}
