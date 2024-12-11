use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model::{SimplifiedPlaylist, TrackId},
    prelude::{BaseClient, OAuthClient},
    AuthCodeSpotify,
};

use crate::auth;

async fn get_target_playlist(target_playlist: &str) -> Option<SimplifiedPlaylist> {
    let spotify = auth::spotify_from_token();
    let playlists = spotify.current_user_playlists();

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
                TrackId::from_id_or_uri(target_song)
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
                TrackId::from_id_or_uri(target_song)
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

pub async fn create(playlist_name: &str) {
    let spotify = auth::spotify_from_token();
    let user = spotify
        .current_user()
        .await
        .expect("unable to get current user!");

    let _ = spotify
        .user_playlist_create(user.id, playlist_name, Some(true), None, None)
        .await
        .expect("could not create playlist!");

    println!("successfully created playlist {}", playlist_name);
}

pub async fn delete(playlist_name: &str) {
    let playlist_result = get_target_playlist(playlist_name).await;
    let playlist;
    match playlist_result {
        None => {
            println!("could not find playlist {}", playlist_name);
            return;
        }
        Some(x) => {
            playlist = x;
        }
    }

    let spotify = auth::spotify_from_token();
    let _ = spotify
        .playlist_unfollow(playlist.id)
        .await
        .expect("couldn't delete playlist!");

    println!("successfully deleted playlist {}", playlist_name);
}

pub async fn search_for_playlist(playlist_name: &str) -> Result<SimplifiedPlaylist, ()> {
    let spotify: AuthCodeSpotify = auth::spotify_from_token();
    let user = spotify
        .current_user()
        .await
        .expect("unable to get current user!");
    println!("{}", user.id);

    let playlists = spotify.user_playlists(user.id);
    pin_mut!(playlists);
    let mut playlist_list = Vec::new();
    while let Some(item) = playlists.try_next().await.unwrap() {
        playlist_list.push(item);
    }

    for playlist in playlist_list {
        if playlist.name == playlist_name {
            return Ok(playlist);
        }
    }
    println!("Playlist {} is not found in your library", playlist_name);
    return Err(());
}

pub async fn create_playlist(
    name: &str,
    public: bool,
    collaborative: bool,
    description: Option<&str>,
) {
    if collaborative == true && public == true {
        println!("Collaborative playlists must be private!");
        return;
    }
    let spotify = auth::spotify_from_token();
    let user = spotify
        .current_user()
        .await
        .expect("unable to get current user!");

    let playlist = spotify
        .user_playlist_create(user.id, name, Some(false), Some(collaborative), description)
        .await
        .expect("Unable to create playlist!");
}

pub async fn delete_playlist(name: &str) {
    let spotify: AuthCodeSpotify = auth::spotify_from_token();
    let playlist = search_for_playlist(name).await;
    if playlist.is_err() {
        println!("Cannot delete playlist!");
        return;
    }
    let playlist_id = playlist.unwrap().id;
    spotify
        .playlist_unfollow(playlist_id)
        .await
        .expect("Cannot delete playlist!");
}

pub async fn change_playlist_name(old_name: &str, new_name: &str) {
    let spotify: AuthCodeSpotify = auth::spotify_from_token();
    let playlist = search_for_playlist(old_name).await;
    if playlist.is_err() {
        return;
    }
    let playlist_id = playlist.unwrap().id;
    spotify
        .playlist_change_detail(playlist_id, Some(new_name), None, None, None)
        .await
        .expect("Cannot change name of playlist!");
}

pub async fn change_playlist_description(old_desc: &str, new_desc: &str) {
    let spotify: AuthCodeSpotify = auth::spotify_from_token();
    let playlist = search_for_playlist(old_desc).await;
    if playlist.is_err() {
        return;
    }
    let playlist_id = playlist.unwrap().id;
    spotify
        .playlist_change_detail(playlist_id, None, None, Some(new_desc), None)
        .await
        .expect("Cannot change name of playlist!");
}
