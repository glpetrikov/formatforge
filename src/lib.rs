//! # formatforge
//!
//! Convert between data formats: JSON, TOML, YAML, XML, ENV, KDL, CBOR, INI, JSON5, JSONC, MessagePack.
//!
//! ## Quick start
//!
//! ```rust
//! use formatforge::convert_bytes;
//!
//! let json = br#"{"name": "test"}"#;
//! let toml = convert_bytes(json, "json", "toml").unwrap();
//! ```

pub mod formats;
use anyhow::Result;
use serde_json::Value;

/// Convert bytes from one format to another.
///
/// # Example
/// ```rust
/// let json = br#"{"name": "test"}"#;
/// let toml = formatforge::convert_bytes(json, "json", "toml").unwrap();
/// ```
pub fn convert_bytes(input: &[u8], from: &str, to: &str) -> Result<Vec<u8>> {
    let value = formats::read_to_value_bytes(input, from)?;
    formats::write_from_value_bytes(&value, to)
}

/// Convert a file to another format, detecting formats from file extensions.
///
/// # Example
/// ```no_run
/// formatforge::convert_file("config.json", "config.toml").unwrap();
/// ```
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

/// Decode bytes into a `serde_json::Value`.
///
/// # Example
/// ```rust
/// let json = br#"{"hello": "world"}"#;
/// let value = formatforge::decode(json, "json").unwrap();
/// ```
pub fn decode(input: &[u8], from: &str) -> Result<Value> {
    formats::read_to_value_bytes(input, from)
}

/// Encode a `serde_json::Value` into bytes of the given format.
///
/// # Example
/// ```rust
/// let value = serde_json::json!({"hello": "world"});
/// let yaml = formatforge::encode(&value, "yaml").unwrap();
/// ```
pub fn encode(value: &Value, to: &str) -> Result<Vec<u8>> {
    formats::write_from_value_bytes(value, to)
}

// Generate all format conversion functions like json_to_toml, yaml_to_json, etc.
macro_rules! make_converters {
    ($($from:ident),*) => {
        $(
            make_converters!(@to $from, json);
            make_converters!(@to $from, toml);
            make_converters!(@to $from, yaml);
            make_converters!(@to $from, xml);
            make_converters!(@to $from, env);
            make_converters!(@to $from, kdl);
            make_converters!(@to $from, cbor);
            make_converters!(@to $from, ini);
            make_converters!(@to $from, json5);
            make_converters!(@to $from, jsonc);
            make_converters!(@to $from, msgpack);
        )*
    };
    (@to $from:ident, $to:ident) => {
        paste::paste! {
            pub fn [<$from _to_ $to>](input: &[u8]) -> Result<Vec<u8>> {
                convert_bytes(input, stringify!($from), stringify!($to))
            }
        }
    };
}

make_converters!(
    json, toml, yaml, xml, env, kdl, cbor, ini, json5, jsonc, msgpack
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_toml() {
        let json = br#"{"name": "test", "version": "1.0"}"#;
        let toml = json_to_toml(json).unwrap();
        let toml_str = String::from_utf8(toml).unwrap();
        assert!(toml_str.contains("name"));
        assert!(toml_str.contains("test"));
    }

    #[test]
    fn test_json_to_msgpack_and_back() {
        let json = br#"{"hello": "world"}"#;
        let msgpack = json_to_msgpack(json).unwrap();
        let back = msgpack_to_json(&msgpack).unwrap();
        let s = String::from_utf8(back).unwrap();
        assert!(s.contains("hello"));
        assert!(s.contains("world"));
    }

    #[test]
    fn test_ini_to_json() {
        let ini = b"[section]\nkey=value\n";
        let json = ini_to_json(ini).unwrap();
        let s = String::from_utf8(json).unwrap();
        assert!(s.contains("section"));
        assert!(s.contains("key"));
    }
}
