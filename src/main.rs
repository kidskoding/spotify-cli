use clap::{Parser, Subcommand};

mod auth;
mod follow;
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

    // follow artist from id
    #[command(arg_required_else_help = true)]
    Follow {
        artist_id: Box<str>,
    },

    // unfollow artist from id
    #[command(arg_required_else_help = true)]
    Unfollow {
        artist_id: Box<str>,
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
        Commands::Follow { ref artist_id } => {
            follow::follow(artist_id).await;
        }
        Commands::Unfollow { ref artist_id } => {
            follow::unfollow(artist_id).await;
        }
    }
    println!("{:?}", cli);
}
