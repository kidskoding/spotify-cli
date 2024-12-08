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

use rspotify::{Credentials, ClientCredsSpotify, model::UserId};
use spotify_cli::*;

// pub mod options;

// extern crate spotify;
// use spotify::Spotify;

// use options::handle_args;
// use std::env;

//#[tokio::main]
//async fn main (){
//    let creds = Credentials::from_env().unwrap();
//    let spotify = ClientCredsSpotify::new(creds);
//    spotify.request_token().await.unwrap();
//    let user_id: UserId<'_> = UserId::from_id("9saqdc9ax0rehxsiyhydswftg").unwrap();
//    let result = playlists_list(spotify.clone(), user_id).await;
//    for playlist in result.clone() {
//        println!("{}", playlist.name);
//   }
//    let __ = item_list_from_playlist(spotify.clone(), result[1].id.clone()).await;

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
