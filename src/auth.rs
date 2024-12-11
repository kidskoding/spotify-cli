use rspotify::{
    clients::OAuthClient, scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token, TokenCallback,
};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::Arc;

// authenticate a user and write their token to ~/.config/spotify-cli/
pub async fn auth() {
    // hardcode values for spotify api...
    let creds = Credentials::new(
        "79dcd16ca7aa440dbd287cb41288888f",
        "912383c81e8e444e83b2963d48fdfb2f",
    );
    let oauth = OAuth {
        redirect_uri: "http://localhost:8000/callback".to_string(),
        scopes: scopes!(
            "user-read-playback-state",
            "user-follow-modify",
            "playlist-modify-private",
            "playlist-modify-public",
            "playlist-read-private",
            "playlist-read-collaborative",
            "user-library-modify"
        ),
        ..Default::default()
    };

    // function to write the auth token we get to ~/.config/spotify-cli/
    let write_token_to_file = |token: Token| {
        let config_path = dirs::home_dir()
            .expect("Unable to find home directory")
            .join(".config/spotify-cli");
        fs::create_dir_all(&config_path).expect("Unable to create config directory");
        let token_path = config_path.join(".token");
        let mut file = File::create(&token_path).unwrap();

        let serialized = serde_json::to_string(&token).unwrap();
        let _ = file.write_all(serialized.to_string().as_bytes());
        println!(
            ">>> Succesfully wrote token to file in {}!",
            token_path.display()
        );
        Ok(())
    };

    // tell it to call this function when we get the token
    let token_callback = TokenCallback(Box::new(write_token_to_file));

    // enabling automatic token refreshing in the config
    let config = Config {
        token_callback_fn: Arc::new(Some(token_callback)),
        ..Default::default()
    };

    // create a spotify object with our settings
    println!(">>> Fetch token with AuthCodeSpotify");
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
    let url = spotify.get_authorize_url(false).unwrap();

    // prompt the user
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");

    println!(">>> authentication completed!");
}

// return a spotify instance from a token
pub fn spotify_from_token() -> AuthCodeSpotify {
    let config_path = dirs::home_dir()
        .expect("Unable to find home directory")
        .join(".config/spotify-cli");
    let token_path = config_path.join(".token");
    let mut file = File::open(&token_path).expect(&format!(
        "couldn't find .token file in {}, maybe try running 'spotify auth' first?",
        config_path.display()
    ));

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let token = serde_json::from_str(&contents).unwrap();
    AuthCodeSpotify::from_token(token)
}
