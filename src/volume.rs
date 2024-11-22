use rspotify::{clients::OAuthClient, model::AdditionalType};

use crate::auth;

pub async fn change_volume(volume_delta: i8) {
    let spotify = auth::spotify_from_token();

    let additional_types = [AdditionalType::Track];
    let results = spotify
        .current_playback(None, Some(&additional_types))
        .await
        .expect("client error connecting to spotify");

    let context = match results {
        None => {
            println!("no current playing context!");
            return;
        }
        Some(x) => x,
    };

    let cur_volume = context.device.volume_percent.unwrap_or(100);
    let mut target_volume = cur_volume as i8 + volume_delta;
    if target_volume > 100 {
        target_volume = 100;
    }
    if target_volume < 0 {
        target_volume = 0;
    }

    match spotify.volume(target_volume as u8, None).await {
        Ok(_) => {
            println!("volume is now {target_volume}");
        }
        Err(x) => {
            println!("error updating volume: {x}");
        }
    }
}
