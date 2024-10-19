pub mod options;

extern crate spotify;
use spotify::Spotify;

use options::handle_args;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    handle_args(args);
}

fn welcome() {
    println!("usage: spotify [-v | --version] [-h | --help] [new]");
}

