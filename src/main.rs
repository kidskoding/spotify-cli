use clap::{CommandFactory, Parser, Subcommand};
use rspotify::model::SearchType;

mod auth;
mod follow;
mod library;
mod playlist;
mod status;
mod search;
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

    Version,

    // authenticate the user
    Auth,

    // get the current playing song
    Status,

    // follow an artist
    #[command(arg_required_else_help = true)]
    Follow {
        artist: String,
    },

    // unfollow an artist
    #[command(arg_required_else_help = true)]
    Unfollow {
        artist: String,
    },
    
    // add track to liked songs
    #[command(arg_required_else_help = true)]
    Add {
        track: String,
    },

    // remove track from liked songs
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

    // parse which command we got...
    let cli = Cli::parse();

    // match it against the list of possible commands
    match &cli.command {
        Commands::Auth => {
            auth::auth().await;
        }
        Commands::Status => {
            println!("{}", status::status().await);
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
            
            // list songs in a playlist, or all your playlists if no argument is provided
            "list" => {
                if second != "" {
                    println!("too many arguments! playlist list Option(<playlist_name>)");
                    return;
                }
                playlist::list(&first).await;
            }
            
            // add track to playlist
            "add" => {
                if first == "" || second == "" {
                    println!("not enough arguments! usage: playlist add <playlist> <track>");
                    return;
                }
                let track_id = search::search(second, SearchType::Track).await;
                playlist::add(&first, &track_id).await;
            }

            // remove track from playlist
            "remove" => {
                if first == "" || second == "" {
                    println!("not enough arguments! usage: playlist remove <playlist> <track>");
                    return;
                }
                let track_id = search::search(second, SearchType::Track).await;
                playlist::remove(&first, &track_id).await;
            }

            // create a new playlist
            "create" => {
                if first == "" {
                    println!("not enough arguments! usage: playlist create <playlist>");
                    return;
                }
                playlist::create(&first).await;
            }

            // delete (unfollow) an existing playlist
            "delete" => {
                if first == "" {
                    println!("not enough arguments! usage: playlist delete <playlist>");
                    return;
                }
                playlist::delete(&first).await;
            }

            // rename an existing playlist
            "rename" => {
                if first == "" || second == "" {
                    println!("not enough arguments! usage: playlist rename <old_name> <new_name>");
                    return;
                }
                playlist::rename(&first, &second).await;
            }

            // change the description on a playlist
            "update" => {
                if first == "" || second == "" {
                    println!("not enough arguments! usage: playlist update <playlist> <description>");
                    return;
                }
                playlist::update_description(&first, &second).await;
            }

            // no valid subcommand matched
            _ => {
                println!("invalid command! valid commands are 'list', 'add', 'remove', 'create', 'delete', 'rename', and 'update'");
            }
        },
        Commands::Add { ref track } => {
            let track_id = search::search(track, SearchType::Track).await;
            library::add(&track_id).await;
        }
        Commands::Remove { ref track } => {
            let track_id = search::search(track, SearchType::Track).await;
            library::remove(&track_id).await;
        }
    }
}
