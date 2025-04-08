use clap::{CommandFactory, Parser, Subcommand};
use rspotify::model::SearchType;

mod auth;
mod follow;
mod library;
mod playlist;
mod status;
mod search;
mod song;
mod track;

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
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Version,

    // authenticate the user
    Auth,

    // get the current playing song
    Status,

    // play the current song
    #[command(arg_required_else_help = true)]
    Play {
        track: String,
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse which command we got...
    let cli = Cli::parse();
    let command = match &cli.command {
        Some(cmd) => cmd,
        None => return Err("no command was provided! try 'spotify help' for a list of commands".into()),
    };

    // match it against the list of possible commands
    match command {
        Command::Auth => {
            auth::auth().await;
        }
        Command::Status => {
            println!("{}", status::status().await);
        }
        Command::Play { track } => {
            let track_id = search::search(track, SearchType::Track).await;
            track::play_track(&track_id).await;
        }
        Command::Follow { artist } => {
            let artist_id = search::search(artist, SearchType::Artist).await;
            follow::follow(&artist_id).await;
        }
        Command::Unfollow { artist } => {
            let artist_id = search::search(artist, SearchType::Artist).await;
            follow::unfollow(&artist_id).await;
        }
        Command::Version => {
            print!("{}", Cli::command().render_version());
        }
        Command::Playlist {
            ref command,
            ref first,
            ref second,
        } => match command.as_str() {
                // list songs in a playlist, or all your playlists if no argument is provided
                "list" => {
                    if second != "" {
                        println!("too many arguments! playlist list Option(<playlist_name>)");
                        return Ok(());
                    }
                    playlist::list(&first).await;
                }
                
                // add track to playlist
                "add" => {
                    if first == "" || second == "" {
                        println!("not enough arguments! usage: playlist add <playlist> <track>");
                        return Ok(());
                    }
                    let track_id = search::search(second, SearchType::Track).await;
                    playlist::add(&first, &track_id).await;
                }

                // remove track from playlist
                "remove" => {
                    if first == "" || second == "" {
                        println!("not enough arguments! usage: playlist remove <playlist> <track>");
                        return Ok(());
                    }
                    let track_id = search::search(second, SearchType::Track).await;
                    playlist::remove(&first, &track_id).await;
                }

                // create a new playlist
                "create" => {
                    if first == "" {
                        println!("not enough arguments! usage: playlist create <playlist>");
                        return Ok(());
                    }
                    playlist::create(&first).await;
                }

                // delete (unfollow) an existing playlist
                "delete" => {
                    if first == "" {
                        println!("not enough arguments! usage: playlist delete <playlist>");
                        return Ok(());
                    }
                    playlist::delete(&first).await;
                }

                // rename an existing playlist
                "rename" => {
                    if first == "" || second == "" {
                        println!("not enough arguments! usage: playlist rename <old_name> <new_name>");
                        return Ok(());
                    }
                    playlist::rename(&first, &second).await;
                }

                // change the description on a playlist
                "update" => {
                    if first == "" || second == "" {
                        println!("not enough arguments! usage: playlist update <playlist> <description>");
                        return Ok(());
                    }
                    playlist::update_description(&first, &second).await;
                }

                // no valid subcommand matched
                _ => {
                    println!("invalid command! valid commands are 'list', 'add', 'remove', 'create', 'delete', 'rename', and 'update'");
                }
        },
        Command::Add { ref track } => {
            let track_id = search::search(track, SearchType::Track).await;
            library::add(&track_id).await;
        }
        Command::Remove { ref track } => {
            let track_id = search::search(track, SearchType::Track).await;
            library::remove(&track_id).await;
        }
    };

    Ok(())
}
