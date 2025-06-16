// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
use std::fmt;
use toml::{Table, Value};

use crate::config::ConfigError;
use crate::TrackTags;

#[derive(Debug, Clone)]
pub struct AlbumTags {
    pub album_name: String,
    pub artist_name: String,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub tracks: Vec<String>,
    pub disc_total: Option<u32>,
    pub tracks_per_disc: Option<Vec<u32>>,
}

impl fmt::Display for AlbumTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "album_name: {}\n", self.album_name)
        .and_then(|_|
            write!(f, "artist_name: {}\n", self.artist_name)
        )
        .and_then(|_|
            write!(f, "year: {}\n", self.year.and_then(|x| Some(x.to_string())).unwrap_or(String::from("N/A")))
        )
        .and_then(|_|
            write!(f, "genre: {}\n", self.genre.clone().unwrap_or(String::from("N/A")))
        )
        .and_then(|_|
            write!(f, "tracks: {:?}\n", self.tracks)
        )
        .and_then(|_|
            write!(f, "disc_total: {}\n", self.disc_total.and_then(|x| Some(x.to_string())).unwrap_or(String::from("N/A")))
        )
        .and_then(|_|
            write!(f, "tracks_per_disc: {:?}", self.tracks_per_disc.clone().unwrap_or(vec![]))
        )
        // write!(f, "({}, {}, {:?}, {:#?}, {:?})", self.album_name, self.artist_name, self.year, self.tracks, self.genre)
            // .and_then(|_| write!(f, "done"))
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

fn get_u32_array<T>(arr: &Vec<Value>) -> Vec<u32> {
    let xs: Vec<u32> = arr
        .iter()
        .map(|x| x.as_integer().and_then(|i| u32::try_from(i).ok()).unwrap())
        .collect();
    xs
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

        let track_total = tracks.len() as u32;

        let disc_total = match table.get("disc_total") {
            Some(x) => x.to_string().parse::<u32>().ok(),
            None => {
                println!("Missing key 'disc_total'");
                Some(1)
            }
        };
        let tracks_per_disc = match table.get("tracks_per_disc") {
            Some(Value::Array(x)) => get_u32_array::<u32>(x),
            _ => {
                println!("Missing array key: 'tracks_per_disc'");
                vec![track_total]
            }
        };

        // FIXME: this is not very safe
        Ok(AlbumTags {
            artist_name: table.get("artist").unwrap().as_str().unwrap().to_string(),
            album_name: table.get("album").unwrap().as_str().unwrap().to_string(),
            year: table.get("year").and_then(|o| o.to_string().parse::<u32>().ok()),
            genre: table.get("genre").and_then(|o| Some(o.to_string())),
            tracks: tracks,
            disc_total: disc_total,
            tracks_per_disc: Some(tracks_per_disc),
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
        let mut disc_num = 1;
        if let Some(ref tpd) = album.tracks_per_disc {
            disc_num = get_disc_number(&tpd, index as u32);   
        }
        tags.push(TrackTags {
            album_name: album.album_name.clone(),
            artist_name: album.artist_name.clone(),
            year: album.year.clone(),
            track_name: album.tracks[index].clone(),
            genre: album.genre.clone(),
            track_number: (index + 1) as u32,
            track_total: track_total as u32,
            disc_number: Some(disc_num),
            disc_total: album.disc_total,
        });

        index += 1;
    }
    tags
}