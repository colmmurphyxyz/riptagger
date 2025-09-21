// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

pub mod config;
pub mod toml_helpers;
pub mod track_tags;
pub mod album_tags;
pub mod fs_utils;
pub mod tests;
use clap::Parser;

use config::load_config_from_file;
use track_tags::{assign_tags_to_track, TrackTags};
use album_tags::{to_track_tags, AlbumTags};
use fs_utils::{get_audio_files_in_directory, rename_audio_file};

static ABOUT: &str = "Copyright (C) 2025 Colm Murphy
GPLv3: GNU GPL version 3 or later <https://www.gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.";

#[derive(Parser, Debug)]
#[command(version, about=ABOUT, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config_path: String,

    #[arg(short, long)]
    album_path: String,

    #[arg(short, long, default_value_t = false)]
    no_rename_files: bool,
}

fn main() {
    let args = Cli::parse();

    let album_tags: AlbumTags = match load_config_from_file(&args.config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            panic!("Could not read config file '{0}'. Error: '{1}'", &args.config_path, e)
        }
    };

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

    if track_tags.len() != tracks.len() {
        panic!("Error: Configuration defines {} tracks, but {} tracks found in directory {}\n({:?})", track_tags.len(), tracks.len(), &args.album_path, tracks);
    }

    for i in 0..tracks.len() {
        let track_path = tracks[i].as_str();
        let tags = &track_tags[i];
        match assign_tags_to_track(tags, track_path) {
            Ok(_) => {
                println!("Tagged '{}'", track_path);
            },
            Err(e) => {
                println!("Error tagging '{}': {:#?}", track_path, e);
                continue
            }
        }
        if args.no_rename_files {
            continue;
        }
        // rename file to 'track number - track name'
        let track_num = tags.track_number.unwrap_or(0);
        match rename_audio_file(track_path, track_num, &tags.track_name) {
            Ok(new_name) => {
                println!("Renamed '{}' to '{}'", track_path, new_name)
            }
            Err(e) => {
                println!("Error renaming file '{0}' ({1})", track_path, e);
            }
        }
    }
}
