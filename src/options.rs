use crate::welcome;

extern crate spotify;
use spotify::Spotify;

pub fn handle_args(args: Vec<String>) {
    // PR test #2
    if args.len() < 2 {
        println!("Usage: spotify-cli [options]");
        println!("-v, --version    Show version");
        println!("-h, --help       Show help");
        return;
    }

    match args[1].as_str() {
        "-v" | "--version" => {
            println!("spotify-cli 0.1.0");

            match Spotify::connect() {
                Ok(spotify) => match spotify.status() {
                    Ok(status) => {
                        println!("Spotify Client (Version {})", status.version());
                    }
                    Err(e) => {
                        eprintln!("Failed to get Spotify status: {:?}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to connect to Spotify: {:?}", e);
                }
            }
        }
        "-h" | "--help" => {
            welcome();
        }
        _ => {
            println!("unknown option: '{}'", args[1]);
            println!("Use -h or --help for usage information.");
        }
    }
}
