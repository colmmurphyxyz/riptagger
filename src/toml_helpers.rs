// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
use toml::Table;

use crate::config::ConfigError;

pub fn get_i64_value(table: &Table, keys: &[&str]) -> Result<i64, ConfigError> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() || !val.unwrap().is_integer() {
            continue;
        }
        return Ok(val.unwrap().as_integer().unwrap());
    }
    Err(ConfigError::MissingKey(String::from(
        "No matching key found",
    )))
}

pub fn get_i64_array<'a>(table: &Table, keys: &[&str]) -> Result<Vec<i64>, ConfigError> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() || !val.unwrap().is_array() {
            continue;
        }
        let arr = val.unwrap().as_array().unwrap();
        let integers: Option<Vec<i64>> = arr.iter().map(|v| v.as_integer()).collect();
        if integers.is_none() {
            continue;
        }
        return Ok(integers.unwrap());
    }
    Err(ConfigError::MissingKey(String::from(
        "No matching key found",
    )))
}

pub fn get_single_or_array_i64<'a>(table: &Table, keys: &[&str]) -> Result<Vec<i64>, ConfigError> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() {
            continue;
        }
        let v = val.unwrap();
        if v.is_integer() {
            return Ok(vec![v.as_integer().unwrap()]);
        } else if v.is_array() {
            let arr = v.as_array().unwrap();
            let integers: Option<Vec<i64>> = arr.iter().map(|v| v.as_integer()).collect();
            if integers.is_none() {
                continue;
            }
            return Ok(integers.unwrap());
        }
    }
    Err(ConfigError::MissingKey(String::from(
        "No matching key found",
    )))
}

pub fn get_string_value(table: &Table, keys: &[&str]) -> Result<String, ConfigError> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() || !val.unwrap().is_str() {
            continue;
        }
        return Ok(val.unwrap().as_str().unwrap().to_string());
    }
    Err(ConfigError::MissingKey(String::from(
        "No matching key found",
    )))
}

pub fn get_string_array<'a>(table: &Table, keys: &[&str]) -> Result<Vec<String>, ConfigError> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() || !val.unwrap().is_array() {
            continue;
        }
        let arr = val.unwrap().as_array().unwrap();
        let strings: Option<Vec<String>> = arr
            .iter()
            .map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if strings.is_none() {
            continue;
        }
        return Ok(strings.unwrap());
    }
    Err(ConfigError::MissingKey(String::from(
        "No matching key found",
    )))
}

pub fn get_single_or_array_string(
    table: &Table,
    keys: &[&str],
) -> Result<Vec<String>, ConfigError> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() {
            continue;
        }
        let v = val.unwrap();
        if v.is_str() {
            return Ok(vec![v.as_str().unwrap().to_string()]);
        } else if v.is_array() {
            let arr = v.as_array().unwrap();
            let strings: Option<Vec<String>> = arr
                .iter()
                .map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if strings.is_none() {
                continue;
            }
            return Ok(strings.unwrap());
        }
    }
    Err(ConfigError::MissingKey(String::from(
        "No matching key found",
    )))
}
