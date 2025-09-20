// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
use std::error::Error;
use toml::Table;

use crate::config::ConfigError;
use crate::album_tags::{to_track_tags, AlbumTags};


#[test]
fn test_string_value() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
    album = "Plantgazer"
    artist = "Show Me a Dinosaur"
    genre = "\"Black Metal\" 'blackgaze'"
    tracks = []
    disc_total = 1
    tracks_per_disc = [0]
    "#)?;
    let tags = AlbumTags::from_toml(table)?;
    assert_eq!(tags.album_name, Some("Plantgazer".to_string()));
    assert_eq!(tags.artist_name, Some("Show Me a Dinosaur".to_string()));
    assert_eq!(tags.genre, Some("\"Black Metal\" 'blackgaze\'".to_string()));

    Ok(())
}

#[test]
fn test_string_array_value() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        tracks = ["Track 1", "\"quotes\"", "'single quotes'"]
        disc_total = 1
        tracks_per_disc = [3]
    "#)?;
    let tags: AlbumTags = AlbumTags::from_toml(table)?;
    assert_eq!(tags.tracks, vec![
        "Track 1".to_string(),
        "\"quotes\"".to_string(),
        "'single quotes'".to_string()
    ]);

    Ok(())
}

#[test]
fn test_disc_number() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        tracks = ["Track 1", "Track 2", "Track 3", "Track 4", "Track 5", "Track 6", "Track 7", "Track 8", "Track 9"]
        disc_total = 3
        tracks_per_disc = [3, 4, 2]
    "#)?;
    let tags: AlbumTags = AlbumTags::from_toml(table)?;
    assert_eq!(tags.disc_total, Some(3));
    assert_eq!(tags.tracks_per_disc, Some(vec![3, 4, 2]));

    let track_tags = to_track_tags(tags);
    assert_eq!(track_tags[0].disc_number, Some(1));
    assert_eq!(track_tags[1].disc_number, Some(1));
    assert_eq!(track_tags[2].disc_number, Some(1));
    assert_eq!(track_tags[3].disc_number, Some(2));
    assert_eq!(track_tags[4].disc_number, Some(2));
    assert_eq!(track_tags[5].disc_number, Some(2));
    assert_eq!(track_tags[6].disc_number, Some(2));
    assert_eq!(track_tags[7].disc_number, Some(3));
    assert_eq!(track_tags[8].disc_number, Some(3));

    Ok(())
}

#[test]
fn test_missing_tracks_key() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        album = "Plantgazer"
        artist = "Show Me a Dinosaur"
        disc_total = 1
        tracks_per_disc = [0]
    "#)?;
    let result = AlbumTags::from_toml(table);
    assert!(result.is_err());
    let err = result.err().unwrap();

    if let ConfigError::MissingKey(key) = err {
        assert_eq!(key, "tracks");
    } else {
        panic!("Expected error 'MissingKey'. Received a different ConfigError variant.");
    }

    Ok(())
}

#[test]
fn test_to_track_tags() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        album = "Plantgazer"
        artist = "Show Me a Dinosaur"
        year = 2020
        genre = "Blackgaze"
        tracks = ["Track 1", "Track 2", "Track 3"]
        disc_total = 2
        tracks_per_disc = [2, 1]
    "#)?;
    let album_tags: AlbumTags = AlbumTags::from_toml(table)?;
    let track_tags = to_track_tags(album_tags);

    assert_eq!(track_tags.len(), 3);
    assert_eq!(track_tags[0].album_name, Some("Plantgazer".to_string()));
    assert_eq!(track_tags[0].artist_name, Some("Show Me a Dinosaur".to_string()));
    assert_eq!(track_tags[0].year, Some(2020));
    assert_eq!(track_tags[0].genre, Some("Blackgaze".to_string()));
    assert_eq!(track_tags[0].track_name, "Track 1".to_string());
    assert_eq!(track_tags[0].track_number, Some(1));
    assert_eq!(track_tags[0].disc_number, Some(1));
    assert_eq!(track_tags[0].disc_total, Some(2));

    assert_eq!(track_tags[1].track_name, "Track 2".to_string());
    assert_eq!(track_tags[1].track_number, Some(2));
    assert_eq!(track_tags[1].disc_number, Some(1));

    assert_eq!(track_tags[2].track_name, "Track 3".to_string());
    assert_eq!(track_tags[2].track_number, Some(3));
    assert_eq!(track_tags[2].disc_number, Some(2));

    Ok(())
}

#[test]
fn test_to_track_tags_no_disc_info() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        album = "Solo Album"
        artist = "Lone Artist"
        year = 2021
        tracks = ["first", "second"]

    "#)?;

    let album_tags: AlbumTags = AlbumTags::from_toml(table)?;
    let track_tags = to_track_tags(album_tags);
    assert_eq!(track_tags.len(), 2);
    assert_eq!(track_tags[0].disc_number, None);
    assert_eq!(track_tags[0].disc_total, None);
    assert_eq!(track_tags[1].disc_number, None);
    assert_eq!(track_tags[1].disc_total, None);

    Ok(())
}
