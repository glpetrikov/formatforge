use anyhow::Result;
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde_json::{Map, Value};

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let s = std::str::from_utf8(bytes)?;
    let doc: KdlDocument = s.parse()?;
    Ok(doc_to_json(&doc))
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let doc = json_to_doc(value);
    Ok(doc.to_string().into_bytes())
}

fn doc_to_json(doc: &KdlDocument) -> Value {
    let mut map = Map::new();
    for node in doc.nodes() {
        let name = node.name().value().to_string();
        let val = node_to_json(node);
        // If key already exists, make it an array
        let entry = map.entry(name).or_insert(Value::Array(vec![]));
        if let Value::Array(arr) = entry {
            arr.push(val);
        }
    }
    // Unwrap single-element arrays
    Value::Object(
        map.into_iter()
            .map(|(k, v)| {
                let v = if let Value::Array(mut arr) = v {
                    if arr.len() == 1 {
                        arr.remove(0)
                    } else {
                        Value::Array(arr)
                    }
                } else {
                    v
                };
                (k, v)
            })
            .collect(),
    )
}

fn node_to_json(node: &KdlNode) -> Value {
    let entries: Vec<&KdlEntry> = node.entries().iter().collect();
    let children = node.children();

    // If there's only one positional arg and no children/props, return scalar
    let args: Vec<&KdlEntry> = entries
        .iter()
        .filter(|e| e.name().is_none())
        .copied()
        .collect();
    let props: Vec<&KdlEntry> = entries
        .iter()
        .filter(|e| e.name().is_some())
        .copied()
        .collect();

    if children.is_none() && props.is_empty() && args.len() == 1 {
        return kdl_value_to_json(args[0].value());
    }

    let mut map = Map::new();

    if !args.is_empty() {
        let vals: Vec<Value> = args.iter().map(|e| kdl_value_to_json(e.value())).collect();
        if vals.len() == 1 {
            map.insert("value".to_string(), vals.into_iter().next().unwrap());
        } else {
            map.insert("values".to_string(), Value::Array(vals));
        }
    }

    for prop in props {
        let k = prop.name().unwrap().value().to_string();
        map.insert(k, kdl_value_to_json(prop.value()));
    }

    if let Some(doc) = children {
        let child_val = doc_to_json(doc);
        if let Value::Object(child_map) = child_val {
            for (k, v) in child_map {
                map.insert(k, v);
            }
        }
    }

    Value::Object(map)
}

fn kdl_value_to_json(v: &KdlValue) -> Value {
    match v {
        KdlValue::String(s) => Value::String(s.clone()),
        KdlValue::Integer(i) => Value::Number((*i as i64).into()),
        KdlValue::Float(f) => Value::Number(serde_json::Number::from_f64(*f).unwrap_or(0.into())),
        KdlValue::Bool(b) => Value::Bool(*b),
        KdlValue::Null => Value::Null,
    }
}

fn json_to_doc(value: &Value) -> KdlDocument {
    let mut doc = KdlDocument::new();
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                doc.nodes_mut().push(json_to_node(k, v));
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                doc.nodes_mut().push(json_to_node(&i.to_string(), v));
            }
        }
        other => {
            doc.nodes_mut().push(json_to_node("value", other));
        }
    }
    doc
}

fn json_to_node(name: &str, value: &Value) -> KdlNode {
    let mut node = KdlNode::new(name);
    match value {
        Value::Object(map) => {
            let mut child_doc = KdlDocument::new();
            for (k, v) in map {
                child_doc.nodes_mut().push(json_to_node(k, v));
            }
            node.set_children(child_doc);
        }
        Value::Array(arr) => {
            let mut child_doc = KdlDocument::new();
            for (i, v) in arr.iter().enumerate() {
                child_doc.nodes_mut().push(json_to_node(&i.to_string(), v));
            }
            node.set_children(child_doc);
        }
        Value::String(s) => {
            node.push(KdlEntry::new(KdlValue::String(s.clone())));
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                node.push(KdlEntry::new(KdlValue::Integer(i.into())));
            } else if let Some(f) = n.as_f64() {
                node.push(KdlEntry::new(KdlValue::Float(f)));
            }
        }
        Value::Bool(b) => {
            node.push(KdlEntry::new(KdlValue::Bool(*b)));
        }
        Value::Null => {
            node.push(KdlEntry::new(KdlValue::Null));
        }
    }
    node
}
