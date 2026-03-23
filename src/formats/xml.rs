use anyhow::{bail, Result};
use serde_json::{Map, Value};

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
    // Simple XML -> JSON: use quick-xml events
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(s);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<(String, Map<String, Value>)> = vec![];
    let mut result = Value::Null;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                stack.push((name, Map::new()));
            }
            Ok(Event::End(_)) => {
                if let Some((name, map)) = stack.pop() {
                    let val = if map.is_empty() {
                        Value::Object(Map::new())
                    } else {
                        Value::Object(map)
                    };
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
                if let Some(parent) = stack.last_mut() {
                    let entry = parent.1.entry(name).or_insert(Value::Array(vec![]));
                    if let Value::Array(arr) = entry {
                        arr.push(Value::Object(Map::new()));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => bail!("XML parse error: {}", e),
            _ => {}
        }
    }

    // Unwrap single-element arrays for cleaner output
    Ok(simplify(result))
}

fn simplify(v: Value) -> Value {
    match v {
        Value::Object(map) => Value::Object(
            map.into_iter()
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
                .collect(),
        ),
        Value::Array(arr) => Value::Array(arr.into_iter().map(simplify).collect()),
        other => other,
    }
}

fn write_xml(value: &Value, tag: &str, out: &mut String, depth: usize) {
    let indent = "  ".repeat(depth);
    match value {
        Value::Object(map) => {
            out.push_str(&format!("{}<{}>\n", indent, tag));
            for (k, v) in map {
                write_xml(v, k, out, depth + 1);
            }
            out.push_str(&format!("{}</{}>\n", indent, tag));
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
