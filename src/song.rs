use rspotify::model::{SimplifiedAlbum, SimplifiedArtist};

pub struct Song {
    name: String,
    artists: Vec<SimplifiedArtist>,
    album: SimplifiedAlbum,
}

impl Song {
    pub fn new(name: String, artists: Vec<SimplifiedArtist>, album: SimplifiedAlbum) -> Self {
        Song {
            name,
            artists,
            album,
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
