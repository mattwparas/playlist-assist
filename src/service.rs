use std::collections::HashSet;
use std::io::Cursor;

use rand::seq::SliceRandom;
use rspotify::model::{AlbumId, FullPlaylist};
use rspotify::model::{
    ArtistId, Country, FullAlbum, Market, Page, SearchResult, SearchType, SimplifiedAlbum,
    SimplifiedPlaylist, SimplifiedTrack,
};
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::AuthCodeSpotify;
use rspotify::{
    model::{FullTrack, Id, PlayableItem, PlaylistId, PlaylistItem, PrivateUser, TrackId},
    ClientResult,
};
// use rspotify::{der};

use log::{debug, info};
// use tokio::pin;

// use tokio::time::sleep;

use rand::thread_rng;
use steel::rvals::{Custom, SteelString};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

// use skim::prelude::*;

// fn fuzzy_find() {
//     let options = SkimOptionsBuilder::default()
//         .height(Some("50%"))
//         .multi(true)
//         .build()
//         .unwrap();

//     let input = "aaaaa\nbbbb\nccc".to_string();

//     // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
//     // `SkimItem` was implemented for `AsRef<str>` by default
//     let item_reader = SkimItemReader::default();
//     let items = item_reader.of_bufread(Cursor::new(input));

//     // `run_with` would read and show items from the stream
//     let selected_items = Skim::run_with(&options, Some(items))
//         .map(|out| out.selected_items)
//         .unwrap_or_else(|| Vec::new());

//     for item in selected_items.iter() {
//         print!("{}{}", item.output(), "\n");
//     }
// }

fn throttle() {
    std::thread::sleep(std::time::Duration::from_millis(100));

    // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

macro_rules! wait {
    () => {
        throttle();
    };
}

pub fn spotify_module() -> BuiltInModule {
    let mut module = BuiltInModule::new("steel/spotify");

    module
        .register_fn("track-search-criteria", TrackSearchCriteria::new)
        .register_fn("album-search-criteria", AlbumSearchCriteria::raw)
        // TODO: This is going to clone everything, but for now we can live with it
        .register_fn("playlist-creation", PlaylistCreation::new);

    module
}

struct SpotifyClient {
    spotify: AuthCodeSpotify,
}

impl Custom for SpotifyClient {}

// use steel_derive::Steel;

// use crate::shuffle::{Recipe, SpotifyTrack};

// use std::collections::HashMap;

// #[derive(Debug, Clone)]
// pub struct SpotifyWrapper {
//     client: Rc<RefCell<Spotify>>,
// }

// impl SpotifyWrapper {
//     pub fn new(client: Spotify) -> Self {
//         SpotifyWrapper {
//             client: Rc::new(RefCell::new(client)),
//         }
//     }

//     pub async fn create_recipe(self, playlist_name: String) -> RecipeWrapper {
//         let mut client = self.client.borrow_mut();
//         let recipe = build_recipe_from_playlist(&mut client, playlist_name.as_str()).await;
//         RecipeWrapper::new(recipe)
//     }

//     pub async fn create_or_update_playlist(
//         self,
//         playlist_name: String,
//         track_list: Vec<SpotifyTrack>,
//     ) {
//         let track_ids: Vec<&TrackId> = track_list
//             .iter()
//             .map(|x| Id::from_id(&x.track_id).unwrap())
//             .collect();

//         let mut client = self.client.borrow_mut();

//         create_or_replace_contents_of_playlist(
//             &mut client,
//             playlist_name.as_str(),
//             None,
//             track_ids.into_iter(),
//         )
//         .await
//         .expect("Couldn't create new playlist");
//     }
// }

// #[derive(Debug, Clone)]
// pub struct RecipeWrapper {
//     recipe: Rc<RefCell<Recipe>>,
// }

// impl RecipeWrapper {
//     fn new(recipe: Recipe) -> Self {
//         RecipeWrapper {
//             recipe: Rc::new(RefCell::new(recipe)),
//         }
//     }

//     pub fn add_group(self, group: Vec<String>) {
//         self.recipe
//             .borrow_mut()
//             .add_group_by_name(group.iter().map(|x| x.as_str()).collect());
//     }

//     pub fn shuffle(self) -> Vec<SpotifyTrack> {
//         self.recipe.borrow().shuffle()
//     }
// }

// pub async fn build_recipe_from_playlist(spotify: &mut Spotify, playlist_name: &str) -> Recipe {
//     let playlist = find_user_playlist(spotify, playlist_name)
//         .await
//         .expect(format!("Unable to find playlist information for {}", playlist_name).as_str());

//     let tracks = get_playlist_tracks(spotify, &playlist).await;

//     let playable_tracks = get_playable_tracks(tracks).await;

//     let object_vec: Vec<_> = playable_tracks
//         .iter()
//         .map(|x| SpotifyTrack::new(x.id.as_ref().unwrap(), x.name.as_str()))
//         .collect();

//     // TODO disambiguate names when they're the same
//     let mut name_map = HashMap::new();
//     for track in &object_vec {
//         name_map.insert(track.name.clone(), track.track_id.clone());
//     }

//     let recipe = Recipe::new(object_vec, name_map);

//     recipe
// }

pub fn get_playable_tracks(tracks: Vec<PlaylistItem>) -> Vec<FullTrack> {
    let mut playable_tracks = Vec::new();

    for item in tracks {
        if let Some(PlayableItem::Track(t)) = item.track {
            playable_tracks.push(t);
        }
    }

    playable_tracks
}

// pub async fn get_features(spotify: &mut Spotify, track_ids: Vec<&Id<Track>>) -> Vec<AudioFeatures> {
//     // batch the feature gathering
//     let window: usize = 50;
//     let playlist_length = track_ids.len();
//     let requests = playlist_length / window;

//     info!(
//         "Playlist length: {}, request count: {}",
//         playlist_length, requests
//     );

//     let mut all_features: Vec<AudioFeatures> = Vec::new();

//     let mut ranges = Vec::new();

//     for i in 0..requests {
//         ranges.push(i * 50..(i + 1) * 50);
//     }

//     if window * requests < playlist_length {
//         ranges.push(requests * 50..playlist_length);
//     }

//     for r in ranges {
//         info!("Gathering data from slice: {:?}", r);
//         let track_slice = track_ids[r].to_vec();
//         sleep(Duration::from_millis(100)).await;
//         let features = spotify.tracks_features(track_slice).await.unwrap();
//         if let Some(mut features) = features {
//             all_features.append(&mut features);
//         }
//     }

//     all_features
// }

// pub async fn collect_all_from_page<T>(spotify: &mut AuthCodeSpotify) {
//     let limit = 50;
//     let mut offset = 0;
//     info!("Gathering playlists");
//     loop {
//         let page = spotify
//             .current_user_playlists_manual(Some(limit), Some(offset))
//             .await
//             .unwrap();
//         for item in page.items {
//             debug!("* {}", item.name);

//             if item.name == playlist_name {
//                 info!("Found playlist: {} with id: {}", playlist_name, item.id);
//                 return Some(item);
//             }
//         }

//         sleep(Duration::from_millis(25)).await;

//         // The iteration ends when the `next` field is `None`. Otherwise, the
//         // Spotify API will keep returning empty lists from then on.
//         if page.next.is_none() {
//             break;
//         }

//         offset += limit;
//     }

//     info!("Failed to find playlist: {}", playlist_name);

//     None
// }

pub fn find_user_playlist(
    spotify: &AuthCodeSpotify,
    playlist_name: &str,
) -> Option<SimplifiedPlaylist> {
    let limit = 50;
    let mut offset = 0;
    info!("Gathering playlists");
    loop {
        let page = spotify
            .current_user_playlists_manual(Some(limit), Some(offset))
            // .await
            .unwrap();
        for item in page.items {
            debug!("* {}", item.name);

            if item.name == playlist_name {
                info!("Found playlist: {} with id: {}", playlist_name, item.id);
                return Some(item);
            }
        }

        // sleep(Duration::from_millis(25)).await;

        throttle();

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

// For whatever reason, the stream adaptors for the spotify API seem to hang for me.
// With this, we can emulate the stream API without needing to actually use the stream
// api, and use all of the `manual` endpoints.
pub fn fake_stream<'a, T, F: Fn(u32, u32) -> ClientResult<Page<T>>, C: FnMut(Page<T>) -> bool>(
    func: F,
    mut collector: C,
) -> ClientResult<()> {
    let limit = 50;
    let mut offset = 0;
    // let mut output = Vec::new();
    loop {
        let page = func(limit, offset)?;

        let should_break = page.next.is_none();

        if !collector(page) {
            break;
        };

        // tokio::time::sleep(Duration::from_millis(50)).await;

        throttle();

        // The iteration ends when the `next` field is `None`. Otherwise, the
        // Spotify API will keep returning empty lists from then on.
        if should_break {
            break;
        }

        offset += limit;
    }

    Ok(())
}

pub fn get_playlist_tracks(
    spotify: &AuthCodeSpotify,
    playlist: &SimplifiedPlaylist,
) -> Vec<PlaylistItem> {
    let limit = 50;
    let mut offset = 0;
    let mut output = Vec::new();
    let id: PlaylistId = playlist.id.clone();
    info!("Gathering track info from playlist: {}", playlist.name);
    loop {
        let mut page = spotify
            .playlist_items_manual(id.clone(), None, None, Some(limit), Some(offset))
            // .await
            .unwrap();

        // tokio::time::sleep(Duration::from_millis(50));

        throttle();

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

pub fn get_playlist_tracks_by_id(
    spotify: &AuthCodeSpotify,
    id: PlaylistId,
) -> ClientResult<Vec<PlaylistItem>> {
    let limit = 50;
    let mut offset = 0;
    let mut output = Vec::new();
    info!("Gathering track info from playlist: {}", id);
    loop {
        let mut page =
            spotify.playlist_items_manual(id.clone(), None, None, Some(limit), Some(offset))?;

        throttle();

        output.append(&mut page.items);

        // The iteration ends when the `next` field is `None`. Otherwise, the
        // Spotify API will keep returning empty lists from then on.
        if page.next.is_none() {
            break;
        }

        offset += limit;
    }

    info!("Gathered tracks for playlist: {}", id);

    Ok(output)
}

pub fn get_user_playlist(spotify: &AuthCodeSpotify, name: &str) -> ClientResult<FullPlaylist> {
    let current_user: PrivateUser = spotify.current_user().unwrap();

    let mut stream = spotify.current_user_playlists();

    // TODO replace with the stream adapter
    while let Some(playlist) = stream.next() {
        // todo!()

        let playlist = playlist?;

        if playlist.name == name {
            info!(
                "Found existing playlist for user: {:?} with id: {}",
                current_user.display_name, current_user.id
            );

            return spotify.user_playlist(current_user.id, Some(playlist.id), None);
            // .await;
        }
    }

    panic!("Playlist not found!");
}

pub fn get_or_create_empty_playlist(
    spotify: &AuthCodeSpotify,
    name: &str,
    description: Option<String>,
) -> ClientResult<FullPlaylist> {
    let current_user: PrivateUser = spotify.current_user().unwrap();

    let mut stream = spotify.current_user_playlists();

    // TODO replace with the stream adapter
    while let Some(playlist) = stream.next() {
        // todo!()

        let playlist = playlist?;

        if playlist.name == name {
            info!(
                "Found existing playlist for user: {:?} with id: {}",
                current_user.display_name, current_user.id
            );

            return spotify.user_playlist(current_user.id, Some(playlist.id), None);
            // .await;
        }
    }

    info!(
        "Creating empty playlist for user: {:?} with id: {}",
        current_user.display_name, current_user.id
    );

    let user_id = current_user.id;

    spotify.user_playlist_create(
        user_id,
        name,
        Some(false),
        Some(false),
        description.as_ref().map(|x| x.as_str()),
    )
    // .await
}

const US_MARKET: Option<Market> = Some(Market::Country(Country::UnitedStates));

#[derive(Debug, Clone)]
pub struct AlbumSearchCriteria {
    album_name: String,
    artist: Option<String>,
    uri: Option<String>,
    filters: Vec<String>,
}

impl Custom for AlbumSearchCriteria {}

impl AlbumSearchCriteria {
    pub fn new(album_name: impl Into<String>) -> Self {
        Self {
            album_name: album_name.into(),
            artist: None,
            uri: None,
            filters: Vec::new(),
        }
    }

    pub fn raw(
        album_name: String,
        artist: Option<String>,
        uri: Option<String>,
        filters: Vec<String>,
    ) -> SpotifyExpr {
        SpotifyExpr::Album(Self {
            album_name,
            artist,
            uri,
            filters,
        })
    }

    pub fn with_artist(mut self, artist: impl Into<String>) -> Self {
        self.artist = Some(artist.into());
        self
    }

    pub fn get_album_tracks(
        &self,
        spotify: &AuthCodeSpotify,
    ) -> ClientResult<Vec<SimplifiedTrack>> {
        //  If the designation has a URI, prefer it over all others
        if let Some(uri) = &self.uri {
            let mut stream = spotify.album_track(AlbumId::from_uri(&uri).unwrap(), US_MARKET);

            let mut tracks = Vec::new();

            // TODO: Replace with the stream adapter
            while let Some(track) = stream.next() {
                let track = track?;
                tracks.push(track);
            }

            return Ok(tracks);
        }

        let mut search_query = format!("{}", self.album_name);

        if let Some(artist_search_criteria) = &self.artist {
            search_query += &format!(" artist:{}", artist_search_criteria);
        }

        let search_results = spotify.search(
            &self.album_name,
            SearchType::Album,
            Some(Market::Country(Country::UnitedStates)),
            None,
            None,
            None,
        )?;
        // .await?;

        if let SearchResult::Albums(simplified_album) = search_results {
            let simplified_albums = simplified_album.items;

            for result in simplified_albums {
                let mut stream = spotify.album_track(result.id.unwrap(), US_MARKET);

                let mut tracks = Vec::new();

                // TODO: Replace with the stream adapter
                while let Some(track) = stream.next() {
                    let track = track?;
                    tracks.push(track);
                }

                return Ok(tracks);
            }

            todo!("Didn't find a match!");
        } else {
            todo!("Handle the case where our search returns nothing!")
        }
    }
}

impl<T: Into<String>> From<T> for AlbumSearchCriteria {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub struct PlaylistCreation {
    name: String,
    description: Option<String>,
    archive_to: Option<String>,
    values: Vec<SpotifyExpr>,
}

impl Custom for PlaylistCreation {}

impl PlaylistCreation {
    pub fn new(
        name: String,
        description: Option<String>,
        archive_to: Option<String>,
        values: Vec<SpotifyExpr>,
    ) -> Self {
        Self {
            name,
            description,
            archive_to,
            values,
        }
    }

    pub fn build(&self, spotify: &AuthCodeSpotify) -> ClientResult<()> {
        // call out to API here
        // todo!()

        let playlist = get_or_create_empty_playlist(spotify, &self.name, self.description.clone())?;

        wait!();

        let mut total_tracks: Vec<TrackId> = Vec::new();

        for value in &self.values {
            throttle();

            match value {
                SpotifyExpr::Track(track) => {
                    let track = track.search(spotify)?;

                    total_tracks.push(track.id.unwrap());
                }
                SpotifyExpr::Album(album) => {
                    let tracks = album.get_album_tracks(spotify)?;

                    // log::debug!("{:#?}", tracks);

                    // TODO: Case insensitive searching
                    let filters = album.filters.iter().collect::<HashSet<_>>();

                    let count = total_tracks.len();

                    for track in tracks {
                        if filters.is_empty() {
                            total_tracks.push(track.id.unwrap());
                            continue;
                        }

                        if filters.contains(&track.name) {
                            total_tracks.push(track.id.unwrap());
                            continue;
                        }

                        for value in &filters {
                            // Naively do this until I implement the fuzzy matching
                            if track.name.starts_with(value.as_str()) {
                                total_tracks.push(track.id.unwrap());
                                break;
                            }
                        }
                    }

                    if !filters.is_empty() {
                        let found = total_tracks.len() - count;

                        // log::info!("Applying filter for selected album: {}", album.album_name);

                        if found != filters.len() {
                            log::error!(
                                "Album: {} - Found tracks did not match selected subset!",
                                album.album_name
                            )
                        }
                    }
                }
            }
        }

        replace_contents_of_playlist(spotify, playlist.id, total_tracks.clone())?;

        // If this is specified, archive the given playlist to a destination location
        if let Some(archive_to) = &self.archive_to {
            let playlist =
                get_or_create_empty_playlist(spotify, archive_to, self.description.clone())?;

            replace_contents_of_playlist(spotify, playlist.id, total_tracks)?;
        }

        Ok(())

        // playlist.id

        // todo!()
    }
}

// Attempt to create some sort of abstraction through a workflow
// like this, where finish applies the computation?
pub struct PlaylistBuilder {
    name: String,
    albums: Vec<AlbumSearchCriteria>,
    tracks: Vec<String>,

    all_discography: Vec<String>,
}

impl PlaylistBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            albums: Vec::new(),
            tracks: Vec::new(),
            all_discography: Vec::new(),
        }
    }

    pub fn with_album(mut self, name: impl Into<AlbumSearchCriteria>) -> Self {
        self.albums.push(name.into());
        self
    }

    pub fn with_track(mut self, name: impl Into<String>) -> Self {
        self.tracks.push(name.into());
        self
    }

    pub fn shuffle_by_album(mut self) -> Self {
        self.albums.shuffle(&mut thread_rng());
        self
    }

    pub fn with_all_discography(mut self, name: impl Into<String>) -> Self {
        self.all_discography.push(name.into());
        self
    }

    pub fn build(self, spotify: &AuthCodeSpotify) -> ClientResult<()> {
        // call out to API here
        // todo!()

        let playlist = get_or_create_empty_playlist(spotify, &self.name, None)?;

        wait!();

        let mut total_tracks: Vec<TrackId> = Vec::new();

        for album in self.albums {
            log::info!("Fetching album: {:?}", album);

            let tracks = album.get_album_tracks(spotify)?;

            wait!();

            for track in tracks {
                total_tracks.push(track.id.unwrap())
            }
        }

        for artist in self.all_discography {
            log::info!("Fetching all discography for: {}", artist);

            let albums = get_all_albums_for_artist(spotify, &artist).unwrap();

            wait!();

            let hydrated = hydrate_albums(spotify, albums).unwrap();

            wait!();

            let ranked = rank_full_albums_by_popularity(hydrated);

            let filtered = ranked
                .into_iter()
                .map(|x| filter_to_just_tracks_that_contain_artist(artist.clone(), x))
                .collect::<Vec<_>>();

            for value in filtered {
                total_tracks.extend(value.filtered_tracks.iter().filter_map(|x| {
                    if x.is_playable? {
                        x.id.clone()
                    } else {
                        None
                    }
                }));
            }
        }

        // Ok(())

        replace_contents_of_playlist(spotify, playlist.id, total_tracks)

        // playlist.id

        // todo!()
    }
}

/// Archive playlist at a point in time. This loses any of the metadata associated with the playlist,
/// but is helpful for snapshotting what the playlist has in it.
pub fn archive_playlist(
    spotify: &AuthCodeSpotify,
    source_playlist_name: &str,
    destination_playlist_name: &str,
) -> ClientResult<()> {
    let source = get_user_playlist(spotify, source_playlist_name)?;

    let tracks = get_playlist_tracks_by_id(spotify, source.id.clone())?;

    let playlist = get_or_create_empty_playlist(spotify, destination_playlist_name, None)?;

    // Replace the contents of the new playlist, with the contents of the old one
    replace_contents_of_playlist(
        spotify,
        playlist.id,
        tracks
            .into_iter()
            .filter_map(|x| match x.track {
                Some(PlayableItem::Track(t)) => t.id,
                _ => None,
            })
            .collect(),
    )
}

/// Replaces the contents of the given playlist with all of the tracks from the vector of track ids
pub fn replace_contents_of_playlist<'a>(
    spotify: &AuthCodeSpotify,
    playlist_id: PlaylistId<'a>,
    track_ids: Vec<TrackId<'a>>,
) -> ClientResult<()> {
    info!("Replacing contents of playlist");
    info!("Replacing with track list of length: {}", track_ids.len());

    if track_ids.len() <= 100 {
        let collected = track_ids.into_iter().map(|x| x.into()).collect::<Vec<_>>();

        return spotify.playlist_replace_items(playlist_id, collected);
    } else {
        // Nuke the playlist state, override
        spotify.playlist_replace_items(playlist_id.clone(), None)?;

        wait!();

        for chunk in track_ids.chunks(100) {
            // Add the playlist items
            spotify.playlist_add_items(
                playlist_id.clone(),
                chunk.into_iter().map(|x| x.clone().into()), // .collect::<Vec<_>>(),
                None,
            )?;

            throttle();
        }
    }

    Ok(())
}

// Fetch all of the albums for
pub fn get_all_albums_for_artist(
    spotify: &AuthCodeSpotify,
    artist: &str,
) -> ClientResult<Vec<SimplifiedAlbum>> {
    log::info!("Searching for: {}", artist);
    let search_results = spotify
        .search(artist, SearchType::Artist, US_MARKET, None, None, None)
        // .await
        .unwrap();

    wait!();
    let mut albums = Vec::new();

    if let SearchResult::Artists(paginated_artists) = search_results {
        log::info!("Found search results...");

        let top_result = paginated_artists.items[0].clone();

        let fetcher = move |limit, offset| {
            spotify.artist_albums_manual(
                top_result.id.clone(),
                None,
                US_MARKET,
                Some(limit),
                Some(offset),
            )
        };

        fake_stream(fetcher, |page| {
            for item in page.items {
                debug!("* {}", item.name);
                albums.push(item);
            }

            true
        })?;

        log::info!("Collected albums for artist: {}", artist);

        return Ok(albums);
    }

    todo!()
}

// Turn a sequence of simplified albums into a vector of full albums
pub fn hydrate_albums(
    spotify: &AuthCodeSpotify,
    albums: Vec<SimplifiedAlbum>,
) -> ClientResult<Vec<FullAlbum>> {
    // Chunk the albums into groups of 20

    let mut results = Vec::new();

    for chunk in albums.chunks(20) {
        results.extend(spotify.albums(chunk.into_iter().filter_map(|x| x.id.clone()), US_MARKET)?);
        throttle();
    }

    Ok(results)
}

// Rank the albums by popularity
pub fn rank_full_albums_by_popularity(mut albums: Vec<FullAlbum>) -> Vec<FullAlbum> {
    albums.sort_by(|a, b| b.popularity.cmp(&a.popularity));

    albums
}

#[derive(Debug)]
pub struct SimplifiedAlbumByArtistWithTracks {
    pub artist: String,
    pub album: FullAlbum,
    pub filtered_tracks: Vec<SimplifiedTrack>,
}

pub fn filter_to_just_tracks_that_contain_artist(
    artist: String,
    album: FullAlbum,
) -> SimplifiedAlbumByArtistWithTracks {
    let mut filtered_tracks = Vec::new();

    for track in &album.tracks.items {
        for track_artist in &track.artists {
            if artist == track_artist.name {
                filtered_tracks.push(track.clone());
                break;
            }
        }
    }

    SimplifiedAlbumByArtistWithTracks {
        artist,
        album,
        filtered_tracks,
    }
}

// There is a specific endpoint for this
// TODO: Create curated spotify playlist index
// pub fn index_curated_spotify_playlists(
//     spotify: &AuthCodeSpotify,
// ) -> ClientResult<HashMap<String, SimplifiedPlaylist>> {
//     let mut results = HashMap::with_capacity(1500);

//     // Index the playlists made by spotify
//     let fetcher = move |limit, offset| {
//         spotify.user_playlists_manual(
//             UserId::from_uri("44b56d41c99740bf").unwrap(),
//             Some(limit),
//             Some(offset),
//         )
//     };

//     fake_stream(fetcher, |page| {
//         for item in page.items {
//             debug!("* {}", item.name);
//             results.insert(item.name.clone(), item);
//         }

//         true
//     })?;

//     Ok(results)
// }

// Snag top 10 tracks for a given artist
pub fn top_ten_tracks_for_artist(
    spotify: &AuthCodeSpotify,
    artist: ArtistId<'_>,
) -> ClientResult<Vec<FullTrack>> {
    spotify.artist_top_tracks(artist, US_MARKET)
}

#[derive(Debug, Clone)]
pub struct TrackSearchCriteria {
    name: String,
    album: Option<String>,
    artist: Option<String>,
}

impl Custom for TrackSearchCriteria {}

impl TrackSearchCriteria {
    pub fn new(name: String, album: Option<String>, artist: Option<String>) -> SpotifyExpr {
        SpotifyExpr::Track(Self {
            name,
            album,
            artist,
        })
    }

    pub fn search(&self, spotify: &AuthCodeSpotify) -> ClientResult<FullTrack> {
        let mut query = self.name.clone();

        if let Some(album) = &self.album {
            query += &format!(" album:{}", album);
        }

        if let Some(artist) = &self.artist {
            query += &format!(" artist:{}", artist);
        }

        let search_results = spotify.search(
            &self.name,
            SearchType::Track,
            Some(Market::Country(Country::UnitedStates)),
            None,
            None,
            None,
        )?;

        if let SearchResult::Tracks(simplified_track) = search_results {
            let simplified_tracks = simplified_track.items;

            return Ok(simplified_tracks.into_iter().next().unwrap());
        } else {
            todo!("Handle the case where our search returns nothing!")
        }
    }
}

// Spotify AST

#[derive(Clone)]
pub enum SpotifyExpr {
    Track(TrackSearchCriteria),
    Album(AlbumSearchCriteria),
}

impl Custom for SpotifyExpr {}

// enum SpotifyValue {}

// https://open.spotify.com/user/spotify?si=44b56d41c99740bf

// pub async fn create_or_replace_contents_of_playlist(
//     spotify: &mut Spotify,
//     name: &str,
//     playlist_id: Option<&PlaylistId>,
//     track_ids: impl Iterator<Item = &TrackId>,
// ) -> ClientResult<()> {
//     if let Some(playlist_id) = playlist_id {
//         return replace_contents_of_playlist(spotify, playlist_id, track_ids).await;
//     } else {
//         let playlist = find_user_playlist(spotify, name).await;
//         sleep(Duration::from_millis(100)).await;

//         match playlist {
//             Some(p) => {
//                 let playlist_id = Id::from_id(&p.id).unwrap();
//                 return replace_contents_of_playlist(spotify, playlist_id, track_ids).await;
//             }
//             None => {
//                 let created_playlist = create_empty_playlist(spotify, name)
//                     .await
//                     .expect("Failed to create playlist");

//                 sleep(Duration::from_millis(100)).await;

//                 let playlist_id = Id::from_id(created_playlist.id.as_str()).unwrap();
//                 return replace_contents_of_playlist(spotify, playlist_id, track_ids).await;
//             }
//         };
//     }
// }

// enum SpotifyValue {
//     Playlist()
// }
