use clap::{Parser, Subcommand};

mod auth;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// authenticates a user
    Auth,

    /// literally just for testing how clap works
    #[command(arg_required_else_help = true)]
    Test {
        /// value to logged to prove that the test works
        value: i32,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Auth => {
            auth::test_auth().await;
        }
        Commands::Test { value } => {
            println!("test called with value of {:?}", value)
        }
    }
    println!("{:?}", cli);
}
