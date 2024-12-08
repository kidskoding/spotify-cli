use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::model::{Country, Market, PlaylistId, PlaylistItem, SimplifiedPlaylist, UserId};
use rspotify::prelude::{BaseClient, OAuthClient};
use spotify_cli::item_list_from_playlist;

use crate::auth;

pub async fn list(target_playlist: &str) {
    let spotify = auth::spotify_from_token();
    let user = spotify
        .current_user()
        .await
        .expect("unable to get current user!");

    let playlists = spotify.user_playlists(user.id);

    pin_mut!(playlists);
    let mut playlist_list = Vec::new();
    while let Some(item) = playlists.try_next().await.unwrap() {
        playlist_list.push(item);
    }

    if target_playlist == "" {
        println!("here's all your playlists:");
        for playlist in playlist_list {
            println!("\t{}", playlist.name);
        }
        return;
    }

    for playlist in playlist_list {
        if playlist.name == target_playlist {
            let stream = spotify.playlist(playlist.id, None, None);
            pin_mut!(stream);
            let playlist = stream.await.expect("unable to fetch playlist!");

            println!("here's the tracks in {target_playlist}");
            for track in playlist.tracks.items {
                println!("\t{:?}", track.track.unwrap().id());
            }
            return;
        }
    }
    println!("could not find playlist {}", target_playlist);
}
