use clap::{CommandFactory, Parser, Subcommand};

mod auth;
mod follow;
mod playlist;
mod query;
mod volume;

#[derive(Parser, Debug)]
#[command(
    version, 
    about, 
    long_about = None,
    disable_help_flag = true,
    disable_version_flag = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Auth,
    Query,
    #[command(arg_required_else_help = true)]
    Volume {
        volume_delta: i8,
    },
    #[command(arg_required_else_help = true)]
    Follow {
        artist_id: String,
    },
    #[command(arg_required_else_help = true)]
    Unfollow {
        artist_id: String,
    },
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Auth => {
            auth::auth().await;
        }
        Commands::Query => {
            query::query().await;
        }
        Commands::Volume { volume_delta } => {
            volume::change_volume(*volume_delta).await;
        }
        Commands::Follow { artist_id } => {
            follow::follow(artist_id).await;
        }
        Commands::Unfollow { artist_id } => {
            follow::unfollow(artist_id).await;
        }
        Commands::Version => {
            print!("{}", Cli::command().render_version());
        }
    }
}
