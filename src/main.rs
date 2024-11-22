use clap::{Parser, Subcommand};

mod auth;
mod query;
mod volume;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // authenticates a user
    Auth,

    // query what track is currently playing
    Query,

    // change volume by volume_delta
    #[command(arg_required_else_help = true)]
    Volume {
        volume_delta: i8,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Auth => {
            auth::auth().await;
        }
        Commands::Query => {
            query::query().await;
        }
        Commands::Volume { volume_delta } => {
            volume::change_volume(volume_delta).await;
        }
    }
    println!("{:?}", cli);
}
