use std::{
    fmt, fs, io::Error, net::{
        IpAddr, Ipv4Addr, SocketAddr,
    }
};

use crate::error::fatal_error;

pub struct Config {
    address: IpAddr,
    port: u16,
}

pub trait FromFile {
    fn from_file(path: &str) -> Config;
}

impl FromFile for Config {
    fn from_file(path: &str) -> Config {
        let file: Vec<&str> = match fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => {
                fatal_error::<Error>("Unable to load config file", &e);
                unreachable!();
            }
        }.split("\n").collect();

        todo!()
    }
}

pub trait NewFile {
    fn new_file(path: String) -> Result<(), FileExistsError>;
}

impl NewFile for Config {
    fn new_file(path: String) -> Result<(), FileExistsError> {
        let exists_result = match fs::exists(path) {
            Ok(r) => {
                match r {
                    true => return Err(FileExistsError),
                    false => return Ok(()),
                }
            }
            Err(e) => {
                eprintln!("IO Error: {}", e);
                return Err(FileExistsError)
            },
        };
    }
}

pub trait GetBindSocket {
    fn get_bind_socket(&self) -> SocketAddr;
}

impl GetBindSocket for Config {
    fn get_bind_socket(&self) -> SocketAddr {
        return (self.address, self.port).into()
    }
}

#[derive(Debug, Clone)]
pub struct FileExistsError;

impl fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file with the provided path already exists")
    }
}