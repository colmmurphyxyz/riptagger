use std::fmt;

#[derive(Debug)]
pub struct TrackTags {
    pub album_name: &'static str,
    pub artist_name: &'static str,
    pub year: &'static str,
    pub track_name: &'static str,
    pub genre: &'static str,
}

impl fmt::Display for TrackTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not implemented")
    }
}