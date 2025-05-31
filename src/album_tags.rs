// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
use std::fmt;
use toml::{Table, Value};

use crate::config::ConfigError;
use crate::TrackTags;

#[derive(Debug)]
pub struct AlbumTags {
    album_name: String,
    artist_name: String,
    year: u32,
    genre: String,
    tracks: Vec<String>,
}

impl fmt::Display for AlbumTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {:#?}, {})", self.album_name, self.artist_name, self.year, self.tracks, self.genre)
    }
}

fn get_string_array(table: &Table, key: &str) -> Result<Vec<String>, ConfigError> {
    match table.get(key) {
        Some(Value::Array(arr)) => {
            let strings: Result<Vec<String>, _> = arr.iter()
                .map(|v| v.as_str().map(|s| s.to_string()).ok_or("..."))
                .collect();

            strings.map_err(Into::into)
        }
        _ => Err(ConfigError::MissingKey(String::from(format!("Missing key '{}'", key))))
    }
}

impl AlbumTags {
    pub fn from_toml(table: Table) -> Result<Self, ConfigError> {
        if !table.contains_key("artist") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'artist'.")));
        }
        if !table.contains_key("album") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'album'.")));
        }
        if !table.contains_key("year") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'year'.")));
        } else {
            let n = table.get("year").unwrap().to_string().parse::<u32>();
            if n.is_err() {
                return Err(ConfigError::TypeError(String::from("Expected numerical value for key 'year'.")));
            }
        }
        if !table.contains_key("genre") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'genre'.")));
        }

        let tracks = match get_string_array(&table, "tracks") {
            Ok(arr) => arr,
            Err(e) => return Err(e)
        };

        Ok(AlbumTags {
            artist_name: table.get("artist").unwrap().to_string(),
            album_name: table.get("album").unwrap().to_string(),
            year: table.get("year").unwrap().to_string().parse::<u32>().unwrap(),
            genre: table.get("genre").unwrap().to_string(),
            tracks: tracks,
        })
    }
}

pub fn to_track_tags(album: AlbumTags) -> Vec<TrackTags> {
    let mut tags = Vec::new();
    let mut index = 0;
    let track_total = album.tracks.len();
    while index < track_total {
        tags.push(TrackTags {
            album_name: album.album_name.clone(),
            artist_name: album.artist_name.clone(),
            year: album.year.clone(),
            track_name: album.tracks[index].clone(),
            genre: album.genre.clone(),
            track_number: (index + 1) as u32,
            track_total: track_total as u32,
        });

        index += 1;
    }
    tags
}