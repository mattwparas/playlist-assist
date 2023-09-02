use playlist::service::{spotify_module, PlaylistCreation};
use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};

use env_logger::Builder;

use log::LevelFilter;
use steel::rvals::AsRefSteelVal;
use steel::steel_vm::engine::Engine;

fn main() {
    let mut builder = Builder::new();
    builder.filter(Some("playlist"), LevelFilter::Trace).init();

    let creds = Credentials::from_env().unwrap();

    let mut oauth = OAuth::from_env(scopes!(
        "playlist-modify-private",
        "playlist-modify-public",
        "playlist-read-private",
        "user-read-currently-playing"
    ))
    // .redirect_uri("http://localhost:8080/callback")
    // .build()
    .unwrap();

    // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
    //
    // ```
    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scopes: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```
    // let mut oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    oauth.redirect_uri = "http://localhost:8080/callback".to_string();

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    spotify.config.token_cached = true;

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).unwrap();

    let mut engine = Engine::new();

    // Load the spotify module in!
    engine.register_module(spotify_module());

    // Include the kernel to load the stuff right away
    engine.run(include_str!("../kernel/prelude.scm")).unwrap();

    let path = std::env::args().collect::<Vec<String>>()[1].clone();

    let expr = std::fs::read_to_string(&path).unwrap();

    let values = engine.run(&expr).unwrap();

    let mut _nursery = ();

    let generated_playlist = PlaylistCreation::as_ref(&values[0], &mut _nursery);

    generated_playlist
        .as_ref()
        .unwrap()
        .build(&spotify)
        .unwrap();

    println!("Done!");
}
