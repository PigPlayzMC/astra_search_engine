use std::{
    net::{
        AddrParseError,
        TcpListener,
    },
};

mod config;
mod error;

use config::{
    Config,
    FromFile,
};

use crate::{config::{FileExistsError, FileNotFoundError, GetBindSocket, NewFile}, error::{error, fatal_error}};

fn main() {
    let config_path = "astra.conf";

    let config: Config;
    match Config::from_file(config_path) {
        Ok(c) => {
            config = c;
        },
        Err(e) => { // Create a new config file, then load from that
            error::<FileNotFoundError>("Unable to open config file, attempting to create new file", &e);

            match Config::new_file(config_path) {
                Ok(_) => (),
                Err(e) => {
                    fatal_error::<FileExistsError>("Unable to create default config file", &e);

                    unreachable!();
                }
            };

            config = Config::from_file(config_path).expect("Config file should be avaliable now, as it has been created.");
        }
    };

    let listener: TcpListener = match TcpListener::bind(config.get_bind_socket()) {
        Ok(t) => t,
        Err(e) => {
            fatal_error::<AddrParseError>(&("Unable to bind to socket ".to_string() + &config.get_bind_socket().to_string()), &e);

            unreachable!();
        },
    };
}
