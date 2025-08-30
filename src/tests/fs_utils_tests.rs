use crate::fs_utils;

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