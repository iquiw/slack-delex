use std::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

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

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        let s = self.text.replace("\n", "\\n");
         match &self.subtype {
            &Some(ref st) => write!(f, "{} [{}] {:.70}", &self.ts, &st, &s),
            &None         => write!(f, "{} {:.70}", &self.ts, &s),
         }
    }
}

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<Vec<Msg>, Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    let msgs: Vec<Msg> = serde_json::from_str(&buf)?;
    Ok(msgs)
}
