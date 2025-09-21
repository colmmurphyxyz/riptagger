use std::error::Error;
use std::fmt::Display;

use toml::Table;

#[derive(Debug)]
pub enum TomlError<'a> {
    KeyNotFound(&'a str),
    TypeError(&'a str),
    Other(&'a str),
}

impl Display for TomlError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TomlError::KeyNotFound(s) => write!(f, "Key not found: {}", s),
            TomlError::TypeError(s) => write!(f, "Type error: {}", s),
            TomlError::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for TomlError<'_> {
    fn description(&self) -> &str {
        match *self {
            TomlError::KeyNotFound(s) => s,
            TomlError::TypeError(s) => s,
            TomlError::Other(s) => s,
        }
    }

}

pub fn get_i64_value(table: &Table, keys: Vec<String>) -> Result<i64, TomlError> {
    for key in keys {
        if !table.contains_key(key.as_str()) {
            continue;
        }
        let val = table.get(key.as_str());
        if val.is_none() || !val.unwrap().is_integer() {
            continue;
        }
        return Ok(val.unwrap().as_integer().unwrap());
    }
    Err(TomlError::KeyNotFound("No matching key found"))
}

pub fn get_i64_array<'a>(table: &Table, keys: &[&str]) -> Result<Vec<i64>, TomlError<'a>> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() || !val.unwrap().is_array(){
            continue;
        }
        let arr = val.unwrap().as_array().unwrap();
        let integers: Option<Vec<i64>> = arr.iter()
            .map(|v| v.as_integer())
            .collect();
        if integers.is_none() {
            continue;
        }
        return Ok(integers.unwrap());
    }
    Err(TomlError::KeyNotFound("No matching key found"))
}

pub fn get_single_or_array_i64<'a>(table: &Table, keys: &[&str]) -> Result<Vec<i64>, TomlError<'a>> {
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
            return Ok(vec![v.as_integer().unwrap()])
        } else if v.is_array() {
            let arr = v.as_array().unwrap();
            let integers: Option<Vec<i64>> = arr.iter()
                .map(|v| v.as_integer())
                .collect();
            if integers.is_none() {
                continue;
            }
            return Ok(integers.unwrap());
        }
    }
    Err(TomlError::KeyNotFound("No matching key found"))
}

pub fn get_string_value(table: &Table, keys: Vec<String>) -> Result<String, TomlError> {
    for key in keys {
        if !table.contains_key(key.as_str()) {
            continue;
        }
        let val = table.get(key.as_str());
        if val.is_none() || !val.unwrap().is_str() {
            continue;
        }
        return Ok(val.unwrap().as_str().unwrap().to_string());
    }
    Err(TomlError::KeyNotFound("No matching key found"))
}

pub fn get_string_array<'a>(table: &Table, keys: &[&str]) -> Result<Vec<String>, TomlError<'a>> {
    for key in keys {
        if !table.contains_key(*key) {
            continue;
        }
        let val = table.get(*key);
        if val.is_none() || !val.unwrap().is_array(){
            continue;
        }
        let arr = val.unwrap().as_array().unwrap();
        let strings: Option<Vec<String>> = arr.iter()
            .map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if strings.is_none() {
            continue;
        }
        return Ok(strings.unwrap());
    }
    Err(TomlError::KeyNotFound("No matching key found"))
}

pub fn get_single_or_array_string(table: &Table, keys: Vec<String>) -> Result<Vec<String>, TomlError> {
    for key in keys {
        if !table.contains_key(key.as_str()) {
            continue;
        }
        let val = table.get(key.as_str());
        if val.is_none() {
            continue;
        }
        let v = val.unwrap();
        if v.is_str() {
            return Ok(vec![v.as_str().unwrap().to_string()])
        } else if v.is_array() {
            let arr = v.as_array().unwrap();
            let strings: Option<Vec<String>> = arr.iter()
                .map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if strings.is_none() {
                continue;
            }
            return Ok(strings.unwrap());
        }
    }
    Err(TomlError::KeyNotFound("No matching key found"))
}