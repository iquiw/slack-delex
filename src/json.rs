use std::io::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Msg {
    ts: String,
    text: String,
    subtype: Option<String>,
}

impl Msg {
    pub fn ts(&self) -> &str {
        &self.ts
    }
}

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<Vec<Msg>, Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    let msgs: Vec<Msg> = serde_json::from_str(&buf)?;
    Ok(msgs)
}
