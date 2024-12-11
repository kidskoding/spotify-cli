use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{ArtistId, FullArtist};

use crate::auth;

// helper function to get extra information about an artist from an id
pub async fn parse_artist_id(artist_id: &str) -> FullArtist {
    let spotify = auth::spotify_from_token();
    let result = ArtistId::from_id_or_uri(artist_id).expect("invalid artist id!");
    spotify
        .artist(result)
        .await
        .expect("error fetching artist details")
}

pub async fn follow(id: &str) {
    // user_follow_artists takes an iterator...
    // so we put the one artist we want to follow into a vec
    let mut artists: Vec<ArtistId> = Vec::new();
    let artist_id = ArtistId::from_id_or_uri(id).expect("invalid artist id!");
    artists.push(artist_id.clone());

    let spotify = auth::spotify_from_token();
    let _ = spotify
        .user_follow_artists(artists)
        .await
        .expect("couldn't follow artist!");

    let artist = parse_artist_id(id).await;
    println!(
        "successfully followed {} with an id of {}",
        artist.name, artist_id
    );
}

pub async fn unfollow(id: &str) {
    // similar logic for unfollowing artists
    let mut artists: Vec<ArtistId> = Vec::new();
    let artist_id = ArtistId::from_id_or_uri(id).expect("invalid artist id!");
    artists.push(artist_id.clone());

    let spotify = auth::spotify_from_token();
    let _ = spotify
        .user_unfollow_artists(artists)
        .await
        .expect("couldn't unfollow artist!");

    let artist = parse_artist_id(id).await;
    println!(
        "successfully unfollowed {} with an id of {}",
        artist.name, artist_id
    );
}
