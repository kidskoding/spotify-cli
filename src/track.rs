use rspotify::{model::{PlayableId, TrackId}, prelude::OAuthClient};
use crate::{auth, song::Song, status::{get_current_song, get_next_song}};

pub async fn play_track(track: &String) {
    let spotify = auth::spotify_from_token();

    let devices = match spotify.device().await {
        Ok(devices) => devices,
        Err(_) => {
            println!("couldn't get devices!");
            return;
        }
    };

    if devices.is_empty() {
        println!("no devices found!");
        return;
    }

    let device_id = devices
        .first()
        .and_then(|d| d.id.clone());

    match spotify.start_uris_playback(
        vec![PlayableId::Track(TrackId::from_uri(track).unwrap())],
        device_id.as_deref(),
        None,
        None,
    ).await {
        Ok(_) => {
            println!("now playing: {}", Song::new(track).await.to_string());
            println!("next in queue: {}", get_next_song().await.unwrap().to_string());
        }
        Err(e) => println!("Error starting playback: {}", e),
    }
}
