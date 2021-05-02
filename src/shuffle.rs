use rspotify::{
    client::Spotify,
    model::{idtypes::Track, IdBuf},
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::{HashMap, HashSet};

use steel_derive::Steel;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub(crate) track_list: Vec<SpotifyTrack>,
    pub(crate) groups: Vec<Vec<SpotifyTrack>>,
    pub(crate) map: HashMap<String, String>,
}

impl Recipe {
    pub fn new(track_list: Vec<SpotifyTrack>, map: HashMap<String, String>) -> Self {
        Self {
            // track_list: track_list.into_iter().map(Unit::Track).collect(),
            track_list,
            groups: Vec::new(),
            map,
        }
    }

    pub fn add_group_by_name(&mut self, group: Vec<&str>) -> &mut Self {
        let group = group
            .into_iter()
            .map(|x| SpotifyTrack::new(&self.map[x], x))
            .collect::<Vec<_>>();

        let mut hs: HashSet<&str> = HashSet::new();
        for track in &group {
            hs.insert(track.track_id.as_str());
        }

        self.track_list
            .retain(|x| !hs.contains(x.track_id.as_str()));

        self.groups.push(group);
        self
    }

    pub fn add_group(&mut self, group: Vec<SpotifyTrack>) -> &mut Self {
        let mut hs: HashSet<&str> = HashSet::new();
        for track in &group {
            hs.insert(track.track_id.as_str());
        }

        self.track_list
            .retain(|x| !hs.contains(x.track_id.as_str()));
        self.groups.push(group);
        self
    }

    pub fn shuffle(&self) -> Vec<SpotifyTrack> {
        let mut unit_list = Vec::new();
        for track in &self.track_list {
            unit_list.push(Unit::Track(track))
        }

        for group in &self.groups {
            unit_list.push(Unit::Group(group))
        }

        unit_list.shuffle(&mut thread_rng());

        let mut output = Vec::new();
        for unit in unit_list {
            match unit {
                Unit::Track(t) => output.push(t.clone()),
                Unit::Group(g) => {
                    let mut cloned = g.iter().cloned().collect();
                    output.append(&mut cloned);
                }
            }
        }

        output
    }
}

#[derive(Clone, PartialEq, PartialOrd, Hash, Eq, Debug, Steel)]
pub struct SpotifyTrack {
    pub track_id: String,
    pub name: String,
}

impl SpotifyTrack {
    pub fn new(track_id: &str, name: &str) -> Self {
        Self {
            track_id: track_id.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn get_track_id(self) -> String {
        self.track_id
    }

    pub fn get_name(self) -> String {
        self.name
    }
}

// pub struct Bunch(Vec<Unit>);

pub enum Unit<'a> {
    Track(&'a SpotifyTrack),
    Group(&'a [SpotifyTrack]),
}

#[cfg(test)]
mod recipe_tests {
    use super::*;

    #[test]
    fn basic_test() {
        let track_list = vec![
            SpotifyTrack::new("0", "foo"),
            SpotifyTrack::new("1", "bar"),
            SpotifyTrack::new("2", "baz"),
            SpotifyTrack::new("3", "bat"),
            SpotifyTrack::new("4", "quux"),
            SpotifyTrack::new("5", "apple"),
            SpotifyTrack::new("6", "sauce"),
        ];

        let mut recipe = Recipe::new(track_list, HashMap::new());
        recipe.add_group(vec![
            SpotifyTrack::new("0", "foo"),
            SpotifyTrack::new("1", "bar"),
            SpotifyTrack::new("2", "baz"),
        ]);

        let output = recipe.shuffle();

        println!("{:#?}", output);

        assert!(false);
    }
}
