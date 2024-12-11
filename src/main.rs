use clap::{CommandFactory, Parser, Subcommand};
use rspotify::model::SearchType;

mod auth;
mod follow;
mod library;
mod playlist;
mod status;
mod volume;
mod song;
mod search;
mod helper;

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
        artist: String,
    },
    #[command(arg_required_else_help = true)]
    Unfollow {
        artist: String,
    },
    Version,
    
    // like track from id
    #[command(arg_required_else_help = true)]
    Add {
        track: String,
    },

    // unlike track from id
    #[command(arg_required_else_help = true)]
    Remove {
        track: String,
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
        Commands::Follow { artist } => {
            let artist_id = search::search(artist, SearchType::Artist).await;
            follow::follow(&artist_id).await;
        }
        Commands::Unfollow { artist } => {
            let artist_id = search::search(artist, SearchType::Artist).await;
            follow::unfollow(&artist_id).await;
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
                if second != "" {
                    println!("too many arguments! playlist list Option(<playlist_name>)");
                    return;
                }
                playlist::list(&first).await;
            }
            "add" => {
                if first == "" || second == "" {
                    println!("not enough arguments! usage: playlist add <playlist> <track>");
                    return;
                }
                let track_id = search::search(second, SearchType::Track).await;
                playlist::add(&first, &track_id).await;
            }
            "remove" => {
                if first == "" || second == "" {
                    println!("not enough arguments! usage: playlist remove <playlist> <track>");
                    return;
                }
                let track_id = search::search(second, SearchType::Track).await;
                playlist::remove(&first, &track_id).await;
            }
            "create" => {
                if first == "" {
                    println!("not enough arguments! usage: playlist create <playlist>");
                    return;
                }
                playlist::create(&first).await;
            }
            "delete" => {
                if first == "" {
                    println!("not enough arguments! usage: playlist delete <playlist>");
                    return;
                }
                playlist::delete(&first).await;
            }
            "rename" => {
                if first == "" {
                    println!("not enough arguments! usage: playlist rename <old_name> <new_name>");
                    return;
                }
                playlist::rename(&first, &second).await;
            }
            _ => {
                println!("invalid command! valid commands are 'list', 'add', 'remove', 'create', 'delete', and 'rename'");
            }
        },
        Commands::Add { ref track } => {
            let track_id = search::search(track, SearchType::Track).await;
            let track_parsed = helper::parse_track_id(&track_id).await;
            println!("here's the song we got for {}: {}", track, track_parsed.to_string());
            library::add((&track_id, track_parsed)).await;
        }
        Commands::Remove { ref track } => {
            let track_id = search::search(track, SearchType::Track).await;
            let track_parsed = helper::parse_track_id(&track_id).await;
            println!("here's the song we got for {}: {}", track, track_parsed.to_string());
            library::remove((&track_id, track_parsed)).await;
        }
    }
}
