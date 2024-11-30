use rspotify::{Credentials, ClientCredsSpotify, model::UserId};
use spotify_cli::*;

// pub mod options;

// extern crate spotify;
// use spotify::Spotify;

// use options::handle_args;
// use std::env;

#[tokio::main]
async fn main (){
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();
    let user_id: UserId<'_> = UserId::from_id("9saqdc9ax0rehxsiyhydswftg").unwrap();
    let result = playlists_list(spotify.clone(), user_id).await;
    for playlist in result.clone() {
        println!("{}", playlist.name);
    }
    let __ = item_list_from_playlist(spotify.clone(), result[1].id.clone()).await;
}

fn welcome() {
    println!("usage: spotify [-v | --version] [-h | --help] [new]");
}