use std::io::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

use serde_json;
use serde_json::Value;

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<Vec<Value>, Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    let msgs: Vec<Value> = serde_json::from_str(&buf)?;
    Ok(msgs)
}

pub fn msg_ts(v: &Value) -> Option<String> {
    match v {
        &Value::Object(ref map) => {
            match map.get("ts")? {
                &Value::String(ref ts) => Some(ts.to_owned()),
                _ => None
            }
        },
        _ => None
    }
}
