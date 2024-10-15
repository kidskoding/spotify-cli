use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if(args[0] == "spotify") {
        if(args.len() == 1) {
            welcome();
        } else {
            match args[1].as_str() {
                "-v" | "--version" => {
                    println!("spotify 0.1.0");
                },
                "-h" | "--help" => {
                    welcome();
                },
                _ => {
                    println!("unknown option: '{}'", args[1]);
                }
            }
        }
    }
}

fn welcome() {
    println!("usage: spotify [-v | --version] [-h | --help]");
}