use anyhow::Result;
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let text = std::str::from_utf8(bytes)?;
    let value: Value = json5::from_str(text)?;
    Ok(value)
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    // JSON5 output is just pretty JSON (json5 crate has no serializer)
    Ok(serde_json::to_vec_pretty(value)?)
}
