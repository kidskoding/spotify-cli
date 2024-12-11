# spotify-cli

A blazing fast CLI tool for Spotify, built using Rust

Created by Surya Bhamidi (suryab2), Anirudh Konidala (ak123), Canchen Li
(canchen4)

## Motivation

- CLI's are cool
- Spotify doesn't have an official CLI alternative

## Functionality

- list the current playing song and upcoming song in queue
- follow and unfollow artists
- query information about playlists
- create and delete playlists
- add and remove songs from playlists
- rename playlists and update playlist description

## Technical Overview

**external crates:**
- uses the `rspotify` crate to interact with the Spotify API
- uses the `clap` crate to help with parsing command line arguments

**program structure:**
- `src/main.rs` contains the main program
- other files in `src/` contains the functionality for different subcommands

## Challenges

- all the useful Spotify API endpoints (playing songs, adding to queue, etc.)
need a premium account
- our Spotify for Developers program is in Development mode, which means that
only up to 25 preselected accounts can use the application. In order to allow
everybody to use our tool, we need to apply for an Extension Request with
Spotify. As a result, you probably can't use this CLI tool :(

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details
