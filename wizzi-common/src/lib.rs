#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::panic)]
#![deny(clippy::indexing_slicing)]

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

impl Source {
    pub fn file(path: &str) -> Self {
        Source::File(FileSource {
            path: path.to_string(),
        })
    }

    pub fn serial(port: &str, baud_rate: u32) -> Self {
        Source::Serial(SerialSource {
            port: port.to_string(),
            baud_rate,
        })
    }
}

pub struct FixedSizeQueue<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T> FixedSizeQueue<T> {
    pub fn new(size: usize) -> Self {
        FixedSizeQueue {
            data: Vec::with_capacity(size),
            size,
        }
    }

    pub fn push(&mut self, item: T) {
        while self.data.len() >= self.size {
            self.data.remove(0);
        }
        self.data.push(item);
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }
}
