use rspotify::prelude::BaseClient;
use rspotify::{clients::OAuthClient, model::AdditionalType};

use crate::auth;

pub async fn query() -> String {
    let spotify = auth::spotify_from_token();

    let additional_types = [AdditionalType::Track];
    let results = spotify
        .current_playing(None, Some(&additional_types))
        .await
        .expect("client error connecting to spotify");

    let playable_item = match results {
        None => {
            return String::from("no current playing context!");
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

    let mut currently_playing: String = format!("currently playing: {}", playing_track.name + " - ");
    for i in 0..playing_track.artists.len() {
        if i > 0 {
            currently_playing.push_str(", ");
        }
        currently_playing.push_str(&playing_track.artists[i].name);
    }
    
    currently_playing
}
