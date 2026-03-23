use anyhow::{bail, Result};
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let s = std::str::from_utf8(bytes)?;
    let v: toml::Value = toml::from_str(s)?;
    Ok(toml_to_json(v))
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let toml_val = json_to_toml(value)?;
    Ok(toml::to_string_pretty(&toml_val)?.into_bytes())
}

fn toml_to_json(v: toml::Value) -> Value {
    match v {
        toml::Value::String(s) => Value::String(s),
        toml::Value::Integer(i) => Value::Number(i.into()),
        toml::Value::Float(f) => {
            Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into()))
        }
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Array(arr) => Value::Array(arr.into_iter().map(toml_to_json).collect()),
        toml::Value::Table(map) => {
            Value::Object(map.into_iter().map(|(k, v)| (k, toml_to_json(v))).collect())
        }
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
    }
}

fn json_to_toml(v: &Value) -> Result<toml::Value> {
    Ok(match v {
        Value::Null => bail!("TOML does not support null values"),
        Value::Bool(b) => toml::Value::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                toml::Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                toml::Value::Float(f)
            } else {
                bail!("Invalid number: {}", n)
            }
        }
        Value::String(s) => toml::Value::String(s.clone()),
        Value::Array(arr) => {
            toml::Value::Array(arr.iter().map(json_to_toml).collect::<Result<_>>()?)
        }
        Value::Object(map) => toml::Value::Table(
            map.iter()
                .map(|(k, v)| Ok((k.clone(), json_to_toml(v)?)))
                .collect::<Result<_>>()?,
        ),
    })
}
