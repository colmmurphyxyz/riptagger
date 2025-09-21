// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
use std::fmt;
use std::path::Path;
use toml::{Table, Value};

use crate::toml_helpers::{get_i64_array, get_i64_value, get_string_array, get_single_or_array_string};
use crate::config::ConfigError;
use crate::fs_utils::get_current_directory;
use crate::TrackTags;

#[derive(Debug, Clone)]
pub struct AlbumTags {
    pub album_name: Option<String>,
    pub artist_name: Option<String>,
    pub year: Option<u32>,
    pub genre: Vec<String>,
    pub picture_path: Option<String>,
    pub tracks: Vec<String>,
    pub disc_total: Option<u32>,
    pub tracks_per_disc: Option<Vec<u32>>,
}

impl fmt::Display for AlbumTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "album_name: {:?}\n", self.album_name)
        .and_then(|_|
            write!(f, "artist_name: {:?}\n", self.artist_name)
        )
        .and_then(|_|
            write!(f, "year: {:?}\n", self.year)
        )
        .and_then(|_|
            write!(f, "genre: {:?}\n", self.genre)
        )
        .and_then(|_|
            write!(f, "tracks: {:?}\n", self.tracks)
        )
        .and_then(|_|
            write!(f, "disc_total: {:?}\n", self.disc_total)
        )
        .and_then(|_|
            write!(f, "tracks_per_disc: {:?}", self.tracks_per_disc)
        )
        .and_then( |_|
            write!(f, "picture_path: {:?}", self.picture_path)
        )
    }
}

impl AlbumTags {
    pub fn from_toml(table: Table) -> Result<Self, ConfigError> {
        if !table.contains_key("tracks") {
            return Err(ConfigError::MissingKey(String::from("tracks")))
        }

        let tracks = match get_string_array(&table, &["tracks"]) {
            Ok(arr) => arr,
            Err(e) => return Err(e)
        };

        let disc_total = match table.get("disc_total") {
            Some(x) => x.to_string().parse::<u32>().ok(),
            _ => None
        };
        let tracks_per_disc = match get_i64_array(&table, &["tracks_per_disc"]) {
            Ok(arr) => Some(arr),
            _ => None
        }
        let tracks_per_disc: Option<Vec<u32>> = match table.get("tracks_per_disc") {
            Some(Value::Array(x)) => Some(get_i64_array(x)),
            _ => None
        };

        let mut pic_path_str: Option<String> = None;
        // picture path should be relative to the config file
        if let Some(relative_picture_path) = table.get("picture").and_then(|p| Some(p.as_str())).and_then(|p| p.to_owned()) {
            let cwd = get_current_directory();
            if let Ok(pic_path_root) = cwd {
                pic_path_str = pic_path_root.as_path().join(Path::new(&relative_picture_path)).to_str().and_then(|s| Some(s.to_string()));
            }
        }

        // FIXME: this is not very safe
        Ok(AlbumTags {
            artist_name: get_string_value(&table, "artist"),
            album_name: get_string_value(&table, "album"),
            year: table.get("year").and_then(|o| o.to_string().parse::<u32>().ok()),
            genre: get_single_or_array_string(&table, "genre").unwrap_or(vec![]),
            picture_path: pic_path_str,
            tracks: tracks,
            disc_total: disc_total,
            tracks_per_disc: tracks_per_disc,
        })
    }
}

fn get_disc_number(tracks_per_disc: &Vec<u32>, track_num: u32) -> u32 {
    let mut x = 0;
    let mut i: usize = 0;
    while x <= track_num && i < tracks_per_disc.len() {
        x += tracks_per_disc[i];
        i += 1;
    }
    i as u32
}

pub fn to_track_tags(album: AlbumTags) -> Vec<TrackTags> {
    let mut tags = Vec::new();
    let mut index = 0;
    let track_total = album.tracks.len();
    while index < track_total {
        let disc_num = match &album.tracks_per_disc {
            Some(tpd) => Some(get_disc_number(&tpd, index as u32)),
            None => None
        };
        tags.push(TrackTags {
            album_name: album.album_name.clone(),
            artist_name: album.artist_name.clone(),
            year: album.year.clone(),
            track_name: album.tracks[index].clone(),
            genre: album.genre.clone(),
            picture_path: album.picture_path.clone(),
            track_number: Some((index + 1) as u32),
            track_total: Some(track_total as u32),
            disc_number: disc_num,
            disc_total: album.disc_total,
        });

        index += 1;
    }
    tags
}