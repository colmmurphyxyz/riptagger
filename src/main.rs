// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

extern crate taglib;

use std::env;

fn main() {
    let cwd = env::current_dir().unwrap();
    println!("Current dir is {}.", cwd.display());

    let file_name = "test.flac";

    let file = match taglib::File::new_type(file_name, taglib::FileType::FLAC) {
        Ok(f) => f,
        Err(e) => {
            println!("Invalid file {} (error: {:?})", file_name, e);
            panic!("boo!")
        }
    };

    match file.tag() {
        Ok(t) => {
            println!("-- TAG --");
            println!("title   - {}", t.title().unwrap_or_default());
            println!("artist  - {}", t.artist().unwrap_or_default());
            println!("album   - {}", t.album().unwrap_or_default());
            println!("year    - {}", t.year().unwrap_or_default());
            println!("comment - {}", t.comment().unwrap_or_default());
            println!("track   - {}", t.track().unwrap_or_default());
            println!("genre   - {}", t.genre().unwrap_or_default());
        },
        Err(e) => {
            println!("No available tags for {} (error: {:?})", file_name, e);
            panic!("!!!");
        }
    }

    println!("Hello, world!");
}
