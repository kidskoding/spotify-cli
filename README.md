# spotify-cli

A Command Line Interface (CLI) for Spotify built using Rust, where users can manage their Spotify songs and playlists through the Command Line.

## Downloading and Installing the `spotify-cli` tool

### Mac

Mac users can use Homebrew.

1. Install Homebrew [here](https://brew.sh/)
2. Install `spotify-cli` by running:

   ```sh
   brew install spotify-cli
   ```

### Linux

Linux users can use their native package manager.

1. Debian based distributions (Debian, Ubuntu, Linux Mint, etc.) run:

   ```sh
   sudo apt update
   sudo apt install spotify-cli
   ```

2. For Red Hat based distributions (RHEL, Fedora, CentOS, etc.), run:

   ```sh
   sudo dnf install spotify-cli
   ```

3. For Arch based distribution (Arch, Manjaro) run:

   ```sh
   sudo pacman -S spotify-cli
   ```

### Source

Additionally, users can install the `spotify-cli` tool from source.

1. Install Rust [here](https://www.rust-lang.org/learn/get-started)
2. Extract the tarball:

   ```sh
   wget https://github.com/kidskoding/spotify-cli/spotify-cli.tar.gz
   
   tar -xvzf spotify-cli.tar.gz
   ```

3. Build and run the CLI tool:

   ```sh
   cd spotify-cli/
   
   cargo install --path .
   ```

## todo

### What we have worked on

- Authenticating users
- Getting current playing track
- Listing user's playlists
- Getting songs from user's playlist
- Following and unfollowing artists by ID

### What we want

- [x] Playlist management: add and remove from playlist
- [x] Follow artists by name
- [x] Add songs to library by name
- [x] Figure out a way to search by name and get an ID
- [ ] Create and delete playlists
- [x] Figure out how to put the .token in a ~/.config
- [ ] Package the CLI tool with tarball, Homebrew, apt, etc.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
