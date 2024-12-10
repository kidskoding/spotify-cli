use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::model::{playlist, SimplifiedPlaylist};
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::AuthCodeSpotify;

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

pub async fn search_for_playlist(playlist_name: &str) -> Result<SimplifiedPlaylist, ()> {
    let spotify: AuthCodeSpotify = auth::spotify_from_token();
    let user = spotify.current_user()
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
            return Ok(playlist)
        }
    }
    println!("Playlist {} is not found in your library", playlist_name);
    return Err(());
}

pub async fn create_playlist(name: &str, public: bool, collaborative: bool, description: Option<&str>) {
    if collaborative == true && public == true {
        println!("Collaborative playlists must be private!");
        return;
    } 
    let spotify = auth::spotify_from_token();
    let user = spotify.current_user()
    .await
    .expect("unable to get current user!");

    let playlist = spotify.user_playlist_create(user.id, name, Some(false), Some(collaborative), description)
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
    spotify.playlist_unfollow(playlist_id).await.expect("Cannot delete playlist!");
}

pub async fn change_playlist_name(old_name: &str, new_name: &str) {
    let spotify: AuthCodeSpotify = auth::spotify_from_token();
    let playlist = search_for_playlist(old_name).await;
    if playlist.is_err() {
        return;
    }
    let playlist_id = playlist.unwrap().id;
    spotify.playlist_change_detail(playlist_id, Some(new_name), None, None, None)
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
    spotify.playlist_change_detail(playlist_id, None, None, Some(new_desc), None)
    .await
    .expect("Cannot change name of playlist!");
}
