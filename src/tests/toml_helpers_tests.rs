// Copyright: (c) 2025, Colm Murphy
// GNU General Public License v3.0 (see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt)
use std::error::Error;
use toml::Table;

use crate::{
    config::ConfigError,
    toml_helpers::{get_i64_array, get_i64_value, get_single_or_array_i64, get_string_array, get_string_value, get_single_or_array_string},
};

#[test]
fn test_get_i64_value() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(
        r#"
        "key1" = 42
        "key2" = "not an integer"
        "key3" = 100
    "#,
    )?;

    let value1 = get_i64_value(&table, &["key1"]);
    assert!(value1.is_ok());
    assert_eq!(value1.unwrap(), 42);

    let value2 = get_i64_value(&table, &["key2"]);
    assert!(value2.is_err());
    match value2 {
        Err(ConfigError::MissingKey(_)) => (),
        _ => panic!("Expected MissingKey error"),
    }

    let value3 = get_i64_value(&table, &["notakey", "key3"]);
    assert!(value3.is_ok());
    assert_eq!(value3.unwrap(), 100);
    Ok(())
}

#[test]
fn test_get_i64_array() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(
        r#"
        "array1" = [1, 2, 3]
        "stringarray" = ["a", "b", "c"]
        "notanarray" = "foo"
    "#,
    )?;

    let array1 = get_i64_array(&table, &["notakey", "array1"]);
    assert!(array1.is_ok());
    assert_eq!(array1.unwrap(), vec![1, 2, 3]);

    let array2 = get_i64_array(&table, &["stringarray"]);
    assert!(array2.is_err());
    match array2 {
        Err(ConfigError::MissingKey(_)) => (),
        _ => panic!("Expected MissingKey error"),
    }

    let array3 = get_i64_array(&table, &["notanarray"]);
    assert!(array3.is_err());
    assert_eq!(
        array3.unwrap_err().to_string(),
        "Missing key: No matching key found"
    );
    Ok(())
}

#[test]
fn test_get_single_or_array_i64() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        "single" = 7
        "array" = [10, 20, 30]
        "invalidarray" = ["x", "y", "z"]
    "#,)?;

    let single = get_single_or_array_i64(&table, &["someotherkey", "single"]);
    assert!(single.is_ok());
    assert_eq!(single.unwrap(), vec![7]);

    let array = get_single_or_array_i64(&table, &["array"]);
    assert!(array.is_ok());
    assert_eq!(array.unwrap(), vec![10, 20, 30]);

    let invalid = get_single_or_array_i64(&table, &["invalidarray"]);
    assert!(invalid.is_err());
    match invalid {
        Err(ConfigError::MissingKey(_)) => (),
        _ => panic!("Expected MissingKey error"),
    }

    Ok(())
}

#[test]
fn test_get_string_value() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(
        r#"
        "key1" = "value1"
        "key2" = 123
        "key3" = "value3"
    "#,
    )?;
    let value1 = get_string_value(&table, &["key1"]);
    assert!(value1.is_ok());
    assert_eq!(value1.unwrap(), "value1".to_string());
    let value2 = get_string_value(&table, &["key2"]);
    assert!(value2.is_err());
    match value2 {
        Err(ConfigError::MissingKey(_)) => (),
        _ => panic!("Expected MissingKey error"),
    }
    let value3 = get_string_value(&table, &["notakey", "key3"]);
    assert!(value3.is_ok());
    assert_eq!(value3.unwrap(), "value3".to_string());
    Ok(())
}

#[test]
fn test_get_string_array() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        "array1" = ["a", "b", "c"]
        "intarray" = [1, 2, 3]
        "notanarray" = 42
    "#,
    )?;

    let array1 = get_string_array(&table, &["notakey", "array1"]);
    assert!(array1.is_ok());
    assert_eq!(
        array1.unwrap(),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );

    let array2 = get_string_array(&table, &["intarray"]);
    assert!(array2.is_err());
    match array2 {
        Err(ConfigError::MissingKey(_)) => (),
        _ => panic!("Expected MissingKey error"),
    }

    let array3 = get_string_array(&table, &["notanarray"]);
    assert!(array3.is_err());
    assert_eq!(
        array3.unwrap_err().to_string(),
        "Missing key: No matching key found"
    );
    Ok(())
}

#[test]
fn test_get_single_or_array_string() -> Result<(), Box<dyn Error>> {
    let table: Table = toml::from_str(r#"
        "single" = "hello"
        "array" = ["foo", "bar", "baz"]
        "invalidarray" = [1, 2, 3]
    "#,)?;

    let single = get_single_or_array_string(&table, &["someotherkey", "single"]);
    assert!(single.is_ok());
    assert_eq!(single.unwrap(), vec!["hello".to_string()]);

    let array = get_single_or_array_string(&table, &["array"]);
    assert!(array.is_ok());
    assert_eq!(array.unwrap(), vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]);
    
    let invalid = get_single_or_array_string(&table, &["invalidarray"]);
    assert!(invalid.is_err());
    match invalid {
        Err(ConfigError::MissingKey(_)) => (),
        _ => panic!("Expected MissingKey error"),
    }

    Ok(())
}
