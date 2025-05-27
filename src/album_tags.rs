use crate::TrackTags;

#[derive(Debug)]
pub struct AlbumTags {
    album_name: &'static str,
    artist_name: &'static str,
    year: &'static str,
    genre: &'static str,
    tracks: Vec<&'static str>,
}

pub fn to_track_tags(album: AlbumTags) -> Vec<TrackTags> {
    let mut tags = Vec::new();
    let mut index = 0;
    while index < album.tracks.len() {
        tags.push(TrackTags {
            album_name: album.album_name,
            artist_name: album.artist_name,
            year: album.year,
            track_name: album.tracks[index],
            genre: album.genre,
        });

        index += 1;
    }
    tags
}