use toml::{Table, Value};

use crate::TrackTags;

#[derive(Debug)]
pub enum ConfigError {
    MissingKey(String),
    TypeError(String),
}

impl From<&str> for ConfigError {
    fn from(s: &str) -> Self {
        ConfigError::TypeError(s.to_string())
    }
}

#[derive(Debug)]
pub struct AlbumTags {
    album_name: String,
    artist_name: String,
    year: String,
    genre: String,
    tracks: Vec<String>,
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
        if !table.contains_key("artist_name") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'artist_name'.")));
        }
        if !table.contains_key("album_name") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'album_name'.")));
        }
        if !table.contains_key("year") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'year'.")));
        }
        if !table.contains_key("genre") {
            return Err(ConfigError::MissingKey(String::from("Missing key 'genre'.")));
        }

        let tracks = match get_string_array(&table, "tracks") {
            Ok(arr) => arr,
            Err(e) => return Err(e)
        };

        Ok(AlbumTags {
            artist_name: table.get("artist_name").unwrap().to_string(),
            album_name: table.get("artist_name").unwrap().to_string(),
            year: table.get("year").unwrap().to_string(),
            genre: table.get("genre").unwrap().to_string(),
            tracks: tracks,
        })
    }
}

pub fn to_track_tags(album: AlbumTags) -> Vec<TrackTags> {
    let mut tags = Vec::new();
    let mut index = 0;
    while index < album.tracks.len() {
        tags.push(TrackTags {
            album_name: album.album_name.clone(),
            artist_name: album.artist_name.clone(),
            year: album.year.clone(),
            track_name: album.tracks[index].clone(),
            genre: album.genre.clone(),
        });

        index += 1;
    }
    tags
}