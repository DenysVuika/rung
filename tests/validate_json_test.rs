use anyhow::Result;
use rung::{read_json, validate_json};
use std::io::Write;
use tempfile::NamedTempFile;

mod common;

#[test]
fn reads_json_from_file() -> Result<()> {
    common::setup();

    let data = r#"
    {
        "name": "Denys"
    }
    "#;

    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", data)?;

    let value = read_json(file.path())?;

    assert_eq!("Denys", value["name"]);

    Ok(())
}

#[test]
fn passes_validation() -> Result<()> {
    common::setup();

    let json_text = r#"
    {
        "name": "Denys"
    }
    "#;

    let mut json_file = NamedTempFile::new()?;
    writeln!(json_file, "{}", json_text)?;

    let schema_text = r#"
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "http://example.com/product.schema.json",
        "type": "object",
        "properties": {
            "name": {
                "type": "string"
            }
        },
        "required": ["name"]
    }
    "#;

    let mut schema_file = NamedTempFile::new()?;
    writeln!(schema_file, "{}", schema_text)?;

    assert_eq!(true, validate_json(json_file.path(), schema_file.path())?);

    Ok(())
}

#[test]
fn fails_validation() -> Result<()> {
    common::setup();

    let json_text = r#"
    {
        "name": true
    }
    "#;

    let mut json_file = NamedTempFile::new()?;
    writeln!(json_file, "{}", json_text)?;

    let schema_text = r#"
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "http://example.com/product.schema.json",
        "type": "object",
        "properties": {
            "name": {
                "type": "string"
            }
        },
        "required": ["name"]
    }
    "#;

    let mut schema_file = NamedTempFile::new()?;
    writeln!(schema_file, "{}", schema_text)?;

    assert_eq!(false, validate_json(json_file.path(), schema_file.path())?);

    Ok(())
}
