use std::{
    fmt, fs, io::Error, net::{
        IpAddr, 
        Ipv4Addr,
    }
};

use crate::error::fatal_error;

pub struct Config {
    address: IpAddr,
    port: u16,
}

trait FromFile {
    fn from_file(path: String) -> Config;
}

impl FromFile for Config {
    fn from_file(path: String) -> Config {
        let file: Vec<&str> = match fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => {
                fatal_error::<String, Error>("Unable to load config file", &e);
                unreachable!();
            }
        }.split("\n").collect();

        todo!()
    }
}

trait NewFile {
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

#[derive(Debug, Clone)]
struct FileExistsError;

impl fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file with the provided path already exists")
    }
}