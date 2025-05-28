// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

fn get_file_extension(file_name: &OsStr) -> Option<&str> {
    Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
}

fn is_audio_file(f: &DirEntry) -> bool {
    match get_file_extension(&f.file_name()) {
        Some(ext) => {
            match ext {
                "mp3" => true,
                "ogg" => true,
                "flac" => true,
                "asf" => true,
                _ => false,
            }
        },
        None => false
    }
}

pub fn get_audio_files_in_directory(dir_path: &str) -> Result<Vec<DirEntry>, std::io::Error> {
    match fs::read_dir(dir_path) {
        Ok(res) => {
            Ok(res
                .filter_map(Result::ok)
                .filter(is_audio_file)
                .collect())
        }
        Err(e) => {
            return Err(e);
        }
    }
}