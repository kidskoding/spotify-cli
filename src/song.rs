use crate::auth;

use rspotify::clients::BaseClient;
use rspotify::model::{SimplifiedAlbum, SimplifiedArtist, TrackId};

pub struct Song {
    name: String,
    artists: Vec<SimplifiedArtist>,
    album: SimplifiedAlbum,
}

impl Song {
    pub async fn new(track_id: &str) -> Self {
        let spotify = auth::spotify_from_token();
        let track = spotify
            .track(
                TrackId::from_id_or_uri(track_id).expect("invalid track id!"),
                None,
            )
            .await
            .expect("error fetching track!");
        Song {
            name: track.name,
            artists: track.artists,
            album: track.album,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::new();

        let name = &self.name;
        let artists = &self.artists;
        let album = &self.album;

        result.push_str(&(name.clone() + " - "));

        for i in 0..artists.len() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&artists[i].name);
        }

        result.push_str(&(" on ".to_string() + &album.name));

        result
    }
}
