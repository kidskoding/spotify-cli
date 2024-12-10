use rspotify::model::PlayableItem;
use rspotify::prelude::BaseClient;
use rspotify::{clients::OAuthClient, model::AdditionalType};

use crate::auth;
use crate::song::Song;

pub async fn status() -> String {
    let currently_playing = match get_current_song().await {
        Ok(song) => song,
        Err(err) => return err,
    };
    let next_in_queue = match get_next_song().await {
        Ok(song) => song,
        Err(err) => return err,
    };

    let status = String::from("currently playing: ")
        + &currently_playing.to_string()
        + "\n"
        + "next in queue: "
        + &next_in_queue.to_string();

    status
}

async fn get_current_song() -> Result<Song, String> {
    let spotify = auth::spotify_from_token();

    let additional_types = [AdditionalType::Track];
    let results = spotify
        .current_playing(None, Some(&additional_types))
        .await
        .expect("client error connecting to spotify");

    let playable_item = match results {
        None => {
            return Err(String::from("no current playing context!"));
        }
        Some(x) => x.item.unwrap(),
    };

    let current_song = spotify
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

    let name = current_song.name;
    let artists = current_song.artists;
    let album = current_song.album;

    let song = Song::new(name, artists, album);

    Ok(song)
}

async fn get_next_song() -> Result<Song, String> {
    let spotify = auth::spotify_from_token();

    let queue = spotify
        .current_user_queue()
        .await
        .expect("error fetching your queue");

    let next_track = match queue.queue.get(0) {
        Some(track) => track,
        None => return Err(String::from("next in queue: none")),
    };

    let next_song = match next_track {
        PlayableItem::Track(track) => {
            let track = spotify
                .track(
                    track.id.clone().unwrap().try_into().expect("invalid track"),
                    None,
                )
                .await
                .expect("error connecting to spotify");

            let name = track.name;
            let artists = track.artists;
            let album = track.album;

            Song::new(name, artists, album)
        }
        PlayableItem::Episode(_) => {
            return Err(String::from("next in queue: none"));
        }
    };

    Ok(next_song)
}
