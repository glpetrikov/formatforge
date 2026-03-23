use anyhow::Result;
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let value: Value = ciborium::from_reader(bytes)?;
    Ok(value)
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut buf = vec![];
    ciborium::into_writer(value, &mut buf)?;
    Ok(buf)
}
