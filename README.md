# spotify-cli

A Command Line Interface (CLI) tool for Spotify, built using Rust

## Downloading and Installing the `spotify-cli` tool

### macOS

For macOS users, the `spotify-cli` tool can be easily installed using [Homebrew](https://brew.sh/)

1. Install homebrew [here](https://brew.sh/)
2. Add the tap

   ```sh
   brew tap kidskoding/spotify-cli
   ```

3. Install the `spotify-cli` tool

   ```sh
   brew install spotify-cli
   ```

### Source

Those who wish to build the `spotify-cli` tool from source can follow the instructions below

1. Since this CLI tool was built in Rust, you will need to install Rust [here](https://www.rust-lang.org/learn/get-started)
2. Extract the tarball:

   ```sh
   wget https://github.com/kidskoding/spotify-cli/releases/download/v0.1.3-alpha/spotify-cli.tar.gz
   tar -xvzf spotify-cli.tar.gz
   ```

3. Build and run the CLI tool:

   ```sh
   cd spotify-cli-0.1.1-alpha/
   cargo install --path .
   ```

### Standalone Binary for other UNIX systems

1. Download and Extract the standalone binary:

   ```sh
   wget https://github.com/kidskoding/spotify-cli/releases/download/v0.1.3-alpha/spotify-cli.tar
   tar -xvf spotify-cli.tar
   ```

2. Move the executable to an executable binary path

   ```sh
   sudo mv ./spotify /usr/local/bin/spotify
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
