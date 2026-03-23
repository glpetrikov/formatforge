use anyhow::Result;
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let value: Value = rmp_serde::from_slice(bytes)?;
    Ok(value)
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    Ok(rmp_serde::to_vec(value)?)
}
