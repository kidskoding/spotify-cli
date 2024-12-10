use rspotify::clients::OAuthClient;
use rspotify::model::ArtistId;
use rspotify::prelude::BaseClient;

use crate::auth;

pub async fn follow(id: &str) {
    let spotify = auth::spotify_from_token();

    let mut artists: Vec<ArtistId> = Vec::new();
    let artist_id = ArtistId::from_id_or_uri(id).expect("invalid artist id!");
    let artist = spotify.artist(artist_id.clone()).await.expect("error fetching artist details");
    artists.push(artist_id.clone());
    let result = spotify.user_follow_artists(artists).await;

    match result {
        Ok(_) => {
            println!("successfully followed {} with an id of {}", artist.name, artist_id);
        }
        Err(x) => {
            println!("{}", x);
        }
    }
}

pub async fn unfollow(id: &str) {
    let spotify = auth::spotify_from_token();

    let mut artists: Vec<ArtistId> = Vec::new();
    let artist_id = ArtistId::from_id_or_uri(id).expect("invalid artist id!");
    let artist = spotify.artist(artist_id.clone()).await.expect("error fetching artist details");
    artists.push(artist_id.clone());
    let result = spotify.user_unfollow_artists(artists).await;

    match result {
        Ok(_) => {
            println!("successfully unfollowed {} with an id of {}", artist.name, artist_id);
        }
        Err(x) => {
            println!("{}", x);
        }
    }
}
