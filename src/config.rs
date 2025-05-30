// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::fs;
use std::fmt;

use toml::Table;

use crate::album_tags::AlbumTags;

#[derive(Debug)]
pub enum ConfigError {
    MissingKey(String),
    TypeError(String),
}

impl From<&str> for ConfigError {
    fn from(s: &str) -> Self {
        ConfigError::TypeError(s.to_string())
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConfigError::*;
        match self {
            MissingKey(k) => write!(f, "Missing key: {}", k),
            TypeError(t) => write!(f, "Type error: {}", t),
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn load_config_from_file(config_path: &str) -> Result<AlbumTags, Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string(config_path)?;
    let table = config_file.parse::<Table>()?;
    let tags = AlbumTags::from_toml(table)?;
    Ok(tags)
}