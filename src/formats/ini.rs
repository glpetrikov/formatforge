use anyhow::Result;
use ini::Ini;
use serde_json::{Map, Value};

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let text = std::str::from_utf8(bytes)?;
    let ini = Ini::load_from_str(text)?;
    let mut map = Map::new();
    for (section, properties) in &ini {
        let mut section_map = Map::new();
        for (key, val) in properties.iter() {
            section_map.insert(key.to_string(), Value::String(val.to_string()));
        }
        let key = section.clone().unwrap_or_else(|| "global");
        map.insert(key.to_string(), Value::Object(section_map));
    }
    Ok(Value::Object(map))
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut ini = Ini::new();
    if let Value::Object(map) = value {
        for (section, props) in map {
            if let Value::Object(props_map) = props {
                for (key, val) in props_map {
                    let val_str = match val {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    ini.set_to(Some(section.as_str()), key.clone(), val_str);
                }
            }
        }
    }
    let mut buf = Vec::new();
    ini.write_to(&mut buf)?;
    Ok(buf)
}
