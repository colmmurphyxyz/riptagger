// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
pub mod album_tags_tests;

#[cfg(test)]
pub mod config_tests;

#[cfg(test)]
pub mod fs_utils_tests;

#[cfg(test)]
pub mod toml_helpers_tests;

