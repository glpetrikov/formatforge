pub mod cbor;
pub mod env;
pub mod ini;
pub mod json;
pub mod json5;
pub mod jsonc;
pub mod kdl;
pub mod msgpack;
pub mod toml;
pub mod xml;
pub mod yaml;

use anyhow::{bail, Result};
use serde_json::Value;
use std::path::Path;

pub fn detect_format(path: &Path) -> Result<&'static str> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Ok("json"),
        Some("toml") => Ok("toml"),
        Some("yaml") | Some("yml") => Ok("yaml"),
        Some("xml") => Ok("xml"),
        Some("csv") => Ok("csv"),
        Some("env") => Ok("env"),
        Some("kdl") => Ok("kdl"),
        Some("cbor") => Ok("cbor"),
        Some("ini") => Ok("ini"),
        Some("json5") => Ok("json5"),
        Some("jsonc") => Ok("jsonc"),
        Some("msgpack") | Some("mp") => Ok("msgpack"),
        Some(ext) => bail!("Unsupported format: .{}", ext),
        None => bail!("File has no extension: {}", path.display()),
    }
}

pub fn read_to_value_bytes(bytes: &[u8], format: &str) -> Result<Value> {
    match format {
        "json" => json::from_bytes(bytes),
        "toml" => toml::from_bytes(bytes),
        "yaml" => yaml::from_bytes(bytes),
        "xml" => xml::from_bytes(bytes),
        "env" => env::from_bytes(bytes),
        "kdl" => kdl::from_bytes(bytes),
        "cbor" => cbor::from_bytes(bytes),
        "ini" => ini::from_bytes(bytes),
        "json5" => json5::from_bytes(bytes),
        "jsonc" => jsonc::from_bytes(bytes),
        "msgpack" => msgpack::from_bytes(bytes),
        _ => bail!("Unknown format: {}", format),
    }
}

pub fn write_from_value_bytes(value: &Value, format: &str) -> Result<Vec<u8>> {
    match format {
        "json" => json::to_bytes(value),
        "toml" => toml::to_bytes(value),
        "yaml" => yaml::to_bytes(value),
        "xml" => xml::to_bytes(value),
        "env" => env::to_bytes(value),
        "kdl" => kdl::to_bytes(value),
        "cbor" => cbor::to_bytes(value),
        "ini" => ini::to_bytes(value),
        "json5" => json5::to_bytes(value),
        "jsonc" => jsonc::to_bytes(value),
        "msgpack" => msgpack::to_bytes(value),
        _ => bail!("Unknown format: {}", format),
    }
}

pub fn read_to_value(path: &Path, format: &str) -> Result<Value> {
    let bytes = std::fs::read(path)?;
    read_to_value_bytes(&bytes, format)
}

pub fn write_from_value(value: &Value, path: &Path, format: &str) -> Result<()> {
    let bytes = write_from_value_bytes(value, format)?;
    std::fs::write(path, bytes)?;
    Ok(())
}
