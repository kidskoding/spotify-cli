# Installing and Running the `spotify-cli` tool

## Source

Those who wish to build the `spotify-cli` tool from source can follow the instructions below

1. Since this CLI tool was built in Rust, you will need to install Rust [here](https://rustup.rs/)
2. Clone the latest git repository:

   ```sh
   git clone https://github.com/kidskoding/spotify-cli.git
   ```

3. Build and run the CLI tool:

   Note: you may need to install `openssl-devel` on some systems in order to be able to build the application.

   ```sh
   cd spotify-cli/
   cargo install --path .
   spotify
   ```

   Alternatively, build and run without adding to path:

   ```sh
   cd spotify-cli/
   cargo run
   ```

## macOS

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

> [!NOTE]
> This CLI program only works with specific Spotify accounts. You can login
> to a testing account with the following credentials:
> `Email="samplemail490@gmail.com"`, `password="samplepass!"`

### Run the command by type `spotify` in your terminal
