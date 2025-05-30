// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

extern crate taglib;

use std::env;
use config::load_config_from_file;

pub mod config;
pub mod track_tags;
pub mod album_tags;
pub mod fs_utils;

use track_tags::{assign_tags_to_track, TrackTags};
use album_tags::{to_track_tags, AlbumTags};
use fs_utils::get_audio_files_in_directory;

fn main() {
    let cwd = env::current_dir().unwrap();
    println!("Current dir is {}.", cwd.display());

    let config_path = "scratch/config.toml";
    let album_path = "scratch/album";

    // there's surely a better way to do this??
    let album_tags: AlbumTags = match load_config_from_file("scratch/config.toml") {
        Ok(cfg) => cfg,
        _ => {
            panic!("Could not read config {config_path}.")
        }
    };

    println!("{}", album_tags);

    println!("Audio files");
    let mut tracks = match get_audio_files_in_directory(album_path) {
        Ok(entries) => {
            entries.iter()
                .map(|de| String::from(de.path().as_os_str().to_str().unwrap()))
                .collect::<Vec<String>>()
        }
        Err(e) => {
            panic!("Could not read audio files [{:?}]", e)
        }
    };
    tracks.sort();
    let track_tags = to_track_tags(album_tags);

    assert!(track_tags.len() == tracks.len());

    for i in 0..tracks.len() {
        let track_path = tracks[i].as_str();
        let tags = &track_tags[i];
        match assign_tags_to_track(tags, track_path) {
            Ok(_) => {
                println!("Asigned {}", track_path);
            },
            Err(e) => {
                println!("Error: {:#?}", e);
            }
        }
    }
}
