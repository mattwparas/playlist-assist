use futures::channel::oneshot;
use futures::stream::TryStreamExt;
use rspotify::{
    client::ClientResult,
    model::{
        idtypes::Track, AudioFeatures, FullTrack, Id, PlayableItem, PlaylistId, PlaylistItem,
        PrivateUser, TrackId,
    },
};
use rspotify::{client::Spotify, model::FullPlaylist, scopes};
use rspotify::{client::SpotifyBuilder, pagination::paginate};
use rspotify::{
    model::SimplifiedPlaylist,
    oauth2::{CredentialsBuilder, OAuthBuilder},
};

use env_logger::Builder;
use log::LevelFilter;
use log::{debug, info};

use std::{thread::current, time::Duration};
use tokio::time::sleep;

use std::cell::RefCell;
use std::rc::Rc;

use steel_derive::Steel;

use crate::shuffle::{Recipe, SpotifyTrack};

use std::collections::HashMap;

#[derive(Steel, Debug, Clone)]
pub struct SpotifyWrapper {
    client: Rc<RefCell<Spotify>>,
}

impl SpotifyWrapper {
    pub fn new(client: Spotify) -> Self {
        SpotifyWrapper {
            client: Rc::new(RefCell::new(client)),
        }
    }

    pub async fn create_recipe(self, playlist_name: String) -> RecipeWrapper {
        let mut client = self.client.borrow_mut();
        let recipe = build_recipe_from_playlist(&mut client, playlist_name.as_str()).await;
        RecipeWrapper::new(recipe)
    }

    pub async fn create_or_update_playlist(
        self,
        playlist_name: String,
        track_list: Vec<SpotifyTrack>,
    ) {
        let track_ids: Vec<&TrackId> = track_list
            .iter()
            .map(|x| Id::from_id(&x.track_id).unwrap())
            .collect();

        let mut client = self.client.borrow_mut();

        create_or_replace_contents_of_playlist(
            &mut client,
            playlist_name.as_str(),
            None,
            track_ids.into_iter(),
        )
        .await
        .expect("Couldn't create new playlist");
    }
}

#[derive(Debug, Clone, Steel)]
pub struct RecipeWrapper {
    recipe: Rc<RefCell<Recipe>>,
}

impl RecipeWrapper {
    fn new(recipe: Recipe) -> Self {
        RecipeWrapper {
            recipe: Rc::new(RefCell::new(recipe)),
        }
    }

    pub fn add_group(self, group: Vec<String>) {
        self.recipe
            .borrow_mut()
            .add_group_by_name(group.iter().map(|x| x.as_str()).collect());
    }

    pub fn shuffle(self) -> Vec<SpotifyTrack> {
        self.recipe.borrow().shuffle()
    }
}

pub async fn build_recipe_from_playlist(spotify: &mut Spotify, playlist_name: &str) -> Recipe {
    let playlist = find_user_playlist(spotify, playlist_name)
        .await
        .expect(format!("Unable to find playlist information for {}", playlist_name).as_str());

    let tracks = get_playlist_tracks(spotify, &playlist).await;

    let playable_tracks = get_playable_tracks(tracks).await;

    let object_vec: Vec<_> = playable_tracks
        .iter()
        .map(|x| SpotifyTrack::new(x.id.as_ref().unwrap(), x.name.as_str()))
        .collect();

    // TODO disambiguate names when they're the same
    let mut name_map = HashMap::new();
    for track in &object_vec {
        name_map.insert(track.name.clone(), track.track_id.clone());
    }

    let recipe = Recipe::new(object_vec, name_map);

    recipe
}

pub async fn get_playable_tracks(tracks: Vec<PlaylistItem>) -> Vec<FullTrack> {
    let mut playable_tracks = Vec::new();

    for item in tracks {
        if let Some(PlayableItem::Track(t)) = item.track {
            // println!("* {}", t.name);
            playable_tracks.push(t);
        }
    }

    playable_tracks
}

pub async fn get_features(spotify: &mut Spotify, track_ids: Vec<&Id<Track>>) -> Vec<AudioFeatures> {
    // batch the feature gathering
    let window: usize = 50;
    let playlist_length = track_ids.len();
    let requests = playlist_length / window;

    info!(
        "Playlist length: {}, request count: {}",
        playlist_length, requests
    );

    let mut all_features: Vec<AudioFeatures> = Vec::new();

    let mut ranges = Vec::new();

    for i in 0..requests {
        ranges.push(i * 50..(i + 1) * 50);
    }

    if window * requests < playlist_length {
        ranges.push(requests * 50..playlist_length);
    }

    for r in ranges {
        info!("Gathering data from slice: {:?}", r);
        let track_slice = track_ids[r].to_vec();
        sleep(Duration::from_millis(100)).await;
        let features = spotify.tracks_features(track_slice).await.unwrap();
        if let Some(mut features) = features {
            all_features.append(&mut features);
        }
    }

    all_features
}

pub async fn find_user_playlist(
    spotify: &mut Spotify,
    playlist_name: &str,
) -> Option<SimplifiedPlaylist> {
    let limit = 50;
    let mut offset = 0;
    info!("Gathering playlists");
    loop {
        let page = spotify
            .current_user_playlists_manual(Some(limit), Some(offset))
            .await
            .unwrap();
        for item in page.items {
            debug!("* {}", item.name);

            if item.name == playlist_name {
                info!("Found playlist: {} with id: {}", playlist_name, item.id);
                return Some(item);
            }
        }

        sleep(Duration::from_millis(25)).await;

        // The iteration ends when the `next` field is `None`. Otherwise, the
        // Spotify API will keep returning empty lists from then on.
        if page.next.is_none() {
            break;
        }

        offset += limit;
    }

    info!("Failed to find playlist: {}", playlist_name);

    None
}

pub async fn get_playlist_tracks(
    spotify: &mut Spotify,
    playlist: &SimplifiedPlaylist,
) -> Vec<PlaylistItem> {
    let limit = 50;
    let mut offset = 0;
    let mut output = Vec::new();
    let id: &PlaylistId = Id::from_id(&playlist.id).unwrap();
    info!("Gathering track info from playlist: {}", playlist.name);
    loop {
        let mut page = spotify
            .playlist_tracks_manual(id, None, None, Some(limit), Some(offset))
            .await
            .unwrap();

        sleep(Duration::from_millis(50)).await;

        output.append(&mut page.items);

        // The iteration ends when the `next` field is `None`. Otherwise, the
        // Spotify API will keep returning empty lists from then on.
        if page.next.is_none() {
            break;
        }

        offset += limit;
    }

    info!("Gathered tracks for playlist: {}", playlist.name);

    output
}

pub async fn create_empty_playlist(
    spotify: &mut Spotify,
    name: &str,
) -> ClientResult<FullPlaylist> {
    let current_user: PrivateUser = spotify.current_user().await.unwrap();

    info!(
        "Creating empty playlist for user: {:?} with id: {}",
        current_user.display_name, current_user.id
    );

    let user_id = Id::from_id(&current_user.id).unwrap();

    spotify
        .user_playlist_create(
            user_id,
            name,
            Some(false),
            Some(false),
            Some("a test playlist"),
        )
        .await
}

pub async fn replace_contents_of_playlist(
    spotify: &mut Spotify,
    playlist_id: &PlaylistId,
    track_ids: impl Iterator<Item = &TrackId>,
) -> ClientResult<()> {
    info!("Replacing contents of playlist");
    spotify
        .playlist_replace_tracks(playlist_id, track_ids)
        .await
}

pub async fn create_or_replace_contents_of_playlist(
    spotify: &mut Spotify,
    name: &str,
    playlist_id: Option<&PlaylistId>,
    track_ids: impl Iterator<Item = &TrackId>,
) -> ClientResult<()> {
    if let Some(playlist_id) = playlist_id {
        return replace_contents_of_playlist(spotify, playlist_id, track_ids).await;
    } else {
        let playlist = find_user_playlist(spotify, name).await;
        sleep(Duration::from_millis(100)).await;

        match playlist {
            Some(p) => {
                let playlist_id = Id::from_id(&p.id).unwrap();
                return replace_contents_of_playlist(spotify, playlist_id, track_ids).await;
            }
            None => {
                let created_playlist = create_empty_playlist(spotify, name)
                    .await
                    .expect("Failed to create playlist");

                sleep(Duration::from_millis(100)).await;

                let playlist_id = Id::from_id(created_playlist.id.as_str()).unwrap();
                return replace_contents_of_playlist(spotify, playlist_id, track_ids).await;
            }
        };
    }
}
