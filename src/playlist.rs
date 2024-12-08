use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model::TrackId,
    prelude::{BaseClient, OAuthClient},
};

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
            return;
        }
    }
    println!("could not find playlist {}", target_playlist);
}

pub async fn add(target_playlist: &str, target_song: &str) {
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

    let mut playlist_to_add = None;
    for playlist in playlist_list {
        if playlist.name == target_playlist {
            playlist_to_add = Some(playlist);
            break;
        }
    }
    if playlist_to_add == None {
        println!("could not find playlist {}", target_playlist);
        return;
    }

    let _ = spotify
        .playlist_add_items(
            playlist_to_add.clone().unwrap().id,
            Some(
                TrackId::from_id(target_song)
                    .expect("invalid song id!")
                    .into(),
            ),
            Some(0),
        )
        .await
        .expect("couldn't add item to playlist!");

    println!(
        "succesfully added {} to {}",
        target_song,
        playlist_to_add.unwrap().name
    );
}
