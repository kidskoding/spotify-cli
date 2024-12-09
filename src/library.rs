use rspotify::clients::OAuthClient;
use rspotify::model::TrackId;

use crate::auth;

pub async fn add(track_id: &str) {
    let spotify = auth::spotify_from_token();

    let mut tracks = Vec::new();
    tracks.push(TrackId::from_id_or_uri(track_id).expect("invalid track id!"));
    let _ = spotify
        .current_user_saved_tracks_add(tracks)
        .await
        .expect("error adding track to library!");

    println!("successfully added {track_id} to library!");
}

pub async fn remove(track_id: &str) {
    let spotify = auth::spotify_from_token();

    let mut tracks = Vec::new();
    tracks.push(TrackId::from_id_or_uri(track_id).expect("invalid track id!"));
    let _ = spotify
        .current_user_saved_tracks_delete(tracks)
        .await
        .expect("error removing track from library!");

    println!("successfully removed {track_id} to library!");
}
