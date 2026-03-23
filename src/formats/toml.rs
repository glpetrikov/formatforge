use anyhow::{Result, bail};
use serde_json::Value;
use toml_edit::{Array, DocumentMut, Item, Table, value};

pub fn from_bytes(bytes: &[u8]) -> Result<Value> {
    let s = std::str::from_utf8(bytes)?;
    let doc: DocumentMut = s.parse()?;
    Ok(item_to_json(doc.as_item()))
}

pub fn to_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut doc = DocumentMut::new();
    json_to_table(value, doc.as_table_mut())?;
    Ok(doc.to_string().into_bytes())
}

fn item_to_json(item: &Item) -> Value {
    match item {
        Item::Value(v) => toml_value_to_json(v),
        Item::Table(t) => table_to_json(t),
        Item::ArrayOfTables(arr) => Value::Array(arr.iter().map(table_to_json).collect()),
        Item::None => Value::Null,
    }
}

fn toml_value_to_json(v: &toml_edit::Value) -> Value {
    match v {
        toml_edit::Value::String(s) => Value::String(s.value().clone()),
        toml_edit::Value::Integer(i) => Value::Number((*i.value()).into()),
        toml_edit::Value::Float(f) => {
            Value::Number(serde_json::Number::from_f64(*f.value()).unwrap_or(0.into()))
        }
        toml_edit::Value::Boolean(b) => Value::Bool(*b.value()),
        toml_edit::Value::Datetime(dt) => Value::String(dt.value().to_string()),
        toml_edit::Value::Array(arr) => Value::Array(arr.iter().map(toml_value_to_json).collect()),
        toml_edit::Value::InlineTable(t) => Value::Object(
            t.iter()
                .map(|(k, v)| (k.to_string(), toml_value_to_json(v)))
                .collect(),
        ),
    }
}

fn table_to_json(t: &Table) -> Value {
    Value::Object(
        t.iter()
            .map(|(k, v)| (k.to_string(), item_to_json(v)))
            .collect(),
    )
}

fn json_to_table(value: &Value, table: &mut Table) -> Result<()> {
    if let Value::Object(map) = value {
        for (k, v) in map {
            match v {
                Value::Null => bail!("TOML does not support null values"),
                Value::Object(_) => {
                    let mut subtable = Table::new();
                    json_to_table(v, &mut subtable)?;
                    table.insert(k, Item::Table(subtable));
                }
                Value::Array(arr) => {
                    // Check if array of objects -> ArrayOfTables
                    if arr.iter().all(|i| matches!(i, Value::Object(_))) {
                        let mut aot = toml_edit::ArrayOfTables::new();
                        for item in arr {
                            let mut t = Table::new();
                            json_to_table(item, &mut t)?;
                            aot.push(t);
                        }
                        table.insert(k, Item::ArrayOfTables(aot));
                    } else {
                        table.insert(k, Item::Value(json_to_toml_value(v)?));
                    }
                }
                _ => {
                    table.insert(k, Item::Value(json_to_toml_value(v)?));
                }
            }
        }
    }
    Ok(())
}

fn json_to_toml_value(v: &Value) -> Result<toml_edit::Value> {
    Ok(match v {
        Value::Null => bail!("TOML does not support null values"),
        Value::Bool(b) => value(*b).into_value().unwrap(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                value(i).into_value().unwrap()
            } else if let Some(f) = n.as_f64() {
                value(f).into_value().unwrap()
            } else {
                bail!("Invalid number: {}", n)
            }
        }
        Value::String(s) => value(s.as_str()).into_value().unwrap(),
        Value::Array(arr) => {
            let mut toml_arr = Array::new();
            for item in arr {
                toml_arr.push(json_to_toml_value(item)?);
            }
            toml_edit::Value::Array(toml_arr)
        }
        Value::Object(map) => {
            let mut inline = toml_edit::InlineTable::new();
            for (k, val) in map {
                inline.insert(k, json_to_toml_value(val)?);
            }
            toml_edit::Value::InlineTable(inline)
        }
    })
}
