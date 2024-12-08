use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model::{SimplifiedPlaylist, TrackId},
    prelude::{BaseClient, OAuthClient},
};

use crate::auth;

async fn get_target_playlist(target_playlist: &str) -> Option<SimplifiedPlaylist> {
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

    for playlist in playlist_list {
        if playlist.name == target_playlist {
            return Some(playlist);
        }
    }
    return None;
}

pub async fn list(target_playlist: &str) {
    let spotify = auth::spotify_from_token();
    let user = spotify
        .current_user()
        .await
        .expect("unable to get current user!");

    if target_playlist == "" {
        let playlists = spotify.user_playlists(user.id);

        pin_mut!(playlists);
        let mut playlist_list = Vec::new();
        while let Some(item) = playlists.try_next().await.unwrap() {
            playlist_list.push(item);
        }

        println!("here's all your playlists:");
        for playlist in playlist_list {
            println!("\t{}", playlist.name);
        }
        return;
    }

    let playlist_result = get_target_playlist(target_playlist).await;
    let playlist;
    match playlist_result {
        None => {
            println!("could not find playlist {}", target_playlist);
            return;
        }
        Some(x) => {
            playlist = x;
        }
    }

    let stream = spotify.playlist(playlist.id, None, None);
    pin_mut!(stream);
    let playlist = stream.await.expect("unable to fetch playlist!");

    let mut playable_items = Vec::new();
    for track in playlist.tracks.items {
        playable_items.push(track.track.unwrap());
    }

    let mut track_ids: Vec<TrackId> = Vec::new();
    for i in 0..playable_items.len() {
        let track_id = playable_items[i]
            .id()
            .expect("invalid playable id in playlist!")
            .try_into()
            .expect("invalid track in playlist!");
        track_ids.push(track_id);
    }

    let tracks = spotify
        .tracks(track_ids, None)
        .await
        .expect("couldn't get tracks from spotify!");

    println!("here's the tracks in {target_playlist}");
    for track in tracks {
        println!("\t{}", track.name);
    }
}

pub async fn add(target_playlist: &str, target_song: &str) {
    let playlist_result = get_target_playlist(target_playlist).await;
    let playlist;
    match playlist_result {
        None => {
            println!("could not find playlist {}", target_playlist);
            return;
        }
        Some(x) => {
            playlist = x;
        }
    }

    let spotify = auth::spotify_from_token();
    let _ = spotify
        .playlist_add_items(
            playlist.clone().id,
            Some(
                TrackId::from_id(target_song)
                    .expect("invalid song id!")
                    .into(),
            ),
            Some(0),
        )
        .await
        .expect("couldn't add item to playlist!");

    println!("succesfully added {} to {}", target_song, playlist.name);
}

pub async fn remove(target_playlist: &str, target_song: &str) {
    let playlist_result = get_target_playlist(target_playlist).await;
    let playlist;
    match playlist_result {
        None => {
            println!("could not find playlist {}", target_playlist);
            return;
        }
        Some(x) => {
            playlist = x;
        }
    }

    let spotify = auth::spotify_from_token();
    let _ = spotify
        .playlist_remove_all_occurrences_of_items(
            playlist.id,
            Some(
                TrackId::from_id(target_song)
                    .expect("invalid song id!")
                    .into(),
            ),
            None,
        )
        .await
        .expect("couldn't remove song from playlist!");

    println!(
        "successfully removed {} from {}",
        target_song, playlist.name
    );
}
