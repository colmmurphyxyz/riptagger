// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

extern crate taglib;

use std::env;
use std::fs;
use toml::Table;

pub mod track_tags;
pub mod album_tags;

use track_tags::TrackTags;
use album_tags::AlbumTags;

fn main() {
    let cwd = env::current_dir().unwrap();
    println!("Current dir is {}.", cwd.display());

    let config_path = "scratch/config.toml";
    let config = fs::read_to_string(config_path).unwrap().parse::<Table>();
    match config {
        Ok(table) => {
            let my_config = AlbumTags::from_toml(table).unwrap();
            println!("{}", my_config)
        }
        Err(e) => println!("Error reading config file {:?}", e)
    }
}
