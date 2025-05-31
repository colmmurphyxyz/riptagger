// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

extern crate taglib;

pub mod config;
pub mod track_tags;
pub mod album_tags;
pub mod fs_utils;

use clap::Parser;

use config::load_config_from_file;
use track_tags::{assign_tags_to_track, TrackTags};
use album_tags::{to_track_tags, AlbumTags};
use fs_utils::{get_audio_files_in_directory, rename_audio_file};

#[derive(Parser)]
struct Cli {
    config_path: String,
    album_path: String,
}

fn main() {
    let args = Cli::parse();
    println!("args: config {:?}, path {:?}", args.config_path, args.album_path);

    // there's surely a better way to do this??
    let album_tags: AlbumTags = match load_config_from_file(&args.config_path) {
        Ok(cfg) => cfg,
        _ => {
            panic!("Could not read config {0}.", &args.config_path)
        }
    };

    println!("{}", album_tags);

    println!("Audio files");
    let mut tracks = match get_audio_files_in_directory(&args.album_path) {
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
                continue
            }
        }
        // rename file to 'track number - track name'
        match rename_audio_file(track_path, tags.track_number, &tags.track_name) {
            Ok(_) => {}
            Err(e) => {
                println!("Error renaming file {0} ({1})", track_path, e);
            }
        }
    }
}
