use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model::{SimplifiedPlaylist, TrackId},
    prelude::{BaseClient, OAuthClient},
};

use crate::{auth, song::Song};

// util function to get a specific playlist from the current user by name
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

    // if no argument was passed...
    // print all the playlists of the current user
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

    // else search for the playlist in the argument
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

    let playlist = spotify
        .playlist(playlist.id, None, None)
        .await
        .expect("unable to fetch playlist!");

    // put the items in the playlist into a vec...
    let mut playable_items = Vec::new();
    for track in playlist.tracks.items {
        playable_items.push(track.track.unwrap());
    }

    // get their track ids...
    let mut track_ids: Vec<TrackId> = Vec::new();
    for i in 0..playable_items.len() {
        let track_id = playable_items[i]
            .id()
            .expect("invalid playable id in playlist!")
            .try_into()
            .expect("invalid track in playlist!");
        track_ids.push(track_id);
    }

    // query spotify for more info about these tracks...
    let tracks = spotify
        .tracks(track_ids, None)
        .await
        .expect("couldn't get tracks from spotify!");

    // print their names
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
            playlist.id,
            Some(
                TrackId::from_id_or_uri(target_song)
                    .expect("invalid song id!")
                    .into(),
            ),
            Some(0),
        )
        .await
        .expect("couldn't add item to playlist!");

    println!(
        "succesfully added {} to {}",
        Song::new(target_song).await.to_string(),
        playlist.name
    );
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
        "helpersuccessfully removed {} from {}",
        Song::new(target_song).await.to_string(),
        playlist.name
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

pub async fn rename(old_name: &str, new_name: &str) {
    let playlist_result = get_target_playlist(old_name).await;
    let playlist;
    match playlist_result {
        None => {
            println!("could not find playlist {}", old_name);
            return;
        }
        Some(x) => {
            playlist = x;
        }
    }

    let spotify = auth::spotify_from_token();
    spotify
        .playlist_change_detail(playlist.id, Some(new_name), None, None, None)
        .await
        .expect("Cannot change name of playlist!");

    println!(
        "succesfully renamed playlist from {} to {}",
        playlist.name, new_name
    );
    println!("this might take a while for your changes to be reflected");
}

pub async fn update_description(playlist_name: &str, desc: &str) {
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
    spotify
        .playlist_change_detail(playlist.id, None, None, Some(desc), None)
        .await
        .expect("could not update playlist description!");

    println!(
        "succesfully updated playlist {}'s description to {}",
        playlist.name, desc
    );
    println!("this might take a while for your changes to be reflected");
}
