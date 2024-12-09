use rspotify::model::PlayableItem;
use rspotify::prelude::BaseClient;
use rspotify::{clients::OAuthClient, model::AdditionalType};

use crate::auth;

pub async fn status() -> String {
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

    currently_playing.push_str("\n");

    let queue = spotify
        .current_user_queue()
        .await
        .expect("error fetching your queue");

    let next_in_queue: String = if let Some(next_track) = queue.queue.get(0) {
        let next_track_name = match next_track {
            PlayableItem::Track(track) => &track.name,
            PlayableItem::Episode(episode) => &episode.name,
        };
        let mut next_in_queue = format!("next in queue: {} - ", next_track_name);
        match next_track {
            PlayableItem::Track(track) => {
                for(i, artist) in track.artists.iter().enumerate() {
                    if i > 0 {
                        next_in_queue.push_str(", ");
                    }
                    next_in_queue.push_str(&artist.name);
                }
            }
            PlayableItem::Episode(episode) => {
                // Handle episode case if needed
            }
        }
        next_in_queue
    } else {
        String::from("next in queue: none")
    };
    
    return currently_playing + &next_in_queue;
}
