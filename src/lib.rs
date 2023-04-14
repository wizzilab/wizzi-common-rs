use serde::{Deserialize, Serialize};

pub mod dash7;
pub mod hardware;
pub mod json;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileSource {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SerialSource {
    pub port: String,
    pub baud_rate: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Source {
    File(FileSource),
    Serial(SerialSource),
}
