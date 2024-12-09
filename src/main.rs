use clap::{CommandFactory, Parser, Subcommand};

mod auth;
mod follow;
mod library;
mod playlist;
mod status;
mod volume;
mod song;

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
    Status,
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
    
    // like track from id
    #[command(arg_required_else_help = true)]
    Add {
        track_id: String,
    },

    // unlike track from id
    #[command(arg_required_else_help = true)]
    Remove {
        track_id: String,
    },

    // various commands related to controlling playlists
    Playlist {
        #[clap(index = 1)]
        command: String,

        #[clap(default_value = "", index = 2)]
        first: String,

        #[clap(default_value = "", index = 3)]
        second: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Auth => {
            auth::auth().await;
        }
        Commands::Status => {
            println!("{}", status::status().await);
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
        Commands::Playlist {
            ref command,
            ref first,
            ref second,
        } => match command.as_str() {
                "list" => {
                    if first != "" && second != "" {
                        println!("too many arguments! should only take one");
                        return;
                    }
                    playlist::list(&first).await;
                }
                "add" => {
                    if second == "" {
                        println!("not enough arguments! usage: playlist add <playlist> <track>");
                        return;
                    }
                    playlist::add(&first, &second).await;
                }
                "remove" => {
                    if second == "" {
                        println!("not enough arguments! usage: playlist remove <playlist> <track>");
                        return;
                    }
                    playlist::remove(&first, &second).await;
                }
                _ => {
                    println!("invalid command! valid commands are 'list', 'add', and 'remove'");
                }
            },
        Commands::Add { ref track_id } => {
            library::add(track_id).await;
        }
        Commands::Remove { ref track_id } => {
            library::remove(track_id).await;
        }
    }
}
