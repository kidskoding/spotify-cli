use rspotify::{
    model::{SearchResult, SearchType},
    prelude::BaseClient,
};

use crate::auth;

pub async fn search(search_string: &String, search_type: SearchType) -> String {
    let spotify = auth::spotify_from_token();
    let result = spotify
        .search(search_string, search_type, None, None, None, None)
        .await
        .expect("couldn't search on spotify!");

    match result {
        SearchResult::Playlists(page) => {
            let first = page.items[0].clone();
            first.id.to_string()
        }
        SearchResult::Albums(page) => {
            let first = page.items[0].clone();
            first.id.expect("album doesn't have an id!").to_string()
        }
        SearchResult::Artists(page) => {
            let first = page.items[0].clone();
            first.id.to_string()
        }
        SearchResult::Tracks(page) => {
            let first = page.items[0].clone();
            first.id.expect("track doesn't have an id!").to_string()
        }
        SearchResult::Shows(page) => {
            let first = page.items[0].clone();
            first.id.to_string()
        }
        SearchResult::Episodes(page) => {
            let first = page.items[0].clone();
            first.id.to_string()
        }
    }
}
