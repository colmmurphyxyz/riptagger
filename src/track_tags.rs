use std::fmt;

#[derive(Debug)]
pub struct TrackTags {
    pub album_name: String,
    pub artist_name: String,
    pub year: String,
    pub track_name: String,
    pub genre: String,
}

impl fmt::Display for TrackTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not implemented")
    }
}