use std::{
    error::Error,
    process::exit
};

pub fn fatal_error<T2: Error>(text: &str, rust_error: &dyn Error) {
    if !rust_error.to_string().is_empty() {
        eprintln!("Fatal error occured: {} - {}", text, rust_error);
    } else {
        eprintln!("Fatal error occured: {}", text);
    }

    exit(1);
}