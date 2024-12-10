use rspotify::clients::OAuthClient;
use rspotify::model::TrackId;

use crate::auth;
use crate::song::Song;

pub async fn add(track_pair: (&str, Song)) {
    let spotify = auth::spotify_from_token();

    let mut tracks = Vec::new();
    tracks.push(TrackId::from_id_or_uri(track_pair.0).expect("invalid track id!"));
    let _ = spotify
        .current_user_saved_tracks_add(tracks)
        .await
        .expect("error adding track to library!");

    println!("successfully added {} with an id of {} to library!", track_pair.1.to_string(), track_pair.0);
}

pub async fn remove(track_pair: (&str, Song)) {
    let spotify = auth::spotify_from_token();

    let mut tracks = Vec::new();
    tracks.push(TrackId::from_id_or_uri(track_pair.0).expect("invalid track id!"));
    let _ = spotify
        .current_user_saved_tracks_delete(tracks)
        .await
        .expect("error removing track from library!");

    println!("successfully removed {} with an id of {} from library!", track_pair.1.to_string(), track_pair.0);
}
