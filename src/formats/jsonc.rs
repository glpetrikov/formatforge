use anyhow::Result;
use jsonc_parser::parse_to_serde_value;
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let text = std::str::from_utf8(bytes)?;
    let value = parse_to_serde_value(text, &Default::default())?.unwrap_or(Value::Null);
    Ok(value)
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec_pretty(value)?)
}
