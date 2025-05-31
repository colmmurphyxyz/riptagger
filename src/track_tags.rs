// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::fmt;

use taglib::FileError;

#[derive(Debug)]
pub struct TrackTags {
    pub album_name: String,
    pub artist_name: String,
    pub year: u32,
    pub track_name: String,
    pub genre: String,
}

impl fmt::Display for TrackTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.album_name, self.artist_name, self.year, self.track_name, self.genre)
    }
}

pub fn assign_tags_to_track(tags: &TrackTags, track_path: &str) -> Result<(), FileError> {
    let file = match taglib::File::new(track_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };
    let mut t = file.tag()?;

    t.set_album(&tags.album_name);
    t.set_artist(&tags.artist_name);
    t.set_year(tags.year);
    // t.set_track(1);
    t.set_title(&tags.track_name);
    t.set_genre(&tags.genre);
    file.save();
    Ok(())
}