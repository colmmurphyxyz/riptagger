use std::env::temp_dir;
use std::io;
use std::fs::{copy, read_dir, remove_dir_all, create_dir_all};
use std::path::Path;

use crate::fs_utils;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    create_dir_all(&dst)?;
    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[test]
fn test_get_audio_files_in_directory() {
    let path = "test_data/empty_audio_files";
    let files = fs_utils::get_audio_files_in_directory(path).unwrap();
    assert_eq!(files.len(), 4);
    let file_names: Vec<String> = files.iter().map(|f| f.file_name().to_string_lossy().to_string()).collect();
    assert!(file_names.contains(&"track.mp3".to_string()));
    assert!(file_names.contains(&"track.ogg".to_string()));
    assert!(file_names.contains(&"track.flac".to_string()));
    assert!(file_names.contains(&"track.asf".to_string()));
    assert!(!file_names.contains(&"text.txt".to_string()));
}

#[test]
fn test_normalize_file_name() {
    assert!(fs_utils::normalize("normal file name.mp3") == "normal file name.mp3");
    assert!(fs_utils::normalize("file'name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file\"name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file/name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file\\name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file>name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file<name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file:name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file|name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file*name.mp3") == "filename.mp3");
    assert!(fs_utils::normalize("file?name.mp3") == "filename.mp3");
}

#[test]
fn test_rename_audio_file() {
    let original_path = Path::new("test_data/test_rename_audio_file");
    let path = temp_dir().join("riptagger_test");

    copy_dir_all(&original_path, &path).unwrap();
    
    fs_utils::rename_audio_file(&path.join("first.mp3").to_string_lossy(), 1, "First").unwrap();
    fs_utils::rename_audio_file(&path.join("second.mp3").to_string_lossy(), 2, "Second").unwrap();
    fs_utils::rename_audio_file(&path.join("third.mp3").to_string_lossy(), 3, "Third").unwrap();
    
    let files = fs_utils::get_audio_files_in_directory(&path.to_string_lossy()).unwrap();
    assert_eq!(files.len(), 3);
    let file_names: Vec<String> = files.iter().map(|f| f.file_name().to_string_lossy().to_string()).collect();
    assert!(file_names.contains(&"01 - First.mp3".to_string()));
    assert!(file_names.contains(&"02 - Second.mp3".to_string()));
    assert!(file_names.contains(&"03 - Third.mp3".to_string()));

    // cleanup
    remove_dir_all(path).unwrap();
}