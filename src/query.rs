use rspotify::prelude::BaseClient;
use rspotify::{clients::OAuthClient, model::AdditionalType};

use crate::auth;

pub async fn query() {
    let spotify = auth::spotify_from_token();

    let additional_types = [AdditionalType::Track];
    let results = spotify
        .current_playing(None, Some(&additional_types))
        .await
        .expect("client error connecting to spotify");

    let playable_item = match results {
        None => {
            println!("no current playing context!");
            return;
        }
        Some(x) => x.item.unwrap(),
    };

    let playing_track = spotify
        .track(
            playable_item
                .id()
                .unwrap()
                .try_into()
                .expect("invalid track"),
            None,
        )
        .await
        .expect("error connecting to spotify");

    print!("currently playing: {}", playing_track.name + " - ");
    for i in 0..playing_track.artists.len() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", playing_track.artists[i].name);
    }
    println!();
}
