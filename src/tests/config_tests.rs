use crate::config::{ConfigError, load_config_from_file};

#[test]
fn test_valid_config() {
    let config_path = "test_data/configs/valid_config.toml";
    let tags = load_config_from_file(config_path).unwrap();

    assert_eq!(tags.album_name, Some("Romance".to_string()));
    assert_eq!(tags.artist_name, Some("Fontaines D.C".to_string()));
    assert_eq!(tags.year, Some(2024));
    assert_eq!(tags.genre, Some("Indie".to_string()));
    let tracks = tags.tracks;
    assert_eq!(tracks.len(), 11);
    assert_eq!(
        tracks,
        vec![
            "Romance",
            "Starburster",
            "Here's The Thing",
            "Desire",
            "In The Modern World",
            "Bug",
            "Motorcycle Boy",
            "Sundowner",
            "Horseness Is The Whatness",
            "Death Kink",
            "Favourite"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    );
    assert_eq!(tags.disc_total, Some(1));
    assert_eq!(tags.tracks_per_disc, Some(vec![11]));
}

#[test]
fn test_missing_config_file() {
    let config_path = "non_existent.toml";
    let result = load_config_from_file(config_path);
    assert!(result.is_err());
}

#[test]
fn test_malformed_config_file() {
    let config_path = "test_data/configs/missing_tracks_key.toml";
    let result = load_config_from_file(config_path);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let err_str = format!("{}", err);
    assert_eq!(err_str, "Missing key: tracks");
}

#[test]
fn test_config_error_display() {
    let err1 = ConfigError::MissingKey("album".to_string());
    assert_eq!(format!("{}", err1), "Missing key: album");

    let err2 = ConfigError::TypeError("year should be an integer".to_string());
    assert_eq!(format!("{}", err2), "Type error: year should be an integer");
}
