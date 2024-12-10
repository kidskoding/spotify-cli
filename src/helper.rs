use rspotify::model::{ArtistId, FullArtist, TrackId};
use rspotify::prelude::BaseClient;

use crate::auth;
use crate::song::Song;

pub async fn parse_track_id(track_id: &str) -> Song {
    let spotify = auth::spotify_from_token();

    let track = spotify
        .track(TrackId::from_id_or_uri(track_id).expect("invalid track id!"), None)
        .await
        .expect("error fetching track!");

    Song::new(track.name, track.artists, track.album)
}

pub async fn parse_artist_id(artist_id: &str) -> FullArtist {
    let spotify = auth::spotify_from_token();

    let result = ArtistId::from_id_or_uri(artist_id).expect("invalid artist id!");
    
    spotify.artist(result).await.expect("error fetching artist details")
}
