use anyhow::Result;
use serde_json::{Map, Value};

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let s = std::str::from_utf8(bytes)?;
    let mut map = Map::new();

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim().to_string();
            let val = val.trim().trim_matches('"').trim_matches('\'').to_string();
            map.insert(key, Value::String(val));
        }
    }

    Ok(Value::Object(map))
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut out = String::new();
    write_flat("", value, &mut out);
    Ok(out.into_bytes())
}

fn write_flat(prefix: &str, v: &Value, out: &mut String) {
    match v {
        Value::Object(map) => {
            for (k, v) in map {
                let key = if prefix.is_empty() {
                    k.to_uppercase()
                } else {
                    format!("{}_{}", prefix, k.to_uppercase())
                };
                write_flat(&key, v, out);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let key = format!("{}_{}", prefix, i);
                write_flat(&key, v, out);
            }
        }
        Value::String(s) => {
            if s.contains(' ') || s.contains('"') || s.contains('\'') {
                out.push_str(&format!("{}=\"{}\"\n", prefix, s.replace('"', "\\\"")));
            } else {
                out.push_str(&format!("{}={}\n", prefix, s));
            }
        }
        Value::Number(n) => out.push_str(&format!("{}={}\n", prefix, n)),
        Value::Bool(b) => out.push_str(&format!("{}={}\n", prefix, b)),
        Value::Null => out.push_str(&format!("{}=\n", prefix)),
    }
}
