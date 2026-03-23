use anyhow::Result;
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    Ok(serde_yaml::from_slice(bytes)?)
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    Ok(serde_yaml::to_string(value)?.into_bytes())
}
