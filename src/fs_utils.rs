// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::ffi::OsStr;
use std::fs::{DirEntry, read_dir, rename};
use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use once_cell::sync::Lazy;

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
    match read_dir(dir_path) {
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

static DISALLOWED_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    [
        '\'', '"', '/', '\\', '>', '<', ':', '|', '*', '?'
    ].iter().cloned().collect()
});

pub fn normalize(s: &str) -> String {
    s.to_string()
        .chars()
        .filter(|c| !DISALLOWED_CHARS.contains(c)).collect::<String>()
}

pub fn rename_audio_file(file_path: &str, track_number: u32, track_name: &str) -> std::io::Result<String> {
    let parent = Path::new(file_path).parent().unwrap_or(Path::new("."));
    // original file extension, with leading .
    let extension = Path::new(file_path)
        .extension()
        .and_then(|osstr| osstr.to_str())
        .and_then(|s| format!(".{}", s).into());
    // zero-pad file name to at least 2 digits
    // there may be some *long* album out there with >99 tracks.
    // I only plan on using this for tagging CD rips, unlikely this will ever be a problem.
    let new_name = format!(
        "{:#02} - {}{}",
        track_number,
        normalize(track_name),
        extension.unwrap_or(String::from(".unknown")));
    let new_path = parent.join(&new_name);

    match rename(file_path, new_path) {
        Ok(_) => Ok(new_name),
        Err(e) => Err(e),
    }
}

pub fn get_current_directory() -> io::Result<PathBuf> {
    std::env::current_dir()
}
