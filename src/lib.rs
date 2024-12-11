use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model::{Country, Market, PlaylistId, PlaylistItem, SimplifiedPlaylist, UserId},
    prelude::*,
    AuthCodeSpotify,
};

pub async fn user_playlists_list(
    spotify: AuthCodeSpotify,
) -> Vec<SimplifiedPlaylist> {
    let stream = spotify. current_user_playlists();
    pin_mut!(stream);
    let mut playlist_list = Vec::new();
    while let Some(item) = stream.try_next().await.unwrap() {
        playlist_list.push(item);
    }
    return playlist_list;
}

pub async fn item_list_from_playlist(
    spotify: AuthCodeSpotify,
    playlist_id: PlaylistId<'_>,
) -> Result<Vec<PlaylistItem>, ()> {
    let market = Market::Country(Country::UnitedStates);
    let fields = "fields=items(added_by.id,track(name,href,album(name,href)))";
    let stream = spotify.playlist(playlist_id, Some(fields), Some(market));
    pin_mut!(stream);
    let playlist = stream.await;
    if playlist.is_err() {
        // What to do here?
        println!("playlist is not able to be fetched!");
        return Err(());
    }
    return Ok(playlist.unwrap().tracks.items);
}
