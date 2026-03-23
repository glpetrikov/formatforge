use anyhow::{Result, bail};
use indexmap::IndexMap;
use serde_json::Value;

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let s = std::str::from_utf8(bytes)?;
    parse_xml(s)
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    write_xml(value, "root", &mut out, 0);
    Ok(out.into_bytes())
}

fn parse_xml(s: &str) -> Result<Value> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(s);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<(String, IndexMap<String, Value>)> = vec![];
    let mut result = Value::Null;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                let mut map = IndexMap::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = format!("@{}", std::str::from_utf8(attr.key.as_ref())?);
                    let val = attr.unescape_value()?.to_string();
                    map.insert(key, Value::String(val));
                }
                stack.push((name, map));
            }
            Ok(Event::End(_)) => {
                if let Some((name, map)) = stack.pop() {
                    // Convert IndexMap to serde_json::Map preserving order
                    let json_map: serde_json::Map<String, Value> = map.into_iter().collect();
                    let val = Value::Object(json_map);
                    if let Some(parent) = stack.last_mut() {
                        let entry = parent.1.entry(name).or_insert(Value::Array(vec![]));
                        if let Value::Array(arr) = entry {
                            arr.push(val);
                        }
                    } else {
                        result = val;
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape()?.to_string();
                if !text.trim().is_empty() {
                    if let Some(parent) = stack.last_mut() {
                        parent.1.insert("#text".to_string(), Value::String(text));
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                let mut map = IndexMap::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = format!("@{}", std::str::from_utf8(attr.key.as_ref())?);
                    let val = attr.unescape_value()?.to_string();
                    map.insert(key, Value::String(val));
                }
                if let Some(parent) = stack.last_mut() {
                    let json_map: serde_json::Map<String, Value> = map.into_iter().collect();
                    let entry = parent.1.entry(name).or_insert(Value::Array(vec![]));
                    if let Value::Array(arr) = entry {
                        arr.push(Value::Object(json_map));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => bail!("XML parse error: {}", e),
            _ => {}
        }
    }

    Ok(simplify(result))
}

fn simplify(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            let ordered: indexmap::IndexMap<String, Value> = map
                .into_iter()
                .map(|(k, v)| {
                    let v = simplify(v);
                    let v = if let Value::Array(mut arr) = v {
                        if arr.len() == 1 {
                            arr.remove(0)
                        } else {
                            Value::Array(arr.into_iter().map(simplify).collect())
                        }
                    } else {
                        v
                    };
                    (k, v)
                })
                .collect();
            Value::Object(ordered.into_iter().collect())
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(simplify).collect()),
        other => other,
    }
}

fn write_xml(value: &Value, tag: &str, out: &mut String, depth: usize) {
    let indent = "  ".repeat(depth);
    match value {
        Value::Object(map) => {
            let attrs: Vec<_> = map.iter().filter(|(k, _)| k.starts_with('@')).collect();
            let children: Vec<_> = map.iter().filter(|(k, _)| !k.starts_with('@')).collect();

            let attr_str = attrs
                .iter()
                .map(|(k, v)| format!(" {}=\"{}\"", &k[1..], v.as_str().unwrap_or("")))
                .collect::<String>();

            if children.is_empty() {
                out.push_str(&format!("{}<{}{}/>\n", indent, tag, attr_str));
            } else {
                out.push_str(&format!("{}<{}{}>\n", indent, tag, attr_str));
                for (k, v) in &children {
                    write_xml(v, k, out, depth + 1);
                }
                out.push_str(&format!("{}</{}>\n", indent, tag));
            }
        }
        Value::Array(arr) => {
            for item in arr {
                write_xml(item, tag, out, depth);
            }
        }
        Value::String(s) => {
            out.push_str(&format!("{}<{}>{}</{}>\n", indent, tag, escape_xml(s), tag));
        }
        Value::Number(n) => {
            out.push_str(&format!("{}<{}>{}</{}>\n", indent, tag, n, tag));
        }
        Value::Bool(b) => {
            out.push_str(&format!("{}<{}>{}</{}>\n", indent, tag, b, tag));
        }
        Value::Null => {
            out.push_str(&format!("{}<{}/>\n", indent, tag));
        }
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
