use futures::channel::oneshot;
use futures::stream::TryStreamExt;
use rspotify::model::{
    idtypes::Track, AudioFeatures, FullTrack, Id, PlayableItem, PlaylistId, PlaylistItem, TrackId,
};
use rspotify::{client::Spotify, model::FullPlaylist, scopes};
use rspotify::{client::SpotifyBuilder, pagination::paginate};
use rspotify::{
    model::SimplifiedPlaylist,
    oauth2::{CredentialsBuilder, OAuthBuilder},
};

use env_logger::Builder;
use log::info;
use log::LevelFilter;

use std::time::Duration;
use tokio::time::sleep;

// use

use playlist::service::*;
use playlist::shuffle::{Recipe, SpotifyTrack};

use std::collections::HashMap;

use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterAsyncFn;
use steel::steel_vm::register_fn::RegisterFn;

use playlist::service::{RecipeWrapper, SpotifyWrapper};

#[tokio::main]
async fn main() {
    let mut builder = Builder::new();
    builder.filter(Some("playlist"), LevelFilter::Trace).init();

    let creds = CredentialsBuilder::from_env().build().unwrap();

    let oauth = OAuthBuilder::from_env()
        .scope(scopes!(
            "playlist-modify-private",
            "playlist-modify-public",
            "playlist-read-private"
        ))
        .redirect_uri("http://localhost:8080/callback")
        .build()
        .unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    // Obtaining the access token
    spotify.prompt_for_user_token().await.unwrap();

    let mut vm = Engine::new();

    // // Registering a type gives access to a predicate for the type
    vm.register_type::<RecipeWrapper>("Recipe?")
        .register_type::<SpotifyWrapper>("Spotify?");

    // // Structs in steel typically have a constructor that is the name of the struct
    vm.register_async_fn("playlist->recipe", SpotifyWrapper::create_recipe)
        .register_async_fn(
            "tracklist->playlist",
            SpotifyWrapper::create_or_update_playlist,
        );

    vm.register_fn("register-group", RecipeWrapper::add_group)
        .register_fn("shuffle", RecipeWrapper::shuffle);

    vm.register_external_value("*spotify*", SpotifyWrapper::new(spotify))
        .expect("Error registering the spotify client");

    vm.parse_and_execute_from_path("scripts/recipe.rkt")
        .expect("An error occured running the script");

    // // register_fn can be chained
    // vm.register_fn("method-by-value", ExternalStruct::method_by_value)
    //     .register_fn("set-foo", ExternalStruct::set_foo);

    // let external_struct = ExternalStruct::new(1, "foo".to_string(), 12.4);

    // // Registering an external value is fallible if the conversion fails for some reason
    // // For instance, registering an Err(T) is fallible. However, most implementation outside of manual
    // // ones should not fail
    // vm.register_external_value("external-struct", external_struct)
    //     .unwrap();

    // let playlist = find_user_playlist(&mut spotify, "DM")
    //     .await
    //     .expect("Unable to find playlist information for DM");

    // let tracks = get_playlist_tracks(&mut spotify, &playlist).await;

    // let playable_tracks = get_playable_tracks(tracks).await;

    // let object_vec: Vec<_> = playable_tracks
    //     .iter()
    //     .map(|x| SpotifyTrack::new(x.id.as_ref().unwrap(), x.name.as_str()))
    //     .collect();

    // // TODO disambiguate names when they're the same
    // let mut name_map = HashMap::new();
    // for track in &object_vec {
    //     name_map.insert(track.name.clone(), track.track_id.clone());
    // }

    // let mut recipe = Recipe::new(object_vec, name_map);

    // recipe
    //     .add_group_by_name(vec!["Blessed By A Nightmare", "Make A Sound"])
    //     .add_group_by_name(vec![
    //         "Better Not (feat. Wafia)",
    //         "Waikiki - Original Mix",
    //         "Midsummer Madness",
    //     ]);

    // let playable_tracks = recipe.shuffle();

    // let track_ids: Vec<&TrackId> = playable_tracks
    //     .iter()
    //     .map(|x| Id::from_id(&x.track_id).unwrap())
    //     .collect();

    // // let features = get_features(&mut spotify, track_ids.clone()).await;

    // // if features.len() != playable_tracks.len() {
    // //     info!("Unable to gather features for all ")
    // // } else {
    // //     info!("Successfully gathered features for all tracks");
    // // }

    // create_or_replace_contents_of_playlist(
    //     &mut spotify,
    //     "rust test playlist 2",
    //     None,
    //     track_ids.into_iter(),
    // )
    // .await
    // .expect("Couldn't create new playlist");
}
