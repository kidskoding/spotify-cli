use rspotify::prelude::BaseClient;
use rspotify::{clients::OAuthClient, model::AdditionalType, AuthCodeSpotify};

use std::fs::File;
use std::io::Read;

pub async fn query() {
    let mut file = File::open(".token").expect("couldn't find .token file, maybe try auth first?");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let token = serde_json::from_str(&contents).unwrap();
    let spotify = AuthCodeSpotify::from_token(token);

    // Running the requests
    let additional_types = [AdditionalType::Track];
    let results = spotify
        .current_playing(None, Some(&additional_types))
        .await
        .expect("client error connecting to spotify");

    let playing_item = match results {
        None => {
            println!("no current playing context!");
            return;
        }
        Some(x) => x.item.unwrap(),
    };

    let playing_track = spotify
        .track(
            playing_item
                .id()
                .unwrap()
                .try_into()
                .expect("invalid track"),
            None,
        )
        .await
        .expect("error connecting to spotify");

    println!("currently playing: {}", playing_track.name)
}
