// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

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
        write!(f, "({}, {}, {}, {}, {})", self.album_name, self.artist_name, self.year, self.track_name, self.genre)
    }
}