
use std::path::{Path};
use std::process;

pub fn ensure_file(path:&str) {
    if !Path::new(path).exists() {
        eprintln!("err: file does not exist! bailing out! : '{}'", path);
        process::exit(1);
    }
}