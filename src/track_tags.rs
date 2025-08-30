// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::fmt;
use std::fs::read;

use metaflac::{Tag, Error};
use metaflac::block::PictureType::CoverFront;

#[derive(Debug)]
pub struct TrackTags {
    pub album_name: Option<String>,
    pub artist_name: Option<String>,
    pub year: Option<u32>,
    pub track_name: String,
    pub genre: Option<String>,
    pub picture_path: Option<String>,
    pub track_number: Option<u32>,
    pub track_total: Option<u32>,
    pub disc_number: Option<u32>,
    pub disc_total: Option<u32>,
}

impl fmt::Display for TrackTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "album_name: {:?}\n", self.album_name)
        .and_then(|_|
            write!(f, "artist_name: {:?}\n", self.artist_name)
        )
        .and_then(|_|
            write!(f, "year: {:?}\n", self.year.and_then(|x| Some(x.to_string())))
        )
        .and_then(|_|
            write!(f, "track_name: {:?}\n", self.track_name)
        )
        .and_then(|_|
            write!(f, "genre: {:?}\n", self.genre)
        )
        .and_then(|_|
            write!(f, "track_number: {:?}\n", self.track_number)
        )
        .and_then(|_|
            write!(f, "track_total: {:?}\n", self.track_total)
        )
        .and_then(|_|
            write!(f, "disc_number: {:?}\n", self.disc_number.and_then(|x| Some(x.to_string())))
        )
        .and_then(|_|
            write!(f, "disc_total: {:?}", self.disc_total.and_then(|x| Some(x.to_string())))
        )
    }
}

pub fn assign_tags_to_track(tags: &TrackTags, track_path: &str) -> Result<(), Error> {
    let mut file = match Tag::read_from_path(track_path) {
        Ok(f) => f,
        Err(e) => { return Err(e) }
    };

    if let Some(album_name) = &tags.album_name {
        file.set_vorbis("ALBUM", vec![album_name]);
    }

    if let Some(artist_name) = &tags.artist_name {
        file.set_vorbis("ARTIST", vec![artist_name]);
    }

    file.set_vorbis("TITLE", vec![&tags.track_name]);

    if let Some(track_number) = &tags.track_number {
        file.set_vorbis("TRACKNUMBER", vec![track_number.to_string()]);
    }

    if let Some(track_total) = &tags.track_total {
        file.set_vorbis("TRACKTOTAL", vec![track_total.to_string()]);
    }

    if let Some(year) = tags.year {
        file.set_vorbis("DATE", vec![year.to_string()]);
    }

    if let Some(genre) = &tags.genre {
        file.set_vorbis("GENRE", vec![genre]);
    }

    if let Some(disc_num) = &tags.disc_number {
        file.set_vorbis("DISCNUMBER", vec![disc_num.to_string()]);
    }

    if let Some(disc_total) = &tags.disc_total {
        file.set_vorbis("TOTALDISCS", vec![disc_total.to_string()])
    }

    if let Some(cover_path) = &tags.picture_path {
        let pic = read(cover_path).unwrap();
        file.add_picture("image/jpeg", CoverFront, pic)
    }

    file.save()
}
