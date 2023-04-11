use serde::{Deserialize, Serialize};

pub mod dash7;
pub mod hardware;
pub mod json;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SourceConf {
    File { path: String },
    Serial { path: String, baud: u32 },
}
