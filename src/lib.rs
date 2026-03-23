pub mod formats;

use anyhow::Result;
use serde_json::Value;

pub fn convert_bytes(input: &[u8], from: &str, to: &str) -> Result<Vec<u8>> {
    let value = formats::read_to_value_bytes(input, from)?;
    formats::write_from_value_bytes(&value, to)
}

/// convert file to file (format determined by file extension).
pub fn convert_file(
    input: impl AsRef<std::path::Path>,
    output: impl AsRef<std::path::Path>,
) -> Result<()> {
    let input = input.as_ref();
    let output = output.as_ref();
    let in_fmt = formats::detect_format(input)?;
    let out_fmt = formats::detect_format(output)?;
    let value = formats::read_to_value(input, in_fmt)?;
    formats::write_from_value(&value, output, out_fmt)
}

pub fn decode(input: &[u8], from: &str) -> Result<Value> {
    formats::read_to_value_bytes(input, from)
}

pub fn encode(value: &Value, to: &str) -> Result<Vec<u8>> {
    formats::write_from_value_bytes(value, to)
}

pub fn json_to_toml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "json", "toml")
}
pub fn json_to_yaml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "json", "yaml")
}
pub fn json_to_xml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "json", "xml")
}
pub fn json_to_env(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "json", "env")
}
pub fn json_to_kdl(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "json", "kdl")
}
pub fn json_to_cbor(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "json", "cbor")
}

pub fn toml_to_json(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "toml", "json")
}
pub fn toml_to_yaml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "toml", "yaml")
}
pub fn toml_to_xml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "toml", "xml")
}
pub fn toml_to_env(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "toml", "env")
}
pub fn toml_to_kdl(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "toml", "kdl")
}
pub fn toml_to_cbor(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "toml", "cbor")
}

pub fn yaml_to_json(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "yaml", "json")
}
pub fn yaml_to_toml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "yaml", "toml")
}
pub fn yaml_to_xml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "yaml", "xml")
}
pub fn yaml_to_env(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "yaml", "env")
}
pub fn yaml_to_kdl(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "yaml", "kdl")
}
pub fn yaml_to_cbor(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "yaml", "cbor")
}

pub fn xml_to_json(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "xml", "json")
}
pub fn xml_to_toml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "xml", "toml")
}
pub fn xml_to_yaml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "xml", "yaml")
}
pub fn xml_to_env(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "xml", "env")
}
pub fn xml_to_kdl(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "xml", "kdl")
}
pub fn xml_to_cbor(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "xml", "cbor")
}

pub fn cbor_to_json(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "cbor", "json")
}
pub fn cbor_to_toml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "cbor", "toml")
}
pub fn cbor_to_yaml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "cbor", "yaml")
}
pub fn cbor_to_xml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "cbor", "xml")
}
pub fn cbor_to_env(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "cbor", "env")
}
pub fn cbor_to_kdl(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "cbor", "kdl")
}

pub fn env_to_json(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "env", "json")
}
pub fn env_to_toml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "env", "toml")
}
pub fn env_to_yaml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "env", "yaml")
}
pub fn env_to_xml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "env", "xml")
}
pub fn env_to_kdl(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "env", "kdl")
}
pub fn env_to_cbor(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "env", "cbor")
}

pub fn kdl_to_json(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "kdl", "json")
}
pub fn kdl_to_toml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "kdl", "toml")
}
pub fn kdl_to_yaml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "kdl", "yaml")
}
pub fn kdl_to_xml(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "kdl", "xml")
}
pub fn kdl_to_env(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "kdl", "env")
}
pub fn kdl_to_cbor(input: &[u8]) -> Result<Vec<u8>> {
    convert_bytes(input, "kdl", "cbor")
}
