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

1. `cd ~/Downloads/spotify-cli`
2. `sudo cp target/release/spotify usr/local/bin`
