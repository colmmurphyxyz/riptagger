// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::fmt;

use id3::{Tag, TagLike, Error, Version};

#[derive(Debug)]
pub struct TrackTags {
    pub album_name: String,
    pub artist_name: String,
    pub year: Option<u32>,
    pub track_name: String,
    pub genre: Option<String>,
    pub track_number: u32,
    pub track_total: u32,
    pub disc_number: Option<u32>,
    pub disc_total: Option<u32>,
}

impl fmt::Display for TrackTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {:?}, {}, {:?})", self.album_name, self.artist_name, self.year, self.track_name, self.genre)
    }
}

pub fn assign_tags_to_track(tags: &TrackTags, track_path: &str) -> Result<(), Error> {
    let mut file = match Tag::read_from_path(track_path) {
        Ok(f) => f,
        Err(e) => { return Err(e); }
    };

    file.set_album(&tags.album_name);
    file.set_artist(&tags.artist_name);
    file.set_album_artist(&tags.artist_name);
    if let Some(year) = tags.year {
        file.set_year(year as i32)
    }
    file.set_title(&tags.track_name);
    if let Some(genre) = &tags.genre {
        file.set_genre(genre);
    }
    file.set_track(tags.track_number);
    file.set_total_tracks(tags.track_total);
    if let Some(disc_num) = tags.disc_number {
        file.set_disc(disc_num);
    }
    if let Some(disc_total) = tags.disc_total {
        file.set_total_discs(disc_total);
    }
    file.write_to_path(track_path, Version::Id3v24)?;
    Ok(())
}