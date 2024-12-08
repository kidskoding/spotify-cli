# spotify-cli

A Command Line Interface (CLI) for Spotify built using Rust, where users can manage their Spotify songs and playlists through the Command Line

## Enable command globally

### Windows

1. Open Command Prompt as an Administrator
2. `cd ~/Downloads/spotify-cli`
3. `mkdir "C:\Program Files\spotify"`
4. `copy target\release\spotify.exe "C:\Program Files\spotify\"`
5. Add to System Environment PATH Variables
   - Go to system variables and edit PATH and add the path `C:\Program Files\spotify\`

### UNIX Systems (Mac and Linux)

1. Open up any command line
2. `cd ~/Downloads/spotify-cli`
3. `sudo cp target/release/spotify usr/local/bin` -> administrator is required

## todo:

what we have worked on:
- authenticating users
- getting current playing track
- list user's playlists
- get songs from user's playlist
- follow and unfollow artists by id

what we want:
- [x] playlist management: add and remove from playlist
- [ ] create and delete playlists
- [ ] follow artists by name
- [ ] add songs to library by name
- [ ] figure out a way to search by name and get an id
